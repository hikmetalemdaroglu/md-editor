# md-editor — Desktop Markdown Editor

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Fork of** [ThinkerDan/browser-markdown-editor](https://github.com/ThinkerDan/browser-markdown-editor) — Converted from a single-file HTML browser app to a **native Windows desktop application** using **Tauri v2 + Rust**.

---

## About

Orijinal proje, kısıtlı ortamlarda (kurulum izni olmayan iş bilgisayarları, public bilgisayarlar) kullanılmak üzere tek bir `.html` dosyasından çalışan bir Markdown editörüydü.

Bu fork ile aynı özellikler **Windows masaüstü uygulamasına** dönüştürüldü:

| Orijinal | Bu Fork |
|----------|---------|
| Tarayıcıda çalışır (`.html`) | Native `.exe` (Windows) |
| Browser File API ile dosya işlemleri | Native dialog + Rust fs |
| JSZip (CDN) ile ZIP çıktısı | Rust `zip` crate ile native ZIP |
| `showDirectoryPicker()` ile klasör içe aktarma | Rust ile recursive klasör okuma |
| Vite ile build | Tauri v2 + Vite + Rust |

---

## Özellikler

- **Live Markdown Preview:** Gerçek zamanlı bölünmüş ekran (Düzenleyici | Önizleme), [marked.js](https://marked.js.org/) kullanır
- **Çoklu Not Yönetimi:** Sekmeler, kenar çubuğu, açılır menü
- **Not Organizasyonu:** Sabitleme, sıralama, etiketleme (`#tag`)
- **Dosya İşlemleri:**
  - Tek notu `.md` olarak kaydetme / açma
  - Birden çok `.md`/`.txt` dosyasını içe aktarma
  - Klasör içe aktarma (alt klasörler dahil)
  - Tüm notları **ZIP** olarak dışa aktarma
- **Düzenleme Araçları:** Biçimlendirme araç çubuğu, Bul/Değiştir, otomatik eşleştirme, otomatik liste devamı
- **Oturum Kalıcılığı:** Notlar `localStorage`'a otomatik kaydedilir
- **Açık/Koyu Tema**
- **Klavye Kısayolları:** `Ctrl+S` (Kaydet), `Ctrl+N` (Yeni), `Ctrl+F` (Bul), `Ctrl+B` (Kalın), `Ctrl+I` (İtalik)

---

## Gereksinimler

- **Windows 10/11** (WebView2 yüklü olmalı — Windows 11'de varsayılan, Windows 10'da genelde var)
- **.exe** çalıştırmak için ek bir kurulum veya yönetici yetkisi gerekmez

---

## Kullanım

### Hazır Build (Önerilen)

En güncel sürümü **Releases** sayfasından indirin:

- **`md-editor.exe`** — Portable, tek dosya, direkt çalıştır
- **`Markdown Editor_...-setup.exe`** — NSIS kurulum sihirbazı
- **`Markdown Editor_...msi`** — MSI kurulum paketi

### Kendiniz Derlemek

```bash
# Bağımlılıkları yükleyin
npm install

# Geliştirme modunda çalıştırın
npm run tauri:dev

# Release build alın (.exe + installer)
npm run tauri:build
```

Build çıktıları:
- **Portable EXE:** `src-tauri/target/release/md-editor.exe`
- **NSIS Kurulum:** `src-tauri/target/release/bundle/nsis/`
- **MSI Kurulum:** `src-tauri/target/release/bundle/msi/`

---

## Proje Yapısı

```
md-editor/
├── index.html                # Frontend (HTML/CSS/JS + marked.js)
├── vite.config.js            # Vite build ayarları
├── package.json              # npm scripts ve bağımlılıklar
├── src-tauri/
│   ├── Cargo.toml            # Rust bağımlılıkları
│   ├── tauri.conf.json       # Tauri yapılandırması
│   ├── capabilities/         # İzin tanımları
│   ├── icons/                # Uygulama ikonları
│   └── src/
│       ├── main.rs           # Giriş noktası
│       └── lib.rs            # Rust backend komutları
│           ├── fn_read_file      # Dosya okuma
│           ├── fn_write_file     # Dosya yazma
│           ├── fn_import_folder  # Klasör içe aktarma
│           └── fn_export_zip     # ZIP dışa aktarma
```

---

## Orijinal Projeye Göre Değişiklikler

1. **Platform:** Tarayıcı → Native Windows (Tauri v2 + WebView2)
2. **Dosya İşlemleri:** Browser File API → Rust `std::fs` + `tauri-plugin-dialog`
3. **Klasör İçe Aktarma:** `showDirectoryPicker()` → Rust recursive walk
4. **ZIP Dışa Aktarma:** JSZip (CDN) → Rust `zip` crate
5. **Build Sistemi:** Yok (düz HTML) → Vite + `tauri build`
6. **İkonlar:** Eklendi (Tauri bundle için)

---

## Lisans

MIT License. Orijinal proje [ThinkerDan/browser-markdown-editor](https://github.com/ThinkerDan/browser-markdown-editor) baz alınmıştır.
