<script lang="ts">
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
  import { 
    LayoutDashboard, 
    ShieldAlert, 
    CalendarClock, 
    Settings, 
    Info, 
    Heart, 
    MessageSquare,
    ScrollText,
    BrainCircuit,
    Activity,
    Sun,
    Moon
  } from 'lucide-svelte';
  import { themeState } from '$lib/stores/theme.svelte';

  let currentPath = $derived(page.url.pathname);

  type ValidRoute = '/' | '/rules' | '/schedules' | '/settings' | '/about' | '/donate' | '/feedback' | '/history' | '/heuristics';

  const mainNav = [
    { name: 'Dashboard', path: '/' as ValidRoute, icon: LayoutDashboard },
    { name: 'Heuristics', path: '/heuristics' as ValidRoute, icon: BrainCircuit },
    { name: 'Rules', path: '/rules' as ValidRoute, icon: ShieldAlert },
    { name: 'Schedules', path: '/schedules' as ValidRoute, icon: CalendarClock },
    { name: 'Audit History', path: '/history' as ValidRoute, icon: ScrollText }
  ];

  const bottomNav = [
    { name: 'Settings', path: '/settings' as ValidRoute, icon: Settings },
    { name: 'About', path: '/about' as ValidRoute, icon: Info },
    { name: 'Donate', path: '/donate' as ValidRoute, icon: Heart },
    { name: 'Feedback', path: '/feedback' as ValidRoute, icon: MessageSquare }
  ];
</script>

<aside class="w-14 bg-card border-r border-border flex flex-col h-full relative z-20 transition-all select-none pt-2">
  <div class="w-full flex items-center justify-center py-4 text-primary">
    <Activity class="w-6 h-6" />
  </div>

  <div class="flex-1 py-4 flex flex-col items-center gap-4">
    {#each mainNav as item (item.name)}
      <a 
        href={resolve(item.path)}
        class="w-10 h-10 flex items-center justify-center rounded-lg transition-all duration-200 group relative {currentPath === item.path ? 'bg-primary/10 text-primary' : 'text-neutral-500 hover:text-foreground hover:bg-neutral-800/50'}"
        title={item.name}
      >
        <item.icon size={22} class={currentPath === item.path ? 'text-primary' : 'text-neutral-500 group-hover:text-foreground transition-colors'} />
        <!-- Tooltip generic fallback -->
        <span class="absolute left-12 px-2 py-1 bg-neutral-800 text-foreground text-xs rounded opacity-0 group-hover:opacity-100 pointer-events-none z-50 whitespace-nowrap shadow-lg border border-border transition-opacity">
          {item.name}
        </span>
      </a>
    {/each}
  </div>

  <div class="py-4 border-t border-border/50 flex flex-col items-center gap-4">
    {#each bottomNav as item (item.name)}
      <a 
        href={resolve(item.path)}
        class="w-10 h-10 flex items-center justify-center rounded-lg transition-all duration-200 group relative {currentPath === item.path ? 'bg-primary/10 text-primary' : 'text-neutral-500 hover:text-foreground hover:bg-neutral-800/50'}"
        title={item.name}
      >
        <item.icon size={20} class={currentPath === item.path ? 'text-primary' : 'text-neutral-500 group-hover:text-foreground transition-colors'} />
        <span class="absolute left-12 px-2 py-1 bg-neutral-800 text-foreground text-xs rounded opacity-0 group-hover:opacity-100 pointer-events-none z-50 whitespace-nowrap shadow-lg border border-border transition-opacity">
          {item.name}
        </span>
      </a>
    {/each}

    <!-- Theme Toggle -->
    <button 
      onclick={() => themeState.toggle()}
      class="w-10 h-10 flex items-center justify-center rounded-lg transition-all duration-200 group relative text-neutral-500 hover:text-foreground hover:bg-neutral-800/50 mt-2"
      aria-label="Toggle Theme"
    >
      {#if themeState.isDark}
        <Sun size={20} class="text-neutral-500 group-hover:text-amber-400 transition-colors" />
      {:else}
        <Moon size={20} class="text-neutral-500 group-hover:text-blue-400 transition-colors" />
      {/if}
      <span class="absolute left-12 px-2 py-1 bg-neutral-800 text-foreground text-xs rounded opacity-0 group-hover:opacity-100 pointer-events-none z-50 whitespace-nowrap shadow-lg border border-border transition-opacity">
        Toggle Theme
      </span>
    </button>
  </div>
</aside>

