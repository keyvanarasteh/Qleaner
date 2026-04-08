<?xml version="1.0" encoding="UTF-8"?>
<rules project="Qleaner" version="1.0.0">

  <!-- ═══════════════════════════════════════════════════════════
       PROJECT IDENTITY & GITHUB DEVOPS
  ═══════════════════════════════════════════════════════════ -->
  <section id="project-identity" source="github-config">
    <rule id="PI-01" severity="required">Project name is Qleaner. A blazing fast, cross-platform system utility and disk cleaner built with Rust, Svelte 5, and Tauri.</rule>
    <rule id="PI-02" severity="required">Default branch is `main`. Never push to or interact with `master`.</rule>
    <rule id="PI-03" severity="required">Branch Protection is ENABLED for `main`. Direct pushes are restricted. You MUST create a branch, submit a Pull Request, and pass status validations before merge.</rule>
    <rule id="PI-04" severity="required">GitHub PRs require "Linear History". Do not pollute the commit tree with ugly merge commits. Squash & Merge or Rebase patterns are enforced.</rule>
    <rule id="PI-05" severity="info">GitHub Features enabled: Discussions, Wikis, Projects, and Standardized Issue Templates (Bug & Feature). Suggest users to open standard issues.</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       BACKEND: RUST & TAURI V2
  ═══════════════════════════════════════════════════════════ -->
  <section id="backend-architecture">
    <rule id="BE-01" severity="required">Backend is strictly Rust using Tauri v2. Memory-safety is paramount for a system utility; completely avoid `unsafe` blocks.</rule>
    <rule id="BE-02" severity="required">File system traversal and disk sweeping must be asynchronous (using `tokio`). Never block the Tauri event loop or main thread.</rule>
    <rule id="BE-03" severity="required">Graceful failure: Panic states or unexpected halts must be logged to the Sentry Telemetry integration without exposing sensitive local paths or PII.</rule>
    <rule id="BE-04" severity="required">All Tauri commands must return strongly typed `Result<T, AppError>` utilizing Serde for pristine frontend transmission.</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       FRONTEND: SVELTE 5
  ═══════════════════════════════════════════════════════════ -->
  <section id="frontend-architecture" source="svelte-runes">
    <rule id="FE-01" severity="required">Svelte 5 Runes ($state, $derived, $effect) are strictly enforced. Never use legacy Svelte 4 reactivity syntax (`let x = 0`, `$: x = y`).</rule>
    <rule id="FE-02" severity="required">TailwindCSS must be used for layout and animations. Aesthetics prioritize Glassmorphism, deep dark/obsidian backgrounds, and neon magenta/teal typography.</rule>
    <rule id="FE-03" severity="required">Every `{#each}` block MUST use a key: `{#each items as item (item.id)}`. Svelte compiler arrays will warn otherwise.</rule>
    <rule id="FE-04" severity="required">Type safety: Use TypeScript strictly. No `any` types allowed for props, snippet parameters or event handlers.</rule>
    <rule id="FE-05" severity="required">Navigation: Strictly wrap internal path logic using SvelteKit's `$app/paths` `resolve()` when routing.</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       DEVOPS, TRACKING & TELEGRAM REPORTING
  ═══════════════════════════════════════════════════════════ -->
  <section id="devops-reporting">
    <rule id="DO-01" severity="required">As you progress, `TODO.md` MUST be marked with details: done, ongoing, implemented, incomplete-implemented, or skipped (with reason, info and details). YOU MUST update the `📈 Progress Statistics` header at the top of the file on every state change.</rule>
    <rule id="DO-02" severity="required">Agent MUST commit and push code on every stable point.</rule>
    <rule id="DO-03" severity="required">Report each progress on commits via Telegram. BOT_TOKEN: `8605055500:AAFEXsohgjJsWdlpoRXw1q_EDpIpop7Ecl0`. Admin IDs: `1426904527` (keyvan arasteh).</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       SVELTE & TAURI AI RESOURCES (QDEBUGGER)
  ═══════════════════════════════════════════════════════════ -->
  <section id="external-rules" source="qdebugger">
    <rule id="EXT-01" severity="required">Utilize these external repositories/directories whenever building logic or designing patterns:
    - Svelte Rules: `file:///home/drvoid/Qix/QDebugger/assets/agents/rules/svelte`
    - Tauri Rules: `file:///home/drvoid/Qix/QDebugger/assets/agents/rules/tauri`
    - Svelte Skills: `file:///home/drvoid/Qix/QDebugger/assets/agents/skills/svelte`
    - Tauri Skills: `file:///home/drvoid/Qix/QDebugger/assets/agents/skills/tauri`
    - Svelte Workflows: `file:///home/drvoid/Qix/QDebugger/assets/agents/workflows/svelte`
    - Tauri Workflows: `file:///home/drvoid/Qix/QDebugger/assets/agents/workflows/tauri`
    </rule>
    <rule id="EXT-02" severity="required">If a task does not exist in `TODO.md` but is requested by the user or identified as crucial, add it directly to `TODO.md`.</rule>
  </section>
  <!-- ═══════════════════════════════════════════════════════════
       DEVELOPMENT PROGRESS REPORTING & PLANNING
  ═══════════════════════════════════════════════════════════ -->
  <section id="dev-progress" source="gemini-directives">
    <rule id="DEV-01" severity="required">All progress MUST include structured Tasks and an implementation plan. Agents are required to rigorously break down UI and backend features before coding.</rule>
    <rule id="DEV-02" severity="required">Throughout the development process, `TODO.md` MUST be updated continuously with detailed architectures and planning to help the user decide the next steps.</rule>
  </section>

</rules>
