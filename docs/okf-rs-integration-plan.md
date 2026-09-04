# Plan d'intégration `okf-rs`

Suite à [`docs/okf-rs-integration-study.md`](okf-rs-integration-study.md). Ce document détaille les étapes,
dans l'ordre, pour intégrer `okf-rs` — de la simple substitution d'outil interne jusqu'à une fonctionnalité
produit exploitée par les workflows SDD que SolidSpec scaffold.

## Mise à jour — passage à l'intégration native (Option B)

Choix explicite : plutôt que de dépendre d'un binaire `okf-rs` externe (installé séparément, détecté via
`which`/`command -v`), SolidSpec **vendorise** les crates bibliothèque d'okf-rs (`okf-core`, `okf-analyzer`,
`okf-generator`, `okf-validator`, `okf-parser`) comme dépendances git épinglées (`tag = "v0.7.0"`) dans
`Cargo.toml`, et les appelle en process via un nouveau module `src/core/okf.rs` + une nouvelle commande
`solidspec okf generate`/`solidspec okf validate`. Zéro binaire externe pour ces deux opérations : ni
détection PATH, ni sous-processus, ni installation séparée pour l'utilisateur.

Vérifié avant d'adopter cette approche (voir historique de session pour le détail) :
- L'API exposée par ces crates est propre et composable (`Project::load` → `analyze_with_cache_lsp` →
  `write_bundle`), directement extraite de l'implémentation réelle d'`okf-cli`'s `cmd_generate` — pas de
  réécriture de logique, juste réutilisation.
- Coût mesuré : ~1 min de compilation supplémentaire à froid pour ce sous-ensemble de crates (tree-sitter ×11
  langages, pas de tantivy/PDF/LSP-actif/DITA/MCP/enrich — ces derniers restent hors du périmètre vendorisé),
  binaire `solidspec` release passant d'environ 8,6 Mo à un ordre de grandeur plus proche de 25-30 Mo.
  Acceptable pour un CLI installé une fois, pas pour un outil qu'on retélécharge à chaque run.
- Portée volontairement limitée à `generate`/`validate` : ce sont les deux seules opérations dont les crates
  sous-jacentes (`okf-analyzer`/`okf-generator`/`okf-validator`) n'entraînent pas tantivy (recherche),
  un client LSP actif, un rendu PDF, ou un client HTTP OpenAI-compatible. `search`/`explore`/`graph`/`diff`/
  `impact`/`review`/le serveur `okf-mcp` restent donc, pour l'instant, derrière le CLI externe — vendoriser
  ce sous-ensemble reste une extension possible du même principe, pas un changement d'approche.

Conséquence sur les étapes ci-dessous : partout où une étape mentionnait "shell out vers `okf-rs`", lire
désormais "appel natif à `src/core/okf::generate`/`validate`" quand l'opération concernée est l'une de ces
deux-là ; le reste (recherche, exploration, impact) attend encore soit un vendoring futur du même style, soit
l'appel au CLI externe / `okf-mcp` comme décrit précédemment.

## Étape 1 — Substituer `graphify` (fait ✅, mis à niveau vers l'intégration native)

**Commits `c5e2193` (swap initial vers le CLI externe) puis mise à jour native.** Remplace le graphe de
connaissances interne (`docs/graph/`, utilisé par les contributeurs et par Claude Code lui-même via
CLAUDE.md) : `graphify` (Python/`uv`) → d'abord `okf-rs` CLI externe, puis `solidspec okf generate`/`validate`
natifs (voir mise à jour ci-dessus).

- `scripts/generate-graph.sh` compile `solidspec` (`cargo build --quiet`) puis appelle
  `solidspec okf generate` + `solidspec okf validate --ci` — plus aucun outil à installer séparément pour
  régénérer `docs/graph/`. `GRAPH_REPORT.md` reprend directement la sortie de `generate` (répartition par
  type de concept) ; les sections "topology"/"coverage" plus riches de l'ancien `okf-rs graph stats`/
  `coverage` ne sont pas reproduites (pas vendorisées — voir ci-dessus), et restent accessibles via le CLI
  externe si besoin ponctuel.
