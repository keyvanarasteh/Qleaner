<script lang="ts">
	import { ParaglideJS } from '@inlang/paraglide-sveltekit'
	import { i18n } from '$lib/i18n'

  import '../app.css';
  import NeuralBootSequence from '$lib/components/ui/NeuralBootSequence.svelte';
  import Sidebar from '$lib/components/ui/Sidebar.svelte';
  import { themeState } from '$lib/stores/theme.svelte';
  import { Sun, Moon, Minus, Square, X, Monitor } from 'lucide-svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  const appWindow = typeof window !== 'undefined' ? getCurrentWindow() : null;

  let { children } = $props();
  let isBooted = $state(false);
</script>
<ParaglideJS {i18n}>


<NeuralBootSequence onComplete={() => isBooted = true} />

{#if isBooted}
<div data-tauri-drag-region class="h-10 w-full bg-background/80 backdrop-blur-md border-b border-border fixed top-0 left-0 flex items-center justify-between px-4 z-[900] select-none">
    <div class="flex items-center gap-2 pointer-events-none">
        <Monitor size={14} class="text-primary" />
        <span class="text-xs font-semibold text-neutral-400 tracking-wide">QLEANER</span>
    </div>
    
    <div class="flex items-center gap-2">
        <button onclick={() => appWindow?.minimize()} class="p-1.5 rounded-md text-neutral-400 hover:bg-neutral-800 hover:text-foreground transition-colors" title="Minimize">
            <Minus size={14} />
        </button>
        <button onclick={() => appWindow?.toggleMaximize()} class="p-1.5 rounded-md text-neutral-400 hover:bg-neutral-800 hover:text-foreground transition-colors" title="Maximize">
            <Square size={14} />
        </button>
        <button onclick={() => appWindow?.close()} class="p-1.5 rounded-md text-neutral-400 hover:bg-red-500 hover:text-white transition-colors" title="Close">
            <X size={14} />
        </button>
    </div>
</div>
{/if}

<div class="h-screen w-full flex pt-10 bg-background text-foreground overflow-hidden font-sans selection:bg-primary/30 antialiased transition-opacity duration-1000 {isBooted ? 'opacity-100' : 'opacity-0'}">
  {#if isBooted}
    <Sidebar />
    <div class="flex-1 flex flex-col h-full min-w-0 relative">
      <!-- Floating Theme Toggle -->
      <button 
        class="absolute top-6 right-6 z-[100] p-2 rounded-full shadow-md bg-card border border-border text-foreground hover:bg-accent hover:text-accent-foreground transition-all duration-300"
        onclick={() => themeState.toggle()}
        aria-label="Toggle Theme"
        title="Toggle Light/Dark Mode"
      >
        {#if themeState.isDark}
          <Sun size={20} />
        {:else}
          <Moon size={20} />
        {/if}
      </button>
      {@render children()}
    </div>
  {/if}
</div>

</ParaglideJS>