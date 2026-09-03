# Revue : code existant vs. vision « moteur de contexte + orchestration »

Comparaison du code réellement présent dans ce dépôt (branche
`claude/okf-rs-integration-study-7zmguq`, commit `4d1258b`) avec la vision décrite dans la
discussion : séparation stricte **Knowledge Graph** (« qu'est-ce qui est vrai ? ») / **DAG de
workflow** (« que faire ensuite ? »), connectées via **MCP**, avec **evidence/provenance
typée** (fact ≠ décision ≠ hypothèse) et une **boucle de mise à jour** code → graphe → workflow.

Méthodologie : chaque affirmation ci-dessous a été vérifiée en lisant le code source
(`src/core/okf.rs`, `src/core/artifact_graph.rs`, `src/core/schema.rs`, `src/cli/pipeline.rs`,
`schemas/*/schema.yaml`, `docs/okf-rs-integration-plan.md`, un fichier réel du bundle
`docs/graph/knowledge/`), pas déduite de la documentation seule.

## Verdict en une phrase

**La séparation KG / DAG existe déjà structurellement dans le code** (deux modules distincts,
`core::okf` vs `core::artifact_graph`+`core::schema`+`core::pipeline`), mais **les deux ne
communiquent pas** : le graphe est généré une fois puis oublié, aucune étape du DAG ne
l'interroge, et la notion d'*evidence/provenance typée* (fait vérifiable vs décision vs
hypothèse) qui est au cœur de la réduction d'hallucinations n'existe pas du tout — le graphe ne
connaît que des faits structurels exacts (tree-sitter), sans lien vers spec/plan/décisions/git
blame.

> **Mise à jour** (postérieure à `4d1258b`) : « aucune étape du DAG ne l'interroge » n'est plus
> vrai pour `analyze` — voir §8/recommandation #1, implémenté depuis. Le reste du verdict
> (pas de boucle de rétroaction, pas d'evidence typée) tient toujours.

---

## 1. Knowledge Graph (OKF) — ce qui existe

| Élément de la vision | État | Détail |
|---|---|---|
| Indexeur AST → concepts | ✅ Fait | `src/core/okf.rs::generate` — vendorise `okf-core`/`okf-analyzer`/`okf-generator` (tree-sitter, 11 langages), aucun outil externe. |
| Format persistant, versionnable, lisible en diff | ✅ Fait | Bundle OKF = un fichier Markdown+YAML par concept (`docs/graph/knowledge/`), commité comme du code normal. |
| Cache incrémental | ✅ Fait | `.okf-cache.json`, seuls les fichiers modifiés sont re-parsés (`analyze_with_cache_lsp`). |
| Relations calls/called_by avec confiance | ✅ Fait, mais figé | Chaque relation porte `resolved_by: tree-sitter` + `confidence: exact` — **la seule valeur observée dans tout le bundle est `exact`** (vérifié par grep sur ~1300 fichiers). Pas de résolution floue, pas de scoring. |
| Provenance fichier/ligne | ✅ Fait, au niveau fichier | `resource: src/agents/registry.rs#L225-L228` sur chaque concept. |
| Provenance commit Git | ⚠️ Partiel | Un seul `source_revision` (hash du commit HEAD) **par bundle entier**, dans `index.md` — pas de commit par fait individuel (pas de "cette relation a été introduite au commit X"). |
| MCP server | ❌ Absent du binaire | `init.rs` écrit une entrée `.mcp.json` qui pointe vers un binaire externe `okf-mcp` (`cargo install --git ... okf-mcp`) — **rien n'est vendorisé ni exécuté en process**. Si `okf-mcp` n'est pas installé, l'entrée `.mcp.json` est un pointeur mort. Un seul format (Claude Code) est géré ; aucun champ `supports_mcp` par agent dans `AGENTS` (`src/agents/config.rs`). |
| `search`/`explore`/`graph`/`impact`/`diff` | ❌ Externe uniquement | Documenté comme tel dans `CLAUDE.md` et `docs/okf-rs-integration-plan.md` — nécessitent `cargo install --git ... okf-cli` séparément. Aucune de ces commandes n'est appelable depuis `solidspec` lui-même. |

**Lecture** : la brique « code knowledge » (catégorie A de la proposition — *ce qui est
réellement présent*) est solide et déjà en place. C'est la seule des trois catégories de
connaissance de la vision qui existe.

## 2. Les 3 types de connaissance — Code / Projet / Historique

| Type | Proposé dans la vision | État réel |
|---|---|---|
| **A. Code Knowledge** (`Player extends CharacterBody3D`, `calls CombatSystem`) | Graphe de faits structurels | ✅ C'est exactement le bundle OKF actuel. |
| **B. Project Knowledge** (requirement, design decision, constraint, acceptance criteria) | Nœuds dédiés dans le graphe, reliés au code | ⚠️ Existe **en silo**, hors du graphe : `spec.md` (FR-###), `plan.md` (section décisions), `intent.md` (Goal/Constraints/Evidence/Risks pour le schéma `intent-driven`) sont du Markdown libre dans `specs/<feature>/`, jamais ingéré par `okf-generator`. Aucune arête `CombatSystem —requirement→ FR-012`. |
| **C. Historical Knowledge** (pourquoi une décision existe, `git blame`) | Nœuds "decision" reliés à un commit + une justification | ❌ Absent. Le seul lien Git est le `source_revision` global du bundle (voir §1). Pas de `git log -p`/`git blame` exploité, pas de nœud "decision". |

**Écart principal** : SolidSpec produit déjà B (spec/plan/intent) et une partie de C
(`solidspec ship` produit une décision SHIP/HOLD, horodatée) — mais **ni l'un ni l'autre n'est
dans le graphe**. Ce sont trois silos (bundle OKF, fichiers `specs/*.md`, historique Git) sans
pont entre eux. C'est très exactement le problème que le schéma en 3 couches de la vision
cherche à résoudre, et rien dans le code actuel ne le fait.

## 3. Evidence / Confidence typée (fact ≠ décision ≠ hypothèse)

C'est le point que l'auteur de la vision qualifie lui-même de « brique la plus importante », et
c'est **le plus grand écart constaté** :

- Le validateur OKF (`okf_validator`) et le générateur ne connaissent qu'un seul type de
  confiance : `confidence: exact` pour une résolution tree-sitter réussie. Il n'existe pas de
  notion de résolution partielle/floue en amont (pas de LSP actif branché — `--lsp` est
  explicitement exclu, voir doc-comment de `src/core/okf.rs`), donc pas non plus de valeur
  `confidence < 1.0` en sortie.
- Il n'existe aucun concept de type `Decision` ou `Inference` dans `ConceptKind` (les seuls
  types générés sont Package/Module/Struct/Enum/Function/Method — voir
  `docs/graph/GRAPH_REPORT.md`).
- `solidspec analyze`/`review`/`ship` produisent des heuristiques et des scores (`review.rs`,
  `fan_out/report.rs`) mais **en dehors du graphe**, dans des rapports Markdown à part
  (`analysis-report.md`, `review-report.md`, `ship-report.md`) sans schéma `FACT`/`DECISION`/
  `INFERENCE` structuré et sans lien retour vers un concept OKF.

## 4. DAG de workflow — ce qui existe

| Élément de la vision | État |
|---|---|
| Le DAG répond à « que faire ensuite ? », séparé du KG | ✅ Fait — `core::artifact_graph::ArtifactGraph` (Kahn topologique) + `core::schema::WorkflowSchema` (YAML) + `core::pipeline` (constantes `PHASES*`, `phases_for_schema`, `filter_phases`) ne référencent jamais `core::okf`. Séparation propre, vérifiée par grep (`schemas/*/schema.yaml` ne contient aucune mention de "knowledge"/"okf"/"graph"). |
| Étapes explicites "Query Knowledge Graph" / "Impact analysis" avant specify/plan | ❌ Absent | Aucun schéma (`spec-driven`, `tdd-driven`, `security-first`, ...) ne déclare d'artefact de ce type. L'agent IA peut interroger le MCP *de sa propre initiative* si `okf-mcp` est configuré et installé, mais ce n'est **jamais un nœud du DAG** — ni obligatoire, ni tracé, ni vérifié par `solidspec analyze`. |
| Le workflow AI-TDD de l'exemple (Understand → Query KG → Impact analysis → Spec → Tests → RED → Implement → …) | ⚠️ Partiel | Le schéma `tdd-driven` existe (RED réel avant implémentation, refactor après — `schemas/tdd-driven/schema.yaml`), mais sans les étapes "Query Knowledge Graph"/"Impact analysis" de l'exemple. |
| 7 schémas figés (minimal, spec-driven, security-first, tdd-driven, intent-driven, apex-driven, intent-apex) vs. workflows arbitraires (code review, bug investigation, migration, audit sécu, perf, release) cités dans la vision | ❌ Non couvert | Le moteur de DAG est générique (`WorkflowSchema` + `ArtifactGraph` acceptent n'importe quel schéma YAML, y compris un schéma projet-local dans `.solidspec/workflows/<name>/`), donc **rien n'empêche techniquement** d'ajouter ces workflows — mais aucun n'est fourni, ni documenté, ni testé aujourd'hui. |

## 5. Execution Engine (DAG node → Action → MCP/Shell/Git/LLM → Result → Next node)

C'est la brique la moins alignée avec la vision. `cli/pipeline.rs::execute_phase` est un `match
phase { "intent" => ..., "specify" => ..., "plan" => ..., ... }` **codé en dur, phase par
phase**, pas un moteur d'exécution générique qui interpréterait un type d'action déclaratif
(`mcp_query`, `shell`, `git`, `llm_call`) depuis le schéma YAML. Ajouter un nouveau type
d'action (ex. « appeler le MCP avant chaque phase ») demande aujourd'hui de modifier du code
Rust, pas de déclarer quelque chose dans `schema.yaml`. Ce n'est pas un défaut en soi (c'est
simple et lisible), mais **ça ne correspond pas** à l'« Execution Engine » découplé de la
vision — c'est un choix à faire consciemment, pas un oubli à corriger mécaniquement.

## 6. La boucle de mise à jour (code change → graph update → next workflow step)

❌ **N'existe pas.** Vérifié : `core::okf::generate` n'est appelé que depuis deux endroits —
`solidspec init` (une fois, à la création du projet) et la commande manuelle `solidspec okf
generate`. Aucun appel depuis `cli/pipeline.rs`, aucun hook après `implement`. Le plan
d'intégration existant (`docs/okf-rs-integration-plan.md`, étape 4) prévoyait un hook de
régénération *avant* `plan` — non implémenté (voir §8). Il n'y a *a fortiori* aucun hook
*après* implémentation, qui est pourtant l'étape la plus utile de la boucle décrite dans la
vision (« Code changed → Graph updated → New facts → Next workflow step »).

## 7. Architecture en crates séparées (KG Engine / DAG Engine / Execution Engine)

La vision propose un workspace `crates/{core,graph,okf,indexer,analyzer,git,mcp,workflow,cli}`.
Le dépôt actuel est **un seul crate** (`solidspec`) avec une séparation par **module**, pas par
crate :

```
src/core/okf.rs            ~ "KG Engine" (fin wrapper, la vraie logique est dans les crates okf-* vendorisées)
src/core/artifact_graph.rs ~ "DAG Engine" (graphe + Kahn)
src/core/schema.rs         ~ "DAG Engine" (définition YAML)
src/core/pipeline.rs       ~ moitié DAG Engine (phases), moitié Execution Engine (should_skip, phase_type)
src/cli/pipeline.rs        ~ "Execution Engine" (execute_phase, tout en dur)
src/agents/invoker.rs      ~ interface LLM (partie de l'Execution Engine)
```

C'est une séparation logique raisonnable pour la taille actuelle du projet ; migrer vers un
workspace multi-crates n'apporterait rien tant que ces modules ne sont pas réutilisés
indépendamment (ex. un futur serveur MCP natif qui aurait besoin de `core::okf` sans le reste
du CLI). À noter pour plus tard, pas urgent.

## 8. Recoupement avec le plan déjà écrit (`docs/okf-rs-integration-plan.md`)

Bonne nouvelle : ce plan existant anticipait déjà une partie de cette vision, avant même cette
discussion. État réel de ses étapes :

| Étape du plan | Statut réel |
|---|---|
| 1. Remplacer `graphify` par `okf-rs` natif | ✅ Fait |
| 2. Extension `okf` optionnelle (scaffolding) | ✅ Fait |
| 3. Enregistrement MCP par agent | ⚠️ **Partiellement fait, hors plan initial** — cette session a ajouté l'écriture de `.mcp.json` dans `solidspec init` lui-même (`src/cli/init.rs::write_okf_mcp_config`), pas gated par un « preset okf actif » comme prévu, et seulement pour Claude Code (pas de champ `supports_mcp` par agent). |
| 4. Hook de régénération avant `plan` | ❌ Non fait |
| 5. Vérification structurelle dans `analyze` (symboles de `tasks.md` existent réellement dans le graphe) | ✅ **Fait** (voir recommandation #1 ci-dessous, mise en œuvre depuis) |
| 6. Rapport d'impact dans `review`/`ship` | ❌ Non fait |

L'étape 5 est exactement le « fact-checking » que la vision met en avant — implémentée nativement via
`okf_parser::read_bundle` (pas besoin de `okf-rs search`/`explore` ni de shell-out, contrairement à ce que
la formulation initiale du plan supposait). L'étape 4 (la « boucle ») reste la plus structurante des
lacunes restantes — voir écart #1 ci-dessous, inchangé.

---

## Synthèse des écarts, par ordre de valeur ajoutée probable

1. **Aucune boucle de rétroaction code → graphe → workflow** (§6). C'est l'écart le plus
   structurant : sans lui, le graphe devient obsolète dès le premier `implement` et personne ne
   le sait.
2. **Aucun lien entre le graphe et spec/plan/decisions** (§2, catégories B et C). Le graphe
   répond à « qu'est-ce qui existe dans le code » mais jamais à « pourquoi » ni « est-ce que ça
   correspond à ce que le spec demande ».
3. ~~**Pas de vérification structurelle automatique** (étape 5 du plan)~~ — **fait** :
   `analyze` compare désormais chaque tâche de `tasks.md` (symboles entre backticks, chemins de
   fichiers existants) au bundle OKF via `core::okf::BundleIndex`, dans une section dédiée
   "Structural cross-check (okf-rs)" de `analysis-report.md`. Voir
   `docs/okf-rs-integration-plan.md` étape 5.
4. **Pas de distinction fact/decision/inference** (§3) — la plus ambitieuse et la plus
   éloignée de l'existant ; demande un nouveau schéma OKF (nouveaux `ConceptKind`), donc du
   travail côté `okf-rs` lui-même (dépôt séparé), pas seulement côté SolidSpec.
5. **MCP non natif, un seul agent géré** (§1) — dépendance à un binaire externe non vendorisé
   et jamais garanti installé ; à l'inverse de `okf.rs` (generate/validate) qui a été vendorisé
   pour cette raison précise, `okf-mcp` ne l'est pas (coût : client HTTP OpenAI-compatible +
   dépendances serveur, documenté comme choix délibéré dans `src/core/okf.rs`).
6. **Pas de nœuds DAG "Query Knowledge Graph"/"Impact analysis"** (§4) — actuellement laissé à
   l'initiative de l'agent, jamais vérifié ni obligatoire.
7. **Execution Engine codé en dur plutôt que déclaratif** (§5) — pas un bug, mais un choix de
   conception à assumer ou à faire évoluer consciemment si on veut des workflows définis
   entièrement en YAML (code review, bug investigation, migration, audit, ... cités dans la
   vision) sans toucher au Rust à chaque fois.

## Recommandation de prochaine étape concrète

1. ~~**Étape 5 du plan existant** (vérification structurelle dans `analyze`)~~ — **fait**, voir
   ci-dessus. Implémentation : `src/core/okf.rs::BundleIndex` (lit un bundle déjà généré via
   `okf_parser::read_bundle`, sans ré-analyser le code) +
   `src/core/analyzer.rs::structural_cross_check` (backticks → noms de symboles, chemins de
   fichiers → extensions reconnues), tous deux testés (`cargo test`, 585 tests) et vérifiés
   manuellement sur un vrai projet.
2. **Étape 4 du plan existant, généralisée** — pas seulement un hook avant `plan`, mais un
   appel à `core::okf::generate` après `implement` dans `cli/pipeline.rs::execute_phase`
   (best-effort, jamais bloquant, comme le reste du code d'intégration OKF) : c'est la boucle
   de rétroaction (§6), et c'est un ajout localisé (quelques lignes dans une fonction qui
   existe déjà) plutôt qu'un nouveau sous-système. Rendue plus utile encore par l'étape 1 : sans
   régénération automatique, la vérification structurelle s'appuie sur un graphe qui ne se
   rafraîchit jamais tout seul.
3. Ensuite seulement, discuter du modèle **fact/decision/inference** (§3) — c'est un vrai
   changement de format OKF, donc une discussion à avoir avec le dépôt `okf-rs` amont plutôt
   qu'un ajout unilatéral côté SolidSpec.

Prochaine étape suggérée : le point 2 (boucle de rétroaction post-`implement`).
