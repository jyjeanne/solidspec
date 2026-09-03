# Plan d'intégration `okf-rs`

Suite à [`docs/okf-rs-integration-study.md`](okf-rs-integration-study.md). Ce document détaille les étapes,
dans l'ordre, pour intégrer `okf-rs` — de la simple substitution d'outil interne jusqu'à une fonctionnalité
produit exploitée par les workflows SDD que SolidSpec scaffold.

## Étape 1 — Substituer `graphify` (fait ✅)

**Commit `c5e2193`.** Remplace le graphe de connaissances interne (`docs/graph/`, utilisé par les
contributeurs et par Claude Code lui-même via CLAUDE.md) : `graphify` (Python/`uv`) → `okf-rs` (Rust).

- `scripts/generate-graph.sh` appelle `okf-rs generate` + `okf-rs validate --ci`, écrit un
  `GRAPH_REPORT.md` à partir de `graph stats`/`coverage`.
- `docs/graph/knowledge/` (bundle OKF, 1258 fichiers Markdown+YAML) remplace `graph.json`/`graph.html`/
  `manifest.json`/`.graphify_analysis.json`.
- CLAUDE.md documente les commandes de requête (`search`, `explore`, `graph callers/path/stats`, `impact`).
- Aucun changement de comportement produit — outillage contributeur uniquement.

Reste à faire côté étape 1 (mineur, non bloquant) :
- Vérifier en CI que `./scripts/generate-graph.sh` s'exécute sans `okf-rs` préinstallé échoue proprement
  (déjà le cas : message d'erreur explicite avec la commande d'installation).
- Envisager un job CI optionnel `okf-rs generate --check-fresh` pour détecter un bundle périmé avant merge
  (parité avec l'intention documentée dans l'étude, non implémenté ici pour rester dans le périmètre
  "outillage contributeur, aucun changement produit").

## Étape 2 — Extension `okf` optionnelle (scaffolding, pas de branchement pipeline) — fait ✅

**Commit à venir.** Objectif : permettre à un projet **scaffoldé par SolidSpec** (pas SolidSpec lui-même)
d'avoir son propre bundle OKF, sans toucher au comportement par défaut de `solidspec init`.

Écart par rapport à la formulation initiale du plan : implémenté comme **extension** (`src/extensions/`),
pas comme **preset** (`src/presets/`). Après lecture du code des deux systèmes, le système de presets sert
exclusivement à *surcharger les templates d'artefacts SDD* (`spec-template.md`, etc. via `provides.templates`
+ résolution par priorité) — pas à scaffolder des fichiers de config arbitraires ni à réagir à un événement
de cycle de vie. Le système d'extensions, lui, a exactement ce qu'il faut : `provides.commands` (fichiers
associés à un id de commande) + `hooks` (déclenchés à des points de cycle de vie précis, dont `after_init`,
via `src/extensions/hooks.rs::fire_hooks`, exécuté en sous-processus `sh`). Aucun des deux systèmes n'a de
catalogue intégré au binaire — `solidspec preset add`/`extension add` exigent un répertoire source local
avec un manifest — donc l'extension est livrée comme code source dans le repo (`extensions/okf/`), à
installer avec `solidspec extension add extensions/okf --dev`.

Contenu livré (`extensions/okf/`) :
- `extension.yml` — manifest déclarant une commande `solidspec.okf.init` et un hook `after_init` (marqué
  `optional: true`).
- `hooks/after-init.sh` — script best-effort : si `okf-rs` est absent du `PATH`, affiche l'instruction
  d'installation et sort en succès (n'échoue jamais `solidspec init`) ; sinon lance
  `okf-rs init --output .solidspec/knowledge --no-agent-files` (le `--no-agent-files` évite qu'okf-rs
  touche CLAUDE.md/AGENTS.md, déjà gérés par `src/agents/registry.rs`) et ajoute `.okf-cache.json` au
  `.gitignore` du projet cible s'il existe.
- `README.md` — usage, y compris la limite connue : le hook `after_init` ne peut pas se déclencher sur le
  tout premier `solidspec init` qui crée le projet (`extension add` exige que `.solidspec/` existe déjà) ;
  il faut soit réexécuter `solidspec init` après avoir ajouté l'extension (idempotent, vérifié), soit lancer
  les deux commandes du hook à la main.
- Pas de MCP à ce stade — juste le scaffolding. Pas de dépendance Rust ajoutée à `Cargo.toml` : `okf-rs`
  reste détecté via `command -v` dans le script, jamais un binaire requis.

**Tests** (`tests/okf_extension.rs`, passent) :
- `okf_extension_installs_and_registers_hook` — `extension add --dev` réussit, `extension info` montre le
  hook enregistré.
- `okf_extension_hook_never_fails_init_regardless_of_okf_rs_availability` — un second `init` (qui déclenche
  le hook) réussit que `okf-rs` soit sur le `PATH` ou non ; si présent, vérifie le contenu de `okf.toml`
  généré.

Vérifié manuellement en plus des tests automatisés : `cargo build`, `cargo clippy --all-targets -- -D
warnings`, `cargo fmt --check` propres ; installation réelle + double `init` dans un projet temporaire avec
`okf-rs` réellement installé, et avec `PATH` restreint pour simuler son absence.

