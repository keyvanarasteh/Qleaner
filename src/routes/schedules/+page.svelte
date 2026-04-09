<script lang="ts">
  import { CalendarClock, Plus, Trash2, Clock, Check, X } from 'lucide-svelte';
  import { schedulesStore } from '$lib/stores/schedules.svelte';

  let newCron = $state("");
  let isSubmitting = $state(false);

  async function handleAdd() {
    if (!newCron.trim()) return;
    isSubmitting = true;
    await schedulesStore.addSchedule(newCron.trim());
    newCron = "";
    isSubmitting = false;
  }
</script>

<div class="flex-1 flex flex-col p-8 gap-8 scroll-optimized w-full h-full">
  <header class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
        <CalendarClock class="w-8 h-8 text-primary" />
        Automated Schedules
      </h1>
      <p class="text-muted-foreground mt-2">Automate system cleaning workflows based on custom time triggers.</p>
    </div>
  </header>

  <!-- Add Schedule Form -->
  <div class="bg-card border border-border p-6 rounded-xl flex items-center gap-4 shadow-sm w-full max-w-2xl">
    <div class="flex-1 flex flex-col">
       <label class="text-sm text-muted-foreground mb-1" for="cron">Cron Expression</label>
       <input 
         id="cron"
         type="text" 
         class="w-full bg-background border border-border rounded-lg px-4 py-2 text-foreground focus:outline-none focus:border-primary transition-colors"
         placeholder="e.g. 0 0 * * * (Daily at midnight)"
         bind:value={newCron}
         onkeydown={(e) => e.key === 'Enter' && handleAdd()}
       />
    </div>
    <button 
      class="mt-6 bg-primary hover:bg-primary/90 text-primary-foreground px-6 py-2.5 rounded-lg flex items-center gap-2 font-medium transition-all active:scale-95 disabled:opacity-50"
      onclick={handleAdd}
      disabled={isSubmitting || !newCron.trim()}
    >
      <Plus size={18} /> Add Target
    </button>
  </div>

  <div class="bg-card border border-border rounded-xl flex-1 flex flex-col shadow-sm overflow-hidden">
    {#if schedulesStore.isLoading && schedulesStore.items.length === 0}
      <div class="flex-1 flex items-center justify-center">
        <Clock class="w-8 h-8 text-neutral-600 animate-pulse" />
      </div>
    {:else if schedulesStore.items.length === 0}
      <div class="flex-1 flex flex-col items-center justify-center p-12 text-center">
        <CalendarClock class="w-16 h-16 text-muted-foreground mb-6" />
        <h3 class="text-2xl font-semibold mt-2 mb-2">No Active Schedules</h3>
        <p class="text-muted-foreground max-w-md">
          You haven't configured any automatic cleaning schedules. Set up daily, weekly, or event-driven triggers to maintain your system automatically.
        </p>
      </div>
    {:else}
      <table class="w-full text-left text-sm whitespace-nowrap">
        <thead class="bg-muted/50">
          <tr>
            <th class="px-6 py-4 font-medium text-muted-foreground">Trigger Expression</th>
            <th class="px-6 py-4 font-medium text-muted-foreground">Status</th>
            <th class="px-6 py-4 font-medium text-muted-foreground text-right">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border">
          {#each schedulesStore.items as item (item.id)}
            <tr class="hover:bg-muted/40 transition-colors">
              <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                  <div class="w-8 h-8 bg-background border border-border rounded-lg flex items-center justify-center">
                    <Clock class="w-4 h-4 text-muted-foreground" />
                  </div>
                  <span class="font-mono text-base font-medium text-foreground">{item.cron_expr}</span>
                </div>
              </td>
              <td class="px-6 py-4">
                <button 
                  class="flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-semibold transition-colors {item.is_active ? 'bg-green-500/10 text-green-500 border border-green-500/20' : 'bg-background text-muted-foreground border border-border'}"
                  onclick={() => schedulesStore.toggleSchedule(item.id, !item.is_active)}
                >
                  {#if item.is_active} <Check size={12}/> Active {:else} <X size={12}/> Disabled {/if}
                </button>
              </td>
              <td class="px-6 py-4 text-right">
                <button 
                  class="p-2 text-red-500/70 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-colors inline-block"
                  aria-label="Delete schedule"
                  onclick={() => schedulesStore.deleteSchedule(item.id)}
                >
                  <Trash2 size={18} />
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>
