<script lang="ts">
	import { ParaglideJS } from '@inlang/paraglide-sveltekit'
	import { i18n } from '$lib/i18n'

  import '../app.css';
  import NeuralBootSequence from '$lib/components/ui/NeuralBootSequence.svelte';
  import Sidebar from '$lib/components/ui/Sidebar.svelte';
  import LogTerminal from '$lib/components/ui/LogTerminal.svelte';
  import { themeState } from '$lib/stores/theme.svelte';
  import { Sun, Moon, Minus, Square, X, Monitor, ShieldAlert } from 'lucide-svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  
  let { children } = $props();
  let isBooted = $state(false);
  let hasFullDiskAccess = $state(true);

  onMount(async () => {
    try {
      hasFullDiskAccess = await invoke('check_system_disk_access');
    } catch(e) {
      console.error(e);
    }
  });
</script>
<ParaglideJS {i18n}>


<NeuralBootSequence onComplete={() => isBooted = true} />

{#if isBooted}
<div class="h-10 w-full fixed top-0 left-0 z-[900] select-none">
    <!-- Drag layer: sits behind everything, handles window dragging -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
        class="absolute inset-0 bg-background/80 backdrop-blur-md border-b border-border"
        onmousedown={() => getCurrentWindow().startDragging()}
    ></div>

    <!-- Foreground content: logo + window controls -->
    <div class="relative z-10 h-full flex items-center justify-between px-4 pointer-events-none">
        <div class="flex items-center gap-2">
            <Monitor size={14} class="text-primary" />
            <span class="text-xs font-semibold text-neutral-400 tracking-wide">QLEANER</span>
        </div>
        
        <div class="flex items-center gap-1 pointer-events-auto">
            <button onclick={() => getCurrentWindow().minimize()} class="p-1.5 rounded-md text-neutral-400 hover:bg-neutral-800 hover:text-foreground transition-colors" title="Minimize" aria-label="Minimize window">
                <Minus size={14} />
            </button>
            <button onclick={() => getCurrentWindow().toggleMaximize()} class="p-1.5 rounded-md text-neutral-400 hover:bg-neutral-800 hover:text-foreground transition-colors" title="Maximize" aria-label="Maximize window">
                <Square size={14} />
            </button>
            <button onclick={() => getCurrentWindow().close()} class="p-1.5 rounded-md text-neutral-400 hover:bg-red-500 hover:text-white transition-colors" title="Close" aria-label="Close window">
                <X size={14} />
            </button>
        </div>
    </div>
</div>
{/if}

<div class="h-screen w-full flex pt-10 bg-background text-foreground overflow-hidden font-sans selection:bg-primary/30 antialiased transition-opacity duration-1000 {isBooted ? 'opacity-100' : 'opacity-0'}">
  {#if isBooted}
    <Sidebar />
    <div class="flex-1 flex flex-col h-full min-w-0 relative">
      {#if !hasFullDiskAccess}
        <div class="w-full bg-red-500/10 border-b border-red-500/20 px-4 py-2 flex items-center justify-between text-sm shrink-0">
          <div class="flex items-center gap-2 text-red-400">
            <ShieldAlert size={16} />
            <span class="font-medium">Full Disk Access Required:</span> Qleaner needs privacy capabilities to scan system caches effectively.
          </div>
          <button 
            class="px-3 py-1 bg-red-500/20 hover:bg-red-500/30 text-red-500 rounded-md font-medium transition-colors"
            onclick={() => invoke('open_privacy_settings')}
          >
            Open Settings
          </button>
        </div>
      {/if}
      <!-- Floating Theme Toggle -->
      <button 
        class="absolute top-4 right-4 z-[100] p-1.5 rounded bg-transparent text-neutral-400 hover:text-foreground transition-all duration-300 pointer-events-auto"
        onclick={() => themeState.toggle()}
        aria-label="Toggle Theme"
        title="Toggle Light/Dark Mode"
      >
        {#if themeState.isDark}
          <Sun size={18} />
        {:else}
          <Moon size={18} />
        {/if}
      </button>
      
      <!-- Editor Zone -->
      <main class="flex-1 flex flex-col min-h-0 relative overflow-hidden">
        {@render children()}
      </main>
      
      <!-- Bottom Console Panel -->
      <LogTerminal />
    </div>
  {/if}
</div>

</ParaglideJS>