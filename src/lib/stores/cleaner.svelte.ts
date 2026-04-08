import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';
import { settingsStore } from './settings.svelte';

export interface CleanResponse {
    freed_bytes: number;
    files_deleted: number;
    errors: string[];
}


export type ScanProgress = {
  current: number;
  total: number;
  percent: number;
  current_location: string;
  found_count: number;
  total_size: number;
};

export type CacheLocation = {
  id: string;
  path: string;
  name: string;
  description: string;
  category: string;
  hint: string;
  impact: string;
  risk: string;
  size: number;
  size_human: string;
  selected: boolean;
  exists: boolean;
};

export type SystemStats = {
  cpu_percent: number;
  cpu_count: number;
  cpu_temp: number;
  memory: {
    total: number;
    used: number;
    free: number;
    percent: number;
    total_human: string;
    used_human: string;
    free_human: string;
  };
  disk: {
    total: number;
    used: number;
    free: number;
    percent: number;
    total_human: string;
    used_human: string;
    free_human: string;
  };
};

class CleanerStore {
    progress = $state<ScanProgress | null>(null);
    results = $state<CacheLocation[]>([]);
    stats = $state<SystemStats | null>(null);
    isScanning = $state(false);
    isCleaning = $state(false);

    constructor() {
        if (typeof window !== 'undefined') {
            this.initListeners();
            this.refreshStats();
            setInterval(() => this.refreshStats(), 2000);
        }
    }

    async initListeners() {
        await listen('scan-progress', (event) => {
            this.progress = event.payload as ScanProgress;
            if (this.progress.percent === 100) {
                this.isScanning = false;
                this.fetchResults();
            }
        });
    }

    async refreshStats() {
        try {
            this.stats = await invoke('get_system_stats');
        } catch (e) {
            console.error(e);
        }
    }

    async startScan() {
        this.isScanning = true;
        this.progress = { current: 0, total: 100, percent: 0, current_location: 'Starting...', found_count: 0, total_size: 0 };
        this.results = [];
        try {
            await invoke('start_scan');
        } catch (e) {
            console.error(e);
            this.isScanning = false;
        }
    }

    async fetchResults() {
        try {
             this.results = await invoke('get_scan_results');
        } catch (e) {
             console.error(e);
        }
    }

    async simulateClean(ids: string[]): Promise<{ freed_bytes: number; files_deleted: number; errors: string[] } | null> {
        this.isCleaning = true;
        try {
             return await invoke('clean_items', { items: ids, dryRun: true, useShredding: settingsStore.useShredding });
        } catch (e) {
             console.error(e);
             toast.error(String(e));
             return null;
        } finally {
            this.isCleaning = false;
        }
    }

    async cleanItems(ids: string[]) {
        this.isCleaning = true;
        try {
             const res: { freed_bytes: number; files_deleted: number; errors: string[] } = await invoke('clean_items', { items: ids, dryRun: false, useShredding: settingsStore.useShredding });
             
             if (res.errors && res.errors.length > 0) {
                 for (const err of res.errors) {
                     toast.error(err);
                 }
                 toast.warning(`Cleaned ${res.files_deleted} items with ${res.errors.length} errors.`);
             } else {
                 toast.success(`Successfully cleaned ${res.files_deleted} items.`);
             }

             await this.fetchResults();
             await this.refreshStats();
        } catch (e) {
             console.error(e);
             toast.error(String(e));
        } finally {
            this.isCleaning = false;
        }
    }

    async abortScan() {
        try {
            await invoke('cancel_scan');
        } catch (e) {
            console.error(e);
        }
    }
    
    async openFolder(path: string) {
        try {
            await invoke('plugin:opener|open', { path });
        } catch (e) {
            console.error("Failed to open folder:", e);
            toast.error("Failed to open folder");
        }
    }

    ignoreItem(id: string) {
        // Future iteration allows syncing to persistent sqlite
        toast.info("Item hidden from current view.");
        this.results = this.results.filter(r => r.id !== id);
    }

    toggleAll(checked: boolean) {
        for (let i = 0; i < this.results.length; i++) {
            this.results[i].selected = checked;
        }
    }

    toggleItem(id: string, checked: boolean) {
        const item = this.results.find(r => r.id === id);
        if (item) {
            item.selected = checked;
        }
    }
}

export const cleanerStore = new CleanerStore();
