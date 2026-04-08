<script lang="ts">
  import { CalendarClock } from 'lucide-svelte';
</script>

<div class="flex-1 flex flex-col p-8 gap-8 overflow-y-auto w-full h-full">
  <header class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
        <CalendarClock class="w-8 h-8 text-primary" />
        Schedules
      </h1>
      <p class="text-neutral-400 mt-2">Automate system cleaning workflows based on custom time triggers.</p>
    </div>
  </header>

  <div class="bg-card border border-border p-12 rounded-xl flex flex-col items-center justify-center text-center flex-1 shadow-sm">
    <CalendarClock class="w-16 h-16 text-neutral-600 mb-6" />
    <h3 class="text-2xl font-semibold mt-2 mb-2">No Active Schedules</h3>
    <p class="text-neutral-400 max-w-md">
      You haven't configured any automatic cleaning schedules. Set up daily, weekly, or event-driven triggers to maintain your system automatically.
    </p>
  </div>
</div>
