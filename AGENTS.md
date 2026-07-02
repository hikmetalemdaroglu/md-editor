# md-editor — Proje Durumu ve Geliştirici Notları

> Proje: Tauri v2 + Rust desktop markdown editor
> Fork from: ThinkerDan/browser-markdown-editor
> GitHub: https://github.com/hikmetalemdaroglu/md-editor

## Son Durum

Visual modernization tamamlandı. Son commit: `3759594` — "Gorsel iyilestirme: modern UI, Inter font, yeni renk paleti"

## Yapılanlar (Özet)

1. **Tauri v2 + Vite + Rust** proje iskeleti kuruldu, çalışıyor
2. **Rust backend** (6 komut): `fn_read_file`, `fn_write_file`, `fn_import_folder`, `fn_export_zip`, `fn_read_config`, `fn_write_config`
3. **Editor/Viewer modu**: Kontrol çubuğunda combo kutusu ile geçiş
4. **Viewer mode**: Editor/resizer/toolbar gizlenir, preview tam ekran, alt köşede badge
5. **Config**: `%APPDATA%/md-editor/md-editor.ini` — varsayılan mod `viewer`
6. **Drag & Drop**: `.md`/`.txt` dosyaları Tauri `onDragDropEvent` ile açılır
7. **Build**: `tauri build` ile ~9.5 MB portable exe + MSI/NSIS installer üretir
8. **Görsel Modernizasyon (son commit)**:
   - Inter font (Google Fonts) + JetBrains Mono
   - Lucide icons CDN
   - Modern renk paleti (slate/blue tabanlı)
   - `border-radius`, `box-shadow`, `transition` iyileştirmeleri
   - Sidebar, tab bar, toolbar, kontroller yeniden stillendirildi
   - Preview markdown stilleri modernize edildi

## Mimari

### Frontend (`index.html`)
- Tek sayfa, `type="module"` ile Tauri API importları
- Marked.js ile MD→HTML dönüşümü
- CSS değişkenleri ile tema (açık/koyu)
- Lucide icons (CDN'den)

### Backend (`src-tauri/src/lib.rs`)
- 6 Tauri komutu, `fn_` prefix
- Özel config okuma/yazma (INI formatı)
- ZIP export (zip crate)
- Folder import (rekürsif walk)
- `#[cfg_attr(mobile, tauri::mobile_entry_point)]` mevcut

### Config (`tauri.conf.json`)
- CSP: `cdn.jsdelivr.net` ve `cdnjs.cloudflare.com` izinli
- Pencere: 1200x800, yeniden boyutlanabilir

### Bağımlılıklar
- Rust: tauri 2, tauri-plugin-dialog 2, serde, zip 2, time pinned "=0.3.36"
- npm: @tauri-apps/api ^2, @tauri-apps/plugin-dialog ^2, vite ^6

## Proje Yapısı

```
index.html              — Tek sayfa frontend (HTML/CSS/JS)
vite.config.js          — Vite yapılandırması
package.json            — npm script'leri
README.md               — Fork dokümantasyonu, build talimatları
AGENTS.md               ← Bu dosya
src-tauri/
├── Cargo.toml          — Rust bağımlılıkları
├── tauri.conf.json     — Tauri app/config yapılandırması
├── capabilities/
│   └── default.json    — İzinler (core:default, dialog:default)
├── icons/              — Uygulama ikonları
└── src/
    ├── main.rs         — Entry point
    └── lib.rs          — Backend komutları
```

## Build & Çalıştırma

```bash
npm run tauri:dev       # Geliştirme modu
npm run tauri:build     # Production build
npm run build           # Sadece frontend build
```

## Kod Kuralları (Projeye Özel)

- Tüm Rust komutları `fn_` prefix
- Config INI formatı, `%APPDATA%/md-editor/md-editor.ini`
- Varsayılan mod: `viewer`
- CSP ayarlarına yeni CDN eklenirse `tauri.conf.json` güncellenmeli
- `index.html` tek dosya frontend (tüm CSS/JS inline)

## Sıradaki (Planlanan)

- [x] Emoji ikonları Lucide SVG ile değiştir (data-lucide attribute)
  - menuToggleBtn: `☰` → `menu`
  - themeToggleBtn: `🎨` → `palette`
  - findReplaceCloseBtn: `×` → `x`
  - searchCloseBtn: `×` → `x`
  - sidebarToggleBtn: `<`/`>` → `chevron-left`/`chevron-right`
  - close-tab-btn: `×` → `x` (dinamik)
  - pin-note-btn: `📌` → `pin`/`pin-off` (dinamik, pinned durumuna göre)
- [x] İkon boyutları standardize edildi: CSS `i[data-lucide] { width:16px; height:16px }` ile tüm ikonlar aynı boyut
- [x] Sidebar collapsed toggle pozisyonu düzeltildi: `top: 58px`, `box-shadow`, `z-index` ayarlandı
- [x] Flexbox düzeni: butonlar `display:inline-flex` ile ikonları ortalar
- [x] Spacing, shadow, border-radius, transition detay iyileştirmeleri
- [ ] Test ekle (frontend lint vs.)
- [ ] GitHub'a push, release oluştur