- `docs/graph/knowledge/` (bundle OKF, ~1275 fichiers Markdown+YAML) remplace `graph.json`/`graph.html`/
  `manifest.json`/`.graphify_analysis.json`.
- CLAUDE.md documente à la fois la régénération native (`./scripts/generate-graph.sh`, zéro dépendance) et
  les commandes de requête qui nécessitent encore le CLI externe (`search`, `explore`, `graph`, `impact`).
- Aucun changement de comportement produit — outillage contributeur uniquement.

## Étape 2 — Extension `okf` optionnelle (scaffolding, pas de branchement pipeline) — fait ✅

**Commits `7ccbfc7` (v0.1.0, détection du binaire externe) puis mise à niveau v0.2.0 (intégration native,
Option B choisie explicitement).** Objectif : permettre à un projet **scaffoldé par SolidSpec** (pas
SolidSpec lui-même) d'avoir son propre bundle OKF, sans toucher au comportement par défaut de
`solidspec init`.

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

**v0.1.0 → v0.2.0 :** la première version détectait un binaire `okf-rs` externe (`command -v okf-rs`) et
lançait `okf-rs init`. Suite à un choix explicite d'éviter toute dépendance à un binaire externe (Option B
de l'étude), le hook a été réécrit pour appeler `solidspec okf generate` — la commande native ajoutée à
l'étape 1 — au lieu de shell-out vers un outil séparé. Nouvelle limite en échange : le hook a besoin que
`solidspec` lui-même soit résoluble sur le `PATH` du sous-shell qui l'exécute (vrai pour toute installation
normale — `cargo install`/binaire prébuilt ; pas garanti pour un `cargo run`/binaire de dev non installé,
cas couvert par le test `okf_extension_hook_never_fails_init_when_solidspec_is_not_on_path`).

Contenu livré (`extensions/okf/`) :
- `extension.yml` — manifest déclarant une commande `solidspec.okf.init` et un hook `after_init` (marqué
  `optional: true`). Plus de `requires.tools` (aucun outil externe requis).
