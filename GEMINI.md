# Qleaner Araç ve Mimari Kuralları (AI Agent Core Directives)

Bu belge, **Qleaner** (Cross-platform Sistem Temizleyici ve Mimari Asistan) deposuna kod yazacak, refactoring yapacak veya analiz yürütecek tüm AI (Gemini/Claude vb.) ajanlarının asgari referans kaynağıdır.

## 1. Profesyonel GitHub ve Git İş Akışı
Qleaner projesi elit açık kaynak standartlarına (**Pro GitHub Config**) sahiptir. Aşağıdaki kurallara tam itaat gereklidir:
- **Dal İsimlendirmesi:** Ana dal (default branch) `main` olarak yapılandırılmıştır. Tüm işlemler için `main` çekilmeli ve alt dallar (feature/bugfix) oluşturulmalıdır.
- **Dal Koruması (Branch Protection):** `main` doğrudan commit kabul etmez! Yaptığınız değişiklikleri `git push origin my-feature-branch` ile uzak sunucuya itmeli ve GitHub Issues/PR sürecine sokmalısınız. PR'lar **"Linear History"**, 1 review onayı ve statü kontrolleri geçtikten sonra birleşebilir.
- **Topluluk ve Telemetri:** Açacağınız Issue'lar için depodaki yerleşik "Bug Report" veya "Feature Request" Template'lerini (şablonlarını) kullanın. GitHub Discussions bölümü teknik tartışmalar için daima aktiftir.

## 2. Svelte 5 & Frontend Mimari Sınırları
- **Svelte 4 Reaktivitesi Yasaktır:** Değişken atamaları veya reaktivite blokları için Svelte 4 sözdiziminden tamamen kurtulun.
  - Sadece `$state()`, `$derived()`, ve `$effect()` Runes mekanizmalarını inşa edin.
  - Props bildirimleri için `let { foo, bar }: Props = $props();` kullanın.
- **Reaktif Performans:** Saniye başına yüzlerce MB temizlik bilgisinin ekrana akacağı anlarda, DOM güncellemelerini kilitlememek için listelemelerde her `{#each}` dögüsüne mutlak suretle eşsiz `(key)` tanımlaması yapın. (Örn: `{#each files as f (f.filePath)}`)
- **Strict Typing:** `any` kullanımı kabul edilemez.

## 3. Rust, Tauri v2 & Sistem Güvenliği (Backend)
- **Disk I/O Operasyonları:** Arka planda devasa dizin taramaları (Directory Traversal) gerçekleştirilecektir. `std::fs` yerine asenkron donanım gücü olan `tokio::fs` ve performanstan ödün vermeden tarama yapan `jwalk` / `ignore` gibi paralel I/O kütüphaneleri kullanılmalıdır.
- **UI Blokajı:** Tauri komutları asenkron olmalıdır (`async fn command() -> Result<T, Error>`). Engine, dosyaları tararken frontend takılmamalı, progress eventleri Tauri'nin `Window::emit` yapısıyla periyodik olarak UI katmanına iletilmelidir.
- **Sıfır Hata Politikası (Safety):** Kodun hiçbir yerinde `unsafe` rust bloğu açılamaz. Bellek taşması oluşabilecek sistem erişimlerinde yetki kontrolü yapılmalıdır.
- **Telemetri Opt-in İşlemi:** Kullanıcıların çökme (Sentry/Crash) raporları alınırken mutlak PII (Personel Identifiable Information) güvenliği uygulanmalıdır. Olası bir 'Panic' izini loglarken kişi dosya adları, spesifik bilgisayar bilgisi filtrelenmiş şekilde gitmelidir.

## 4. UI/UX "Qleaner" Estetiği
Tasarımlar TailwindCSS v4 altyapısı ile beslenir. Standart renk temamız; neon data göstergelerini barındıran Obsidiyen (simsiyah) uzay / laboratuvar hissiyatlı profesyonel koyu teknoloji temalarıdır. Tıpkı kodlar gibi, UI hatları da jilet kadar keskin olmalıdır.
## 5. DevOps, Progress Tracking & Telegram Reporting (MANDATORY)
- **TODO.md Updates:** As you progress, `TODO.md` MUST be marked accordingly with details: `[x] done`, `[/] ongoing`, `[ ] implemented`, `[ ] incomplete-implemented`, or `[-] skipped` (including the exact reason, info, and details if skipped).
  - YOU MUST update the `📈 Progress Statistics` header at the top of the file on every single state change.
  - If a task is requested by the user or discovered globally that is missing, implicitly add it directly to `TODO.md`.
- **Commits:** You must commit and push your code on every stable point.
- **Telegram Reporting:** You MUST report each progress on commits via Telegram to the admin IDs utilizing the API token below:
  - BOT_TOKEN: `8619989850:AAHO-tPqLa88lTLzz7SmhtyK8dwn5aTfZoY`
  - DevOps Manager (keyvan arasteh): `1426904527`
  - Developer (aziz efe cirak): `1905445451`

## 6. Svelte & Tauri AI Integration Libraries (MANDATORY)
You must utilize and strictly adhere to these external directories whenever you implement UI/Backend code:
- **Svelte Rules:** `file:///home/drvoid/Qix/QDebugger/assets/agents/rules/svelte`
- **Tauri Rules:** `file:///home/drvoid/Qix/QDebugger/assets/agents/rules/tauri`
- **Svelte Skills:** `file:///home/drvoid/Qix/QDebugger/assets/agents/skills/svelte`
- **Tauri Skills:** `file:///home/drvoid/Qix/QDebugger/assets/agents/skills/tauri`
- **Svelte Workflows:** `file:///home/drvoid/Qix/QDebugger/assets/agents/workflows/svelte`
- **Tauri Workflows:** `file:///home/drvoid/Qix/QDebugger/assets/agents/workflows/tauri`
