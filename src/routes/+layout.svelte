<script lang="ts">
  import '../app.css';
  let { children } = $props();
</script>

<div class="h-screen w-full flex bg-background text-foreground overflow-hidden font-sans selection:bg-primary/30 antialiased">
  {@render children()}
</div>
