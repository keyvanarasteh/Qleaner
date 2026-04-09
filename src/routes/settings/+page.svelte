<script lang="ts">
  import { Settings, CheckCircle2, Monitor, Languages, HandMetal, Moon, Sun, Lock } from 'lucide-svelte';
  import { themeState } from '$lib/stores/theme.svelte';
  import { cleanerStore } from '$lib/stores/cleaner.svelte';
  import { settingsStore } from '$lib/stores/settings.svelte';

  // Dynamic architecture detection
  let arch = $state('--');
  let selectedLang = $state('en');

  if (typeof window !== 'undefined') {
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes('aarch64') || ua.includes('arm64')) arch = 'aarch64';
    else if (ua.includes('x86_64') || ua.includes('x64') || ua.includes('amd64') || ua.includes('wow64')) arch = 'x86_64';
    else if (ua.includes('i686') || ua.includes('i386')) arch = 'x86';
    else arch = navigator.platform || 'unknown';

    selectedLang = localStorage.getItem('qleaner-lang') || 'en';
  }

  function handleLangChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    selectedLang = target.value;
    localStorage.setItem('qleaner-lang', selectedLang);
  }
</script>

<div class="flex-1 flex flex-col p-8 gap-8 overflow-y-auto w-full h-full">
  <header class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
        <Settings class="w-8 h-8 text-primary" />
        Preferences
      </h1>
      <p class="text-muted-foreground mt-2">Configure application behavior, UI parameters, and system integrations.</p>
    </div>
  </header>

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 pb-12">
    <!-- Theme & Language -->
    <div class="flex flex-col gap-6">
      <div class="bg-card border border-border p-6 rounded-xl shadow-sm hover:border-primary/50 transition-colors">
        <div class="flex items-center gap-3 mb-6">
          <Monitor class="text-primary w-6 h-6" />
          <h3 class="text-xl font-semibold text-foreground">Appearance</h3>
        </div>
        <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
          <div>
            <p class="font-medium">Theme Mode</p>
            <p class="text-sm text-muted-foreground">Toggle dark and light aesthetics</p>
          </div>
          <button 
            onclick={() => themeState.toggle()}
            class="p-3 bg-background border border-border hover:bg-muted text-foreground rounded-lg transition-colors flex items-center gap-2"
          >
            {#if themeState.isDark}
              <Sun size={18} /> Light
            {:else}
              <Moon size={18} /> Dark
            {/if}
          </button>
        </div>
      </div>

      <div class="bg-card border border-border p-6 rounded-xl shadow-sm hover:border-primary/50 transition-colors">
        <div class="flex items-center gap-3 mb-6">
          <Languages class="text-primary w-6 h-6" />
          <h3 class="text-xl font-semibold text-foreground">Localization</h3>
        </div>
        <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
          <div>
            <p class="font-medium">Language</p>
            <p class="text-sm text-muted-foreground">Select application interface language</p>
          </div>
          <select 
            class="bg-background border border-border text-foreground py-2 px-4 rounded-lg focus:ring-1 focus:ring-primary outline-none"
            value={selectedLang}
            onchange={handleLangChange}
          >
            <option value="en">English (US)</option>
            <option value="tr">Türkçe</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Security & System -->
    <div class="flex flex-col gap-6">
      <div class="bg-card border border-border p-6 rounded-xl shadow-sm hover:border-primary/50 transition-colors">
        <div class="flex items-center gap-3 mb-6">
          <Lock class="text-primary w-6 h-6" />
          <h3 class="text-xl font-semibold text-foreground">Permissions</h3>
        </div>
        <div class="flex flex-col gap-3">
          <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
            <span class="font-medium">File System Access</span>
            <span class="px-3 py-1 bg-green-500/20 text-green-500 text-sm font-medium rounded-full flex items-center gap-1">
              <CheckCircle2 size={14} /> Granted
            </span>
          </div>
          <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
            <span class="font-medium">Network Access</span>
            <span class="px-3 py-1 bg-red-500/20 text-red-500 text-sm font-medium rounded-full">
              Restricted
            </span>
          </div>
        </div>
      </div>
      
      <div class="bg-card border border-border p-6 rounded-xl shadow-sm hover:border-primary/50 transition-colors">
        <div class="flex items-center gap-3 mb-6">
          <Lock class="text-primary w-6 h-6" />
          <h3 class="text-xl font-semibold text-foreground">Deletion Security</h3>
        </div>
        <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
          <div>
            <p class="font-medium">DoD 5220.22-M Shredding</p>
            <p class="text-sm text-muted-foreground">Multi-pass cryptographic overwrite bounds</p>
          </div>
          <button 
            onclick={() => settingsStore.toggleShredding()}
            class="p-3 bg-background border border-border hover:bg-muted text-foreground rounded-lg transition-colors flex items-center gap-2 {settingsStore.useShredding ? 'text-red-500 border-red-500/50' : ''}"
          >
            {#if settingsStore.useShredding}
              <CheckCircle2 size={18} /> Enabled
            {:else}
              <CheckCircle2 size={18} class="opacity-30" /> Standard
            {/if}
          </button>
        </div>
      </div>

      <div class="bg-card border border-border p-6 rounded-xl shadow-sm hover:border-primary/50 transition-colors">
        <div class="flex items-center gap-3 mb-6">
          <HandMetal class="text-primary w-6 h-6" />
          <h3 class="text-xl font-semibold text-foreground">OS Environment</h3>
        </div>
        <div class="p-4 bg-muted/50 rounded-lg flex flex-col gap-2">
          <div class="flex justify-between text-sm">
            <span class="text-muted-foreground">Architecture</span>
            <span class="font-medium font-mono text-foreground">{arch}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-muted-foreground">CPU Thread Count</span>
            <span class="font-medium font-mono text-foreground">{cleanerStore.stats?.cpu_count || '--'} Cores</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-muted-foreground">Total System Memory</span>
            <span class="font-medium font-mono text-foreground">{cleanerStore.stats?.memory.total_human || '--'}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
