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

<div class="h-full flex flex-col p-8 pb-0 max-h-screen">
  <div class="mb-8 shrink-0">
    <div class="inline-flex h-10 items-center justify-center rounded-lg bg-primary/20 text-primary px-4 mb-4 font-mono text-sm uppercase tracking-widest outline-1 outline-primary/30 shadow-[0_0_15px_rgba(var(--primary-rgb),0.3)]">
      <Shield size={16} class="mr-2" />
      Audit Logs (DoD 5220.22/HMAC)
    </div>
    <h1 class="text-4xl font-extrabold tracking-tight text-foreground mb-2">Immutable Telemetry</h1>
    <p class="text-neutral-400 max-w-2xl text-lg">
      Cryptographically signed metrics of every successful filesystem operation guaranteeing application integrity and anti-tampering verification.
    </p>
  </div>

  <div class="flex-1 overflow-auto rounded-t-xl border border-border/50 bg-card/50 backdrop-blur-sm relative">
    {#if loading}
      <div class="absolute inset-0 flex items-center justify-center">
        <div class="animate-pulse text-neutral-500 font-mono tracking-widest flex items-center gap-3">
          <Shield size={24} class="animate-spin" />
          QUERYING METRICS...
        </div>
      </div>
    {:else if logs.length === 0}
      <div class="p-8 text-center text-neutral-500 flex flex-col items-center justify-center h-full gap-4">
        <Shield size={64} class="opacity-20" />
        <p class="text-lg">No audit signatures discovered on this volume.</p>
        <p class="text-sm opacity-60">System telemetry metrics will generate here during your target sweeps.</p>
      </div>
    {:else}
      <table class="w-full text-left border-collapse min-w-[800px]">
        <thead>
          <tr class="border-b border-border/50 text-neutral-500 text-xs uppercase tracking-wider bg-card/80 sticky top-0 z-10">
            <th class="p-4 font-semibold w-12">ID</th>
            <th class="p-4 font-semibold">Location Target</th>
            <th class="p-4 font-semibold w-32">Data Reclaimed</th>
            <th class="p-4 font-semibold w-64">HMAC-SHA256 Sig</th>
            <th class="p-4 font-semibold w-48 text-right">Timestamp</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border/20 text-sm">
          {#each logs as log (log.id)}
            <tr class="hover:bg-accent/50 transition-colors group">
              <td class="p-4 text-neutral-500 font-mono">{log.id}</td>
              <td class="p-4 flex items-center gap-3 w-[100%] max-w-[0px]">
                <Trash2 size={16} class="text-destructive shrink-0" />
                <span class="truncate font-mono text-neutral-300" title={log.path}>
                  {log.path}
                </span>
              </td>
              <td class="p-4">
                <div class="flex items-center gap-2 text-emerald-400 font-mono bg-emerald-400/10 px-2 py-1 rounded border border-emerald-400/20 w-fit whitespace-nowrap">
                  <HardDrive size={12} />
                  {formatBytes(log.size_reclaimed)}
                </div>
              </td>
              <td class="p-4">
                <code class="font-mono text-xs text-primary/70 truncate block max-w-[200px]" title={log.signature}>
                  {log.signature}
                </code>
              </td>
              <td class="p-4 text-right text-neutral-500 font-mono whitespace-nowrap">
                <div class="flex items-center justify-end gap-2">
                  <Clock size={12} />
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
