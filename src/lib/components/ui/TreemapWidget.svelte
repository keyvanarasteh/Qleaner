<script lang="ts">
  import { ShieldAlert, Cpu, Search, Trash2, Box } from 'lucide-svelte';

  let { items }: { items: { name: string; size: number; color?: string; category: string }[] } = $props();

  let totalSize = $derived(items.reduce((acc, curr) => acc + curr.size, 0));
  
  // Basic Hash map mapping categorical themes to vibrant hex hashes cleanly natively!
  const categoryColors: Record<string, string> = {
    'Browser Cache': 'bg-blue-500',
    'System Caches': 'bg-neutral-500',
    'Application Logs': 'bg-pink-500',
    'Trash': 'bg-red-500',
    'Downloads': 'bg-orange-500',
    'Privacy Caches': 'bg-purple-500',
  };

  function getColor(category: string, name: string) {
    if (categoryColors[category]) return categoryColors[category];
    // Generic deterministic color
    const hash = name.split('').reduce((a, b) => { a = ((a << 5) - a) + b.charCodeAt(0); return a & a }, 0);
    const colors = ['bg-indigo-500', 'bg-cyan-500', 'bg-teal-500', 'bg-emerald-500', 'bg-fuchsia-500'];
    return colors[Math.abs(hash) % colors.length];
  }

  function formatBytes(bytes: number) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

{#if items.length > 0}
  <div class="w-full flex flex-col gap-3">
    <!-- Telemetry Multi-Segment Bar Array! -->
    <div class="w-full h-8 flex rounded-xl overflow-hidden shadow-sm gap-0.5 group">
      {#each items as item (item.name)}
        <div 
           class="h-full {item.color || getColor(item.category, item.name)} transition-all duration-300 hover:brightness-125 relative cursor-pointer group/item flex items-center justify-center overflow-hidden" 
           style="width: {(item.size / totalSize) * 100}%; opacity: 0.9;"
           title="{item.name}: {formatBytes(item.size)}"
        >
           {#if (item.size / totalSize) > 0.05}
             <span class="text-[10px] font-bold text-white/90 truncate px-1 drop-shadow-md tracking-wider uppercase select-none">{item.name}</span>
           {/if}
           
           <div class="absolute bottom-full mb-2 left-1/2 -translate-x-1/2 bg-background border border-border text-foreground px-3 py-1.5 rounded-lg text-xs font-medium shadow-xl opacity-0 group-hover/item:opacity-100 transition-opacity pointer-events-none z-50 whitespace-nowrap flex items-center gap-2">
              <span class="w-2 h-2 rounded-full {item.color || getColor(item.category, item.name)}"></span>
              {item.name} 
              <span class="text-muted-foreground ml-1 font-mono">{formatBytes(item.size)}</span>
           </div>
        </div>
      {/each}
    </div>
    
    <!-- Legend Grid Bounds -->
    <div class="flex flex-wrap gap-x-4 gap-y-2 mt-1">
      {#each items.slice(0, 6) as item}
        <div class="flex items-center gap-2 text-xs text-muted-foreground">
           <span class="w-3 h-3 rounded-md {item.color || getColor(item.category, item.name)} opacity-80"></span>
           <span class="truncate max-w-[120px]">{item.name}</span>
           <span class="font-mono text-muted-foreground">{((item.size / totalSize) * 100).toFixed(1)}%</span>
        </div>
      {/each}
      {#if items.length > 6}
        <div class="flex items-center gap-2 text-xs text-muted-foreground italic">
          <span class="w-1.5 h-1.5 rounded-full bg-muted"></span>
          +{items.length - 6} more clusters
        </div>
      {/if}
    </div>
  </div>
{/if}
