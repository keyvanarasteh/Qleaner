<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Shield, Clock, HardDrive, Trash2 } from 'lucide-svelte';

  type AuditLog = {
    id: number;
    path: string;
    size_reclaimed: number;
    timestamp: string;
    signature: string;
  };

  let logs = $state<AuditLog[]>([]);
  let loading = $state(true);
  let loaded = $state(false);

  function formatBytes(bytes: number) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  $effect(() => {
    if (loaded) return;
    loaded = true;
    invoke<AuditLog[]>('get_audit_logs')
      .then((data) => {
        logs = data;
        loading = false;
      })
      .catch((err) => {
        console.error("Failed loading logs:", err);
        loading = false;
      });
  });
</script>

<svelte:head>
  <title>Audit Logs - Qleaner</title>
</svelte:head>

<div class="flex-1 flex flex-col overflow-hidden bg-background h-full text-foreground relative">
	
	<!-- VSCode Editor Tab Bar -->
	<div class="h-9 flex items-center bg-neutral-900 border-b border-border pl-2 shrink-0 select-none pt-1">
		<div class="px-4 h-full flex items-center gap-2 bg-background border-x border-t border-border rounded-t-sm text-[13px] text-foreground font-medium relative top-px">
			<Shield size={14} class="text-primary" />
			<span>audit.log</span>
		</div>
	</div>

	<!-- Top App Toolbar (Dense Console Version) -->
	<div class="h-12 flex items-center justify-between px-4 border-b border-border/50 shrink-0 bg-background">
		<div class="flex flex-col">
			<span class="text-xs uppercase tracking-widest text-muted-foreground font-bold">Immutable Telemetry</span>
			<span class="text-[10px] text-primary/70 font-mono">Cryptographically signed metrics of every filesystem sweeping operation</span>
		</div>
	</div>

  <!-- Main Workspace -->
	<div class="flex-1 min-h-0 flex flex-col bg-background relative shadow-inner font-mono text-[13px] overflow-hidden">
    <div class="flex-1 overflow-auto bg-background relative">
    {#if loading}
      <div class="absolute inset-0 flex items-center justify-center">
        <div class="animate-pulse text-neutral-500 font-mono tracking-widest flex items-center gap-3 text-sm">
          <Shield size={18} class="animate-spin" />
          QUERYING METRICS...
        </div>
      </div>
    {:else if logs.length === 0}
      <div class="p-8 text-center text-neutral-500 flex flex-col items-center justify-center h-full gap-4">
        <Shield size={32} class="opacity-30" />
        <p class="text-sm border border-border/50 p-2 rounded bg-card/60">No audit signatures discovered on this volume.</p>
      </div>
    {:else}
      <table class="w-full text-left border-collapse min-w-[800px]">
        <thead>
          <tr class="bg-neutral-900 border-b border-border sticky top-0 z-10 text-[11px] font-bold tracking-wider text-muted-foreground uppercase h-8">
            <th class="px-4 font-bold w-12 border-r border-border/40">ID</th>
            <th class="px-4 font-bold border-r border-border/40">Location Target</th>
            <th class="px-4 font-bold w-32 border-r border-border/40">Data Reclaimed</th>
            <th class="px-4 font-bold w-[340px] border-r border-border/40">HMAC-SHA256 Sig</th>
            <th class="px-4 font-bold w-48 text-right">Timestamp</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border/20 text-xs">
          {#each logs as log (log.id)}
            <tr class="hover:bg-neutral-800/80 transition-colors group h-8">
              <td class="px-4 text-neutral-500 border-r border-border/10">{log.id}</td>
              <td class="px-4 flex items-center gap-3 h-8 border-r border-border/10 w-full overflow-hidden">
                <Trash2 size={12} class="text-red-400 shrink-0" />
                <span class="truncate text-neutral-300 font-sans font-medium" title={log.path}>
                  {log.path}
                </span>
              </td>
              <td class="px-4 border-r border-border/10">
                <div class="flex items-center gap-1 text-emerald-400 font-semibold font-sans w-fit whitespace-nowrap">
                  {formatBytes(log.size_reclaimed)}
                </div>
              </td>
              <td class="px-4 border-r border-border/10">
                <code class="text-[10px] text-primary/60 truncate block w-[320px]" title={log.signature}>
                  {log.signature}
                </code>
              </td>
              <td class="px-4 text-right text-neutral-500 whitespace-nowrap">
                <div class="flex items-center justify-end gap-2 text-[11px]">
                  {new Date(log.timestamp + 'Z').toLocaleString()}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
    </div>
  </div>
</div>