## Étape 3 — Registration MCP pour les agents qui le supportent

- Dans `src/agents/registry.rs`, à côté de l'écriture des fichiers de commande slash par agent, ajouter
  (seulement quand le preset `okf` est actif) une entrée MCP pour les agents dont `AGENTS` (`config.rs`)
  déclare le support MCP — probablement un nouveau champ `mcp_config_path`/`supports_mcp` dans la table
  statique existante.
- Contenu enregistré : pointer vers `okf-mcp` avec le bundle du projet (`.solidspec/knowledge/`), pas vers
  SolidSpec lui-même.
- Rester conservateur : ne rien enregistrer si `okf-rs`/`okf-mcp` n'est pas détecté sur le système au moment
  de `init` — l'agent découvrira l'absence à l'exécution plutôt que SolidSpec ne devine.

## Étape 4 — Hook pipeline avant `plan` (génération, jamais bloquant)

- `src/core/pipeline.rs` : une étape *facultative*, activée seulement si le preset `okf` est présent dans le
  projet (présence de `okf.toml`), qui lance `okf-rs generate --no-cache=false` avant la phase `plan`.
  - Best-effort : un échec (binaire absent, erreur de parsing) logge un avertissement et n'interrompt jamais
    le pipeline — cohérent avec le fait que `plan` doit rester utilisable sans `okf-rs`.
  - Le prompt de la commande `plan` (`templates/commands/plan.md`) mentionne, seulement si le bundle existe,
    qu'un graphe d'appels structurel est disponible via MCP/`okf-rs explore` pour éviter de re-lire des
    fichiers à froid.
- Pas de nouvelle dépendance obligatoire dans `Cargo.toml` : `okf-rs` reste un binaire externe détecté via
  `which`, invoqué en sous-processus (`std::process::Command`), exactement comme les CLIs d'agents dans
  `src/agents/invoker.rs`.

## Étape 5 — Vérification structurelle dans `analyze`

- `src/core/analyzer.rs` fait aujourd'hui des vérifications textuelles (traçabilité FR-###, cohérence entre
  artefacts). Ajouter, seulement si un bundle `okf-rs` existe, une vérification complémentaire :
  - pour chaque tâche de `tasks.md` référençant un fichier/symbole cible, confirmer via
    `okf-rs search`/`explore` (invoqué en sous-processus, sortie parsée) que le symbole existe réellement
    dans le graphe — détecte les références orphelines qu'une simple recherche textuelle raterait.
  - Ce nouveau contrôle apparaît comme une section distincte dans `analysis-report.md`
    ("Structural cross-check (okf-rs)"), jamais mélangée aux heuristiques existantes — même principe
    d'indépendance que documenté pour `ai-spec-review-skill` vs `review.rs` dans CLAUDE.md.
  - Absence du bundle → section simplement omise, aucun échec.

## Étape 6 — `review`/ship gate : rapport d'impact

- `src/core/review.rs` / le ship gate 4-lanes : ajouter une lane optionnelle qui exécute
  `okf-rs review <base-ref> HEAD` (rendu Markdown prêt pour un commentaire de PR) et l'attache à
  `review-report.md` sous une section dédiée.
- Particulièrement utile pour le schéma `security-first` : le blast-radius structurel (`okf-rs impact`)
  donne un signal concret ("cette fonction touchant l'auth a-t-elle des appelants externes inattendus")
  au review de sécurité, en complément — jamais en remplacement — de l'audit OWASP textuel existant.

## Ordre de priorité recommandé

1. ✅ Étape 1 (faite)
2. Étape 2 — scaffolding seul, risque quasi nul, valeur immédiate même sans les étapes suivantes
3. Étape 4 — le hook `plan` est probablement le gain le plus direct pour la qualité des specs/plans générés
4. Étape 5 — renforce `analyze`, dépend du même mécanisme d'invocation que l'étape 4
5. Étape 3 — MCP, une fois qu'on sait quels agents dans `AGENTS` le supportent réellement
6. Étape 6 — la plus périphérique (ship gate), à faire en dernier

Chaque étape à partir de 2 mérite son propre cycle `spec-driven` (spec → plan → tasks → tests →
implement → analyze → review) plutôt qu'un développement ad hoc — conformément à la philosophie du projet.
Aucune étape 2+ n'a été implémentée dans le présent travail ; seule l'étape 1 (substitution d'outillage) a
été commise.

## Risques transverses à garder en tête

- `okf-rs` n'est pas publié sur crates.io ; toute automatisation (CI, preset) doit épingler un tag/rev git
  précis plutôt que `main`.
- Ne jamais rendre `okf-rs` obligatoire pour un chemin critique (`init`, `plan`, `analyze`, `review`) —
  toujours détection + dégradation silencieuse, pour ne pas casser le schéma `minimal` ni les environnements
  CI qui n'ont pas le binaire.
- Garder la séparation stricte entre les heuristiques déterministes existantes (`analyzer.rs`, `review.rs`)
  et tout apport `okf-rs` : additif et clairement labellisé, jamais fusionné.
