import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

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

    async cleanItems(ids: string[]) {
        this.isCleaning = true;
        try {
             await invoke('clean_items', { items: ids });
             await this.fetchResults();
             await this.refreshStats();
        } catch (e) {
             console.error(e);
        } finally {
            this.isCleaning = false;
        }
    }
}

export const cleanerStore = new CleanerStore();
