<script lang="ts">
  import { Terminal, Trash2, X, ChevronUp, ChevronDown } from 'lucide-svelte';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import type { ScanProgress, CacheLocation } from '$lib/stores/cleaner.svelte';
  
  let isExpanded = $state(true);
  
  // Terminal Logs Placeholder
  let logs = $state<string[]>([
    "[SYSTEM] Qleaner Framework Loaded",
    "[SYSTEM] Svelte 5 DOM Virtualization Engaged",
    "[TELEMETRY] Listening for background IPC emissions..."
  ]);
  
  let scrollContainer: HTMLElement | null = $state(null);

  function addLog(msg: string) {
      if (logs.length > 500) {
          logs = logs.slice(250); // keep it lightweight (circular buffer)
      }
      logs.push(msg);
      // Auto-scroll
      requestAnimationFrame(() => {
          if (scrollContainer) {
              scrollContainer.scrollTop = scrollContainer.scrollHeight;
          }
      });
  }

  onMount(() => {
      const u1 = listen('scan-progress', (e) => {
          const p = e.payload as ScanProgress;
          if (p.current % 50 === 0 || p.percent === 100) {
            addLog(`[SCAN] Tracing metrics -> ${p.current_location}`);
          }
      });
      const u2 = listen('clean-progress', (e) => {
          const p = e.payload as ScanProgress;
          addLog(`[SHRED] Purging block -> ${p.current_location}`);
      });
      const u3 = listen('scan-result-item', (e) => {
          const item = e.payload as CacheLocation;
          addLog(`[HIT] Orphan Target Identified: ${item.name} | ${item.size_human}`);
      });
      const u4 = listen('leftover-scan-progress', (e) => {
          const p = e.payload as ScanProgress;
          if (p.current % 5 === 0) {
              addLog(`[HEURISTIC] Analyzing artifact dependencies -> ${p.current_location}`);
          }
      });

      return () => {
          u1.then(f => f());
          u2.then(f => f());
          u3.then(f => f());
          u4.then(f => f());
      };
  });
</script>

<div class="w-full bg-card border-t border-border flex flex-col transition-all duration-300 {isExpanded ? 'h-48' : 'h-8'} z-[100]">
  <!-- Terminal Header -->
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="h-8 flex items-center justify-between px-3 bg-muted border-b border-border/50 shrink-0 cursor-pointer select-none" onclick={() => isExpanded = !isExpanded}>
    <div class="flex items-center gap-2">
      <Terminal size={14} class="text-muted-foreground" />
      <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Terminal Output</span>
    </div>
    <div class="flex items-center gap-2 text-muted-foreground">
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
    <div bind:this={scrollContainer} class="flex-1 p-2 overflow-y-auto font-mono text-xs bg-black text-[#56b6c2] selection:bg-[#3e4451] flex flex-col gap-1 items-start">
      {#each logs as log, i (i)}
        <div class="hover:bg-white/5 w-full px-1 py-[1px] rounded transition-colors break-all">
          <span class="text-[#e5c07b] mr-2">></span>
          {log}
        </div>
      {/each}
      {#if logs.length === 0}
        <div class="text-muted-foreground italic">No output...</div>
      {/if}
    </div>
  {/if}
</div>
