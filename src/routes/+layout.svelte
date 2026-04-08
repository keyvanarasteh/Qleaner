<script lang="ts">
	import { ParaglideJS } from '@inlang/paraglide-sveltekit'
	import { i18n } from '$lib/i18n'

  import '../app.css';
  import NeuralBootSequence from '$lib/components/ui/NeuralBootSequence.svelte';
  import { themeState } from '$lib/stores/theme.svelte';
  import { Sun, Moon } from 'lucide-svelte';
  
  let { children } = $props();
  let isBooted = $state(false);
</script>
<ParaglideJS {i18n}>


<NeuralBootSequence onComplete={() => isBooted = true} />

<div class="h-screen w-full flex bg-background text-foreground overflow-hidden font-sans selection:bg-primary/30 antialiased transition-opacity duration-1000 {isBooted ? 'opacity-100' : 'opacity-0'}">
  {#if isBooted}
    <!-- Floating Theme Toggle -->
    <button 
      class="absolute top-6 right-6 z-[100] p-2 rounded-full shadow-lg bg-card border border-border text-foreground hover:bg-accent hover:text-accent-foreground transition-all duration-300"
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
  {/if}
</div>

</ParaglideJS>