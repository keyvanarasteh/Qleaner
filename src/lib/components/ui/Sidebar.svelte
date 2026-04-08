<script lang="ts">
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import { 
    LayoutDashboard, 
    ShieldAlert, 
    CalendarClock, 
    Settings, 
    Info, 
    Heart, 
    MessageSquare,
    Activity,
    ScrollText
  } from 'lucide-svelte';

  let currentPath = $derived($page.url.pathname);

  type ValidRoute = '/' | '/rules' | '/schedules' | '/settings' | '/about' | '/donate' | '/feedback' | '/history';

  const mainNav: Array<{name: string, path: ValidRoute, icon: any}> = [
    { name: 'Dashboard', path: '/', icon: LayoutDashboard },
    { name: 'Rules', path: '/rules', icon: ShieldAlert },
    { name: 'Schedules', path: '/schedules', icon: CalendarClock },
    { name: 'Audit', path: '/history', icon: ScrollText },
    { name: 'Settings', path: '/settings', icon: Settings }
  ];

  const bottomNav: Array<{name: string, path: ValidRoute, icon: any}> = [
    { name: 'About', path: '/about', icon: Info },
    { name: 'Donate', path: '/donate', icon: Heart },
    { name: 'Feedback', path: '/feedback', icon: MessageSquare }
  ];
</script>

<aside class="w-64 bg-card border-r border-border flex flex-col h-full shadow-lg relative z-20 transition-all">
  <div class="h-20 flex items-center px-6 border-b border-border/50">
    <div class="flex items-center gap-3 text-primary">
      <Activity class="w-7 h-7" />
      <span class="text-xl font-bold tracking-tight text-foreground">Qleaner</span>
    </div>
  </div>

  <div class="flex-1 py-6 px-4 flex flex-col gap-2 overflow-y-auto">
    <div class="text-xs font-semibold text-neutral-500 uppercase tracking-wider mb-2 px-2">Main</div>
    {#each mainNav as item (item.name)}
      <a 
        href={resolve(item.path)}
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg font-medium transition-all duration-200 {currentPath === item.path ? 'bg-primary/10 text-primary' : 'text-neutral-400 hover:text-foreground hover:bg-neutral-800/50'}"
      >
        <item.icon size={20} class={currentPath === item.path ? 'text-primary' : 'text-neutral-500'} />
        {item.name}
      </a>
    {/each}
  </div>

  <div class="p-4 border-t border-border/50 flex flex-col gap-1">
    {#each bottomNav as item (item.name)}
      <a 
        href={resolve(item.path)}
        class="flex items-center gap-3 px-3 py-2 rounded-md transition-colors text-sm {currentPath === item.path ? 'bg-neutral-800 text-foreground' : 'text-neutral-500 hover:text-foreground hover:bg-neutral-800/50'}"
      >
        <item.icon size={16} />
        {item.name}
      </a>
    {/each}
  </div>
</aside>
