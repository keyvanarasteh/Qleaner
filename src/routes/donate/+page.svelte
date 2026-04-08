<script lang="ts">
  import { Heart, Coffee } from 'lucide-svelte';
</script>

<div class="max-w-2xl mx-auto py-16 px-6 text-center">
  <div class="inline-flex items-center justify-center w-24 h-24 rounded-full bg-red-500/10 text-red-500 mb-8">
    <Heart size={48} />
  </div>

  <h1 class="text-4xl font-bold mb-4 text-foreground">Support Our Work</h1>
  
  <p class="text-xl text-foreground/80 mb-8 leading-relaxed">
    Qleaner is 100% free and open source. If you find our forensic cleanup tools valuable, consider buying the team a coffee to support development.
  </p>

  <div class="bg-card border border-border rounded-2xl p-8 mb-8 text-left space-y-6">
    <h2 class="text-2xl font-semibold flex items-center gap-3">
      <Coffee size={24} class="text-yellow-500" /> Buy us a Coffee
    </h2>
    <p class="text-foreground/70">
      Your donations directly support the maintenance of our advanced OS heuristic servers and open-source infrastructure.
    </p>
    <a href="#" class="inline-block bg-[#FF813F] text-white font-bold py-3 px-8 rounded-lg hover:opacity-90 transition-opacity">
      Buy me a coffee →
    </a>
  </div>
</div>
