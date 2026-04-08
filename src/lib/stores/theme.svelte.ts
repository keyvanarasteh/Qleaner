export type ThemeMode = 'system' | 'light' | 'dark';

class ThemeStore {
    // We use standard Reactivity patterns
    mode = $state<ThemeMode>('system');
    isDark = $state(true);

    constructor() {
        if (typeof window !== 'undefined') {
            const saved = localStorage.getItem('qleaner-theme') as ThemeMode;
            if (saved) {
                this.mode = saved;
            }
            this.updateTheme();
            
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
                if (this.mode === 'system') {
                    this.updateTheme();
                }
            });
        }
    }

    setMode(newMode: ThemeMode) {
        this.mode = newMode;
        if (typeof window !== 'undefined') {
            localStorage.setItem('qleaner-theme', newMode);
        }
        this.updateTheme();
    }

    toggle() {
        const next = this.mode === 'dark' ? 'light' : 'dark';
        this.setMode(next);
    }

    private updateTheme() {
        if (typeof window === 'undefined') return;
        
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        this.isDark = this.mode === 'dark' || (this.mode === 'system' && prefersDark);
        
        if (this.isDark) {
            document.documentElement.classList.add('dark');
            document.documentElement.classList.remove('light');
        } else {
            document.documentElement.classList.add('light');
            document.documentElement.classList.remove('dark');
        }
    }
}

export const themeState = new ThemeStore();
