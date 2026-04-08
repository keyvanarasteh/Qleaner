<script lang="ts">
  import '../app.css';
  import NeuralBootSequence from '$lib/components/ui/NeuralBootSequence.svelte';
  
  let { children } = $props();
  let isBooted = $state(false);
</script>

<NeuralBootSequence onComplete={() => isBooted = true} />

<div class="h-screen w-full flex bg-background text-foreground overflow-hidden font-sans selection:bg-primary/30 antialiased transition-opacity duration-1000 {isBooted ? 'opacity-100' : 'opacity-0'}">
  {#if isBooted}
    {@render children()}
  {/if}
</div>