- `hooks/after-init.sh` — script best-effort : si `solidspec` est absent du `PATH`, affiche un message et
  sort en succès (n'échoue jamais `solidspec init`) ; sinon lance
  `solidspec okf generate . --output .solidspec/knowledge` et ajoute `.okf-cache.json` au `.gitignore` du
  projet cible s'il existe.
- `README.md` — usage, y compris la limite connue : le hook `after_init` ne peut pas se déclencher sur le
  tout premier `solidspec init` qui crée le projet (`extension add` exige que `.solidspec/` existe déjà) ;
  il faut soit réexécuter `solidspec init` après avoir ajouté l'extension (idempotent, vérifié), soit lancer
  la commande du hook à la main.
- Pas de MCP à ce stade — juste le scaffolding.

**Tests** (`tests/okf_extension.rs`, passent) :
- `okf_extension_installs_and_registers_hook` — `extension add --dev` réussit, `extension info` montre le
  hook enregistré.
- `okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path` — un second `init` (qui déclenche
  le hook) avec le répertoire du binaire `solidspec` sous test ajouté au `PATH` produit un vrai bundle
  (`.solidspec/knowledge/index.md` existe).
- `okf_extension_hook_never_fails_init_when_solidspec_is_not_on_path` — le même second `init` avec un `PATH`
  ne contenant pas `solidspec` réussit quand même, sans générer de bundle.

Vérifié manuellement en plus des tests automatisés : `cargo build`, `cargo clippy --all-targets -- -D
warnings`, `cargo fmt --check`, et `cargo test` (551 tests) propres ; `solidspec okf generate`/`validate`
exécutés réellement sur le dépôt SolidSpec lui-même.

## Étape 3 — Registration MCP pour les agents qui le supportent

- Dans `src/agents/registry.rs`, à côté de l'écriture des fichiers de commande slash par agent, ajouter
  (seulement quand le preset `okf` est actif) une entrée MCP pour les agents dont `AGENTS` (`config.rs`)
  déclare le support MCP — probablement un nouveau champ `mcp_config_path`/`supports_mcp` dans la table
  statique existante.
- Contenu enregistré : pointer vers `okf-mcp` avec le bundle du projet (`.solidspec/knowledge/`), pas vers
  SolidSpec lui-même.
- Rester conservateur : ne rien enregistrer si `okf-rs`/`okf-mcp` n'est pas détecté sur le système au moment
  de `init` — l'agent découvrira l'absence à l'exécution plutôt que SolidSpec ne devine.

## Étape 4 — Boucle de régénération après `implement` — fait ✅ (reformulée)

**Reformulée par rapport au texte initial** (qui prévoyait un hook *avant* `plan`, conditionné à un preset
`okf.toml` et à un binaire `okf-rs` externe) : après l'étape 5, il est apparu plus utile de régénérer
*après* `implement` plutôt qu'avant `plan` — c'est exactement le moment où le code vient de changer, et
c'est ce qui rend la vérification structurelle de l'étape 5 fiable dans la durée plutôt que basée sur un
graphe figé à `init`. Voir la boucle de rétroaction (recommandation #2 de
`docs/kg-workflow-vision-gap-analysis.md`).

Implémenté nativement, dans le même style que l'étape 5 :
- `src/core/okf.rs::refresh_if_present(project_root)` régénère le bundle à
  `project_root.join(DEFAULT_BUNDLE_DIR)` **seulement s'il existe déjà** — ne crée jamais de bundle pour un
  projet qui n'a pas opté (cette décision reste celle de `solidspec init` / de la commande manuelle `solidspec
  okf generate`). Retourne `None` sans effet quand il n'y a rien à rafraîchir.
- `src/cli/pipeline.rs::refresh_knowledge_graph` appelle cette fonction depuis la branche `"implement"` de
  `execute_phase`, juste après la confirmation utilisateur (`--auto` ou "Press Enter") — c'est le seul point
  du pipeline où le code vient effectivement d'être modifié par l'agent IA. Best-effort : une erreur affiche
  un avertissement et n'interrompt jamais le pipeline.
- La commande standalone `solidspec implement` (hors pipeline) n'a **pas** reçu ce hook : elle imprime les
  tâches en attente puis retourne immédiatement, avant que l'agent IA n'ait rien modifié — il n'y a pas de
  point de confirmation "le code vient de changer" dans ce process-là.
- Pas de nouvelle dépendance : aucun binaire externe, aucun preset requis — juste une fonction du module déjà
  vendorisé (`core::okf`).

## Étape 5 — Vérification structurelle dans `analyze` — fait ✅

**Implémenté nativement, sans `okf-rs search`/`explore`** (ni même `okf-mcp`) : la formulation initiale de
cette étape supposait shell-out vers le CLI externe, mais `okf-parser` (déjà vendorisé) expose
`read_bundle(bundle_dir) -> Vec<Concept>`, qui relit le bundle déjà généré (ses fichiers Markdown+YAML) sans
ré-analyser le code — exactement la seule requête dont ce contrôle a besoin (appartenance, pas recherche
classée). `src/core/okf.rs::BundleIndex` enveloppe ça en deux ensembles (`files`, `symbols`) avec `has_file`/
`has_symbol`.

`src/core/analyzer.rs::structural_cross_check` (appelé depuis `analyze_feature`, seulement si `tasks.md`
existe) détecte deux cas :
- un symbole entre backticks dans `tasks.md` (`` `CombatSystem.calculate_damage()` ``) qui ne correspond à
  aucun nom de concept dans le graphe (sévérité Medium) ;
- un fichier existant sur disque, d'une extension reconnue par `okf_parser::Language::from_extension`,
  référencé dans `tasks.md`, mais absent du graphe — typiquement un bundle devenu obsolète depuis (sévérité
  Low). Un fichier qui n'existe pas encore (le livrable de la tâche) n'est jamais signalé.

Le contrôle apparaît comme une section distincte dans `analysis-report.md` ("## Structural cross-check
(okf-rs)"), jamais mélangée aux heuristiques textuelles existantes — même principe d'indépendance que
documenté pour `ai-spec-review-skill` vs `review.rs` dans CLAUDE.md. Absence de bundle
(`.solidspec/knowledge/` introuvable) → section simplement omise (`AnalysisReport.structural_cross_check =
None`), aucun échec de `analyze`.

Recoupe la recommandation #1 de `docs/kg-workflow-vision-gap-analysis.md`.

## Étape 6 — `review`/ship gate : rapport d'impact

- `src/core/review.rs` / le ship gate 4-lanes : ajouter une lane optionnelle qui exécute
  `okf-rs review <base-ref> HEAD` (rendu Markdown prêt pour un commentaire de PR) et l'attache à
  `review-report.md` sous une section dédiée.
- Particulièrement utile pour le schéma `security-first` : le blast-radius structurel (`okf-rs impact`)
  donne un signal concret ("cette fonction touchant l'auth a-t-elle des appelants externes inattendus")
  au review de sécurité, en complément — jamais en remplacement — de l'audit OWASP textuel existant.

## Ordre de priorité recommandé

1. ✅ Étape 1 (faite)
2. ✅ Étape 2 (faite)
3. ✅ Étape 5 (faite) — n'a finalement pas eu besoin d'attendre l'étape 4 : `okf_parser::read_bundle`
   suffisait, aucun mécanisme d'invocation partagé avec un futur hook `plan` n'était nécessaire.
4. ✅ Étape 4 (faite, reformulée — après `implement` plutôt qu'avant `plan`) — ferme la boucle de
   rétroaction : la vérification structurelle de l'étape 5 s'appuie désormais sur un graphe qui se
   rafraîchit tout seul après chaque implémentation, au lieu de rester figé depuis `init`.
5. Étape 3 — MCP natif, une fois qu'on sait quels agents dans `AGENTS` le supportent réellement (l'entrée
   `.mcp.json` écrite par `solidspec init` aujourd'hui pointe vers un `okf-mcp` externe non vendorisé —
   voir `docs/kg-workflow-vision-gap-analysis.md` §1)
6. Étape 6 — la plus périphérique (ship gate), à faire en dernier

Chaque étape à partir de 2 mérite son propre cycle `spec-driven` (spec → plan → tasks → tests →
implement → analyze → review) plutôt qu'un développement ad hoc — conformément à la philosophie du projet.
Aucune étape 2+ n'a été implémentée dans le présent travail ; seule l'étape 1 (substitution d'outillage) a
été commise.

## Risques transverses à garder en tête

- `okf-rs` n'est pas publié sur crates.io ; les dépendances git dans `Cargo.toml` sont épinglées par
  `tag = "v0.7.0"`, jamais une branche — un bump de version est un choix délibéré (édition manuelle du tag),
  pas une dérive silencieuse. `Cargo.lock` fige de toute façon le commit résolu exact entre deux bumps.
  Pour les capacités encore derrière le CLI externe (recherche/exploration/impact — voir la mise à jour en
  tête de ce document), même logique de prudence si un preset/étape future automatise son installation.
- Ne jamais rendre l'appel à `src/core/okf`/le CLI externe obligatoire pour un chemin critique (`init`,
  `plan`, `analyze`, `review`) — toujours détection + dégradation silencieuse (best-effort), pour ne pas
  casser le schéma `minimal` ni les environnements où le bundle n'existe pas.
- Garder la séparation stricte entre les heuristiques déterministes existantes (`analyzer.rs`, `review.rs`)
  et tout apport `okf` : additif et clairement labellisé, jamais fusionné.
- Le vendoring augmente le temps de compilation et la taille du binaire `solidspec` (voir la mise à jour en
  tête de ce document pour les chiffres mesurés) — une considération à garder à l'esprit avant de vendoriser
  d'autres crates okf-rs (recherche/graphe complet) pour les étapes 3+.
