export class SettingsStore {
    useShredding = $state(false);

    constructor() {
        if (typeof window !== 'undefined') {
            this.useShredding = localStorage.getItem('useShredding') === 'true';
        }
    }

    toggleShredding() {
        this.useShredding = !this.useShredding;
        if (typeof window !== 'undefined') {
            localStorage.setItem('useShredding', String(this.useShredding));
        }
    }
}

export const settingsStore = new SettingsStore();
