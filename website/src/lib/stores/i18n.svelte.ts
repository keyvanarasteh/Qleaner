import { setLocale, getLocale, baseLocale, locales } from '$lib/paraglide/runtime';

type AvailableLanguageTag = (typeof locales)[number];

class I18nStore {
	lang = $state<AvailableLanguageTag>(baseLocale as AvailableLanguageTag);

	constructor() {
		// Browser boundary
		if (typeof window !== 'undefined') {
			const saved = localStorage.getItem('site_lang') as AvailableLanguageTag;
			if (saved && (locales as ReadonlyArray<string>).includes(saved)) {
				this.lang = saved;
				setLocale(saved, { reload: false });
			} else {
				setLocale(this.lang, { reload: false });
			}
		} else {
            try {
                this.lang = getLocale() as AvailableLanguageTag;
            } catch {
                this.lang = baseLocale as AvailableLanguageTag;
            }
        }
	}

	setLang(tag: AvailableLanguageTag) {
		this.lang = tag;
		setLocale(tag, { reload: false });
		if (typeof window !== 'undefined') {
			localStorage.setItem('site_lang', tag);
			window.location.reload(); // Complete reload guarantees UI refresh across static pages natively
		}
	}
}

export const i18nStore = new I18nStore();
