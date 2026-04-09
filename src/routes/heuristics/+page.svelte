<script lang="ts">
  import { ShieldCheck, HardDrive, PackageX, UserCog, History, BrainCircuit } from 'lucide-svelte';

  const categories = [
    {
      id: 'system',
      title: 'System Caches',
      icon: HardDrive,
      color: 'text-blue-400',
      description: 'Standard OS and user-level cache repositories.',
      items: [
        { name: 'User Caches', path: '~/Library/Caches' },
        { name: 'System Logs', path: '~/Library/Logs' },
        { name: 'Temporary Files', path: '/tmp & AppData/Local/Temp' }
      ]
    },
    {
      id: 'privacy',
      title: 'Privacy & Forensics',
      icon: ShieldCheck,
      color: 'text-emerald-400',
      description: 'Browser telemetry and deep diagnostic logs.',
      items: [
        { name: 'Chrome Privacy History', path: '.../Google/Chrome/Default/History' },
        { name: 'Extensions Telemetry', path: '.../Google/Chrome/Default/Extensions' }
      ]
    },
    {
      id: 'containers',
      title: 'Containerization',
      icon: PackageX,
      color: 'text-cyan-400',
      description: 'Orphaned and dangling Docker artifacts.',
      items: [
        { name: 'Build Cache', path: 'docker://build_cache' },
        { name: 'Dangling Images', path: 'docker://dangling_images' },
        { name: 'Stopped Containers', path: 'docker://stopped_containers' },
        { name: 'Dangling Volumes', path: 'docker://volumes' }
      ]
    },
    {
      id: 'developer',
      title: 'Developer Evictions',
      icon: UserCog,
      color: 'text-fuchsia-400',
      description: 'Package managers and IDE build outputs.',
      items: [
        { name: 'Xcode DerivedData', path: '~/Library/Developer/Xcode/DerivedData' },
        { name: 'NPM Global Cache', path: '~/.npm' },
        { name: 'PNPM Store', path: '~/.local/share/pnpm/store' },
        { name: 'Yarn Cache', path: '~/.cache/yarn' },
        { name: 'Rust Cargo Registry', path: '~/.cargo/registry' },
        { name: 'Deep Node Modules', path: '~/.../node_modules' },
        { name: 'Deep Rust Targets', path: '~/.../target' }
      ]
    },
    {
      id: 'leftovers',
      title: 'Leftover Orphans',
      icon: History,
      color: 'text-rose-400',
      description: 'Residues from incompletely removed applications.',
      items: [
        { name: 'Preferences', path: '~/Library/Preferences/*.plist' },
        { name: 'Application Support', path: '~/Library/Application Support/*' },
        { name: 'Launch Agents', path: '~/Library/LaunchAgents/*.plist' },
        { name: 'Containers', path: '~/Library/Containers/*' }
      ]
    }
  ];

  let mermaidContainer: HTMLElement | undefined = $state();

  $effect(() => {
    if (!mermaidContainer) return;
    
    let isCancelled = false;

    import('mermaid').then((m) => {
      if (isCancelled) return;
      const mermaid = m.default;
      mermaid.initialize({
        startOnLoad: false,
        theme: 'base',
        themeVariables: {
          primaryColor: '#09090b',    // bg-background
          primaryTextColor: '#fafafa',// text-foreground
          primaryBorderColor: '#27272a', // border-border
          lineColor: '#2dd4bf',       // teal-400
          secondaryColor: '#18181b',  // bg-card
          tertiaryColor: '#27272a',
          fontFamily: 'inherit'
        }
      });
      // Render the graph by querying the DOM child
      const target = mermaidContainer?.querySelector('.mermaid-target');
      if (target) {
        mermaid.run({ nodes: [target as HTMLElement] }).catch(console.error);
      }
    });

    return () => {
      isCancelled = true;
    };
  });
</script>

<div class="flex-1 flex flex-col overflow-hidden bg-background h-full text-foreground relative">
	
	<!-- VSCode Editor Tab Bar -->
	<div class="h-9 flex items-center bg-neutral-900 border-b border-border pl-2 shrink-0 select-none pt-1">
		<div class="px-4 h-full flex items-center gap-2 bg-background border-x border-t border-border rounded-t-sm text-[13px] text-foreground font-medium relative top-px">
			<BrainCircuit size={14} class="text-primary" />
			<span>engine.heuristics</span>
		</div>
	</div>

	<!-- Top App Toolbar (Dense Console Version) -->
	<div class="h-12 flex items-center justify-between px-4 border-b border-border/50 shrink-0 bg-background">
		<div class="flex flex-col">
			<span class="text-xs uppercase tracking-widest text-muted-foreground font-bold">Detection Heuristics</span>
			<span class="text-[10px] text-primary/70 font-mono">Intelligent algorithms fueling rapid cross-platform cache and orphan detection</span>
		</div>
	</div>

  <!-- Main Workspace -->
	<div class="flex-1 min-h-0 flex flex-col bg-background relative shadow-inner overflow-hidden">
    <div class="flex-1 overflow-auto bg-background/50 p-6">

    <!-- Bento Grid Categories -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each categories as category (category.id)}
        <div class="bg-card border border-border rounded-2xl p-6 flex flex-col hover:border-primary/50 transition-colors">
          <div class="flex items-center gap-3 mb-4">
            <category.icon class="w-6 h-6 {category.color}" />
            <h2 class="text-xl font-semibold">{category.title}</h2>
          </div>
          <p class="text-sm text-neutral-400 mb-6 flex-1">{category.description}</p>
          <div class="space-y-2">
            {#each category.items as item, i (i)}
              <div class="flex flex-col p-2 bg-neutral-900/50 rounded-lg text-xs">
                <span class="font-medium text-neutral-200">{item.name}</span>
                <span class="text-neutral-500 font-mono mt-1 break-all">{item.path}</span>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>

    <!-- Algorithm Workflow Diagram -->
    <div class="bg-card border border-border rounded-2xl overflow-hidden mt-12 shadow-sm" bind:this={mermaidContainer}>
      <div class="bg-neutral-900/40 border-b border-border px-6 py-4 flex items-center justify-between">
        <h2 class="text-xl font-semibold flex items-center gap-2">
          <BrainCircuit class="text-primary w-5 h-5" />
          Leftover Discovery Algorithm Pipeline
        </h2>
      </div>
      <div class="p-8 flex justify-center items-center bg-black/20 overflow-x-auto">
        <pre class="mermaid-target opacity-90 m-0">
flowchart TD
    A[Trigger Scan] -->|mdfind| B[Discover Application Bundles]
    A -->|fs::read_dir| C[Scan /Applications]
    B --> D[Parse Info.plist]
    C --> D
    D --> E[Extract CFBundleIdentifier]
    E --> F((Installed IDs Registry))
    
    F -.-> G[Group Container Sync]
    F -.-> H[Preference Sync]
    F -.-> I[App Support Sync]
    F -.-> J[Launch Agent Sync]
    F -.-> K[Cache Sync]

    G --> L[Compare System Directories vs Registry]
    H --> L
    I --> L
    J --> L
    K --> L

    L --> M(Matching Base Bundle ID Found?)
    M -- Yes --> N[Safe / Ignore]
    M -- No --> O[Flag as Orphan Leftover]
    O --> P[Calculate Size via walk_dir]
        </pre>
      </div>
    </div>
  </div>
</div>
</div>
