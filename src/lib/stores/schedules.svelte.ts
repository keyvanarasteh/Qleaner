import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';

export type ScheduleItem = {
    id: number;
    cron_expr: string;
    is_active: boolean;
};

class SchedulesStore {
    items = $state<ScheduleItem[]>([]);
    isLoading = $state(false);

    constructor() {
        if (typeof window !== 'undefined') {
            this.loadSchedules();
        }
    }

    async loadSchedules() {
        this.isLoading = true;
        try {
            this.items = await invoke('get_schedules');
        } catch (e) {
            console.error(e);
            toast.error(String(e));
        } finally {
            this.isLoading = false;
        }
    }

    async addSchedule(cronExpr: string) {
        try {
            await invoke('add_schedule', { cronExpr });
            toast.success("Schedule created successfully");
            await this.loadSchedules();
        } catch (e) {
            toast.error(String(e));
        }
    }

    async toggleSchedule(id: number, isActive: boolean) {
        try {
            await invoke('toggle_schedule', { id, isActive });
            await this.loadSchedules();
        } catch (e) {
            toast.error(String(e));
        }
    }

    async deleteSchedule(id: number) {
        try {
            await invoke('delete_schedule', { id });
            toast.success("Schedule deleted");
            await this.loadSchedules();
        } catch (e) {
            toast.error(String(e));
        }
    }
}

export const schedulesStore = new SchedulesStore();
