<script lang="ts">
  import { Terminal, Trash2, X, ChevronUp, ChevronDown } from 'lucide-svelte';
  
  let isExpanded = $state(true);
  
  // Terminal Logs Placeholder
  let logs = $state<string[]>([
    "[SYSTEM] Qleaner Framework Loaded",
    "[SYSTEM] Svelte 5 DOM Virtualization Engaged",
    "[TELEMETRY] Listening for background IPC emissions..."
  ]);
  
  // We'll hook into realtime rust events later in Phase 2
</script>

<div class="w-full bg-card border-t border-border flex flex-col transition-all duration-300 {isExpanded ? 'h-48' : 'h-8'} z-[100]">
  <!-- Terminal Header -->
  <div class="h-8 flex items-center justify-between px-3 bg-neutral-900 border-b border-border/50 shrink-0 cursor-pointer select-none" onclick={() => isExpanded = !isExpanded}>
    <div class="flex items-center gap-2">
      <Terminal size={14} class="text-neutral-400" />
      <span class="text-xs font-semibold text-neutral-400 uppercase tracking-wider">Terminal Output</span>
    </div>
    <div class="flex items-center gap-2 text-neutral-500">
      <button class="hover:text-foreground transition-colors p-1" onclick={(e) => { e.stopPropagation(); logs = []; }} title="Clear Output">
        <Trash2 size={14} />
      </button>
      <button class="hover:text-foreground transition-colors p-1" title={isExpanded ? 'Collapse' : 'Expand'}>
        {#if isExpanded}
          <ChevronDown size={14} />
        {:else}
          <ChevronUp size={14} />
        {/if}
      </button>
    </div>
  </div>

  <!-- Terminal Content -->
  {#if isExpanded}
    <div class="flex-1 p-2 overflow-y-auto font-mono text-xs bg-black text-[#56b6c2] selection:bg-[#3e4451] flex flex-col gap-1 items-start">
      {#each logs as log (log)}
        <div class="hover:bg-white/5 w-full px-1 py-0.5 rounded transition-colors break-all">
          <span class="text-[#e5c07b] mr-2">></span>
          {log}
        </div>
      {/each}
      {#if logs.length === 0}
        <div class="text-neutral-600 italic">No output...</div>
      {/if}
    </div>
  {/if}
</div>
