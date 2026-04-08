<script lang="ts">
  import { Info, Code } from 'lucide-svelte';
</script>

<div class="max-w-3xl mx-auto py-16 px-6">
  <div class="flex items-center gap-4 mb-8">
    <div class="w-16 h-16 bg-primary/20 text-primary rounded-2xl flex items-center justify-center">
      <Info size={32} />
    </div>
    <div>
      <h1 class="text-4xl font-bold text-foreground">About Qleaner</h1>
      <p class="text-foreground/60 text-lg">Cross-Platform OS Optimization Engine</p>
    </div>
  </div>
  
  <div class="space-y-6 text-lg text-foreground/80 leading-relaxed">
    <p>
      Qleaner evolved from a forensic macOS proof-of-concept into a modular, highly extensible system cleanup suite. 
      Backed by the Rust programming language and the Tauri native framework, Qleaner guarantees minimal resource consumption and true memory safety.
    </p>

    <div class="bg-card border border-border rounded-xl p-6 my-8">
      <h3 class="text-xl font-bold text-foreground mb-4">Core Principles</h3>
      <ul class="space-y-3">
        <li class="flex items-center gap-3"><span class="w-2 h-2 rounded-full bg-primary"></span> Memory Safe architecture (Rust)</li>
        <li class="flex items-center gap-3"><span class="w-2 h-2 rounded-full bg-primary"></span> Modular OOP backend heuristics</li>
        <li class="flex items-center gap-3"><span class="w-2 h-2 rounded-full bg-primary"></span> 100% Free & Open-Source Software</li>
        <li class="flex items-center gap-3"><span class="w-2 h-2 rounded-full bg-primary"></span> Multi-Environment Support</li>
      </ul>
    </div>

    <p>
      We are fully open-source. Discover our source code, report an issue, or contribute to improving our heuristic engine directly over on GitHub.
    </p>

    <div class="pt-6">
      <a href="https://github.com/keyvanarasteh/Qleaner" target="_blank" class="inline-flex items-center gap-2 bg-foreground text-background px-6 py-3 rounded-lg font-semibold hover:opacity-90 transition">
        <Code size={20} /> View Source Code
      </a>
    </div>
  </div>
</div>
