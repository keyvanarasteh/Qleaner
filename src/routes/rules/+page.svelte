<script lang="ts">
  import { ShieldAlert } from 'lucide-svelte';
</script>

<div class="flex-1 flex flex-col p-8 gap-8 overflow-y-auto w-full h-full">
  <header class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
        <ShieldAlert class="w-8 h-8 text-primary" />
        Ignore Rules
      </h1>
      <p class="text-muted-foreground mt-2">Define explicit paths and patterns that Qleaner should never scan or delete.</p>
    </div>
  </header>

  <div class="bg-card border border-border p-12 rounded-xl flex flex-col items-center justify-center text-center flex-1 shadow-sm">
    <ShieldAlert class="w-16 h-16 text-muted-foreground mb-6" />
    <h3 class="text-2xl font-semibold mt-2 mb-2">Global Protection List</h3>
    <p class="text-muted-foreground max-w-md">
      Add specific directories, file extensions, or Regex patterns to permanently exclude them from all optimization engines.
    </p>
  </div>
</div>
