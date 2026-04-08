<script lang="ts">
  import { Settings } from 'lucide-svelte';
</script>

<div class="flex-1 flex flex-col p-8 gap-8 overflow-y-auto w-full h-full">
  <header class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
        <Settings class="w-8 h-8 text-primary" />
        Preferences
      </h1>
      <p class="text-neutral-400 mt-2">Configure application behavior, UI parameters, and system integrations.</p>
    </div>
  </header>

  <div class="bg-card border border-border p-12 rounded-xl flex flex-col items-center justify-center text-center flex-1 shadow-sm">
    <Settings class="w-16 h-16 text-neutral-600 mb-6" />
    <h3 class="text-2xl font-semibold mt-2 mb-2">Engine Configuration</h3>
    <p class="text-neutral-400 max-w-md">
      Telemetry, notification policies, language selection, and core backend behaviors are managed here.
    </p>
  </div>
</div>
