<script lang="ts">
  import { Shield, Sparkles, Cpu, Send } from 'lucide-svelte';
</script>

<div class="max-w-3xl mx-auto py-12 px-6">
  <h1 class="text-4xl font-bold mb-6 text-foreground">Feedback & Support</h1>
  
  <p class="text-lg text-foreground/80 mb-8 leading-relaxed">
    Encountered an issue or have a suggestion to make Qleaner better? We'd love to hear from you. Please provide as much detail as possible so we can improve our forensic OS cleanup tools.
  </p>

  <form class="space-y-6" onsubmit={(e) => { e.preventDefault(); alert('Feedback submitted! Thank you.'); }}>
    <div class="space-y-2">
      <label for="name" class="text-sm font-medium">Name</label>
      <input id="name" type="text" class="w-full bg-card border border-border rounded-md px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary" placeholder="Your Name" />
    </div>

    <div class="space-y-2">
      <label for="email" class="text-sm font-medium">Email Address (Optional)</label>
      <input id="email" type="email" class="w-full bg-card border border-border rounded-md px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary" placeholder="you@domain.com" />
    </div>

    <div class="space-y-2">
      <label for="message" class="text-sm font-medium">Message Details</label>
      <textarea id="message" rows="5" class="w-full bg-card border border-border rounded-md px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary" placeholder="Describe your experience or issues..."></textarea>
    </div>

    <button type="submit" class="w-full sm:w-auto bg-primary text-primary-foreground hover:bg-primary/90 px-8 py-3 rounded-md font-semibold transition flex items-center justify-center gap-2">
      <Send size={18} /> Send Feedback
    </button>
  </form>
</div>
