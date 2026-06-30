# 🚀 Live News TUI: The Ultimate Terminal Intelligence Aggregator

**Live News TUI** adalah platform agregator berita berbasis Terminal User Interface (TUI) tingkat elit yang dirancang untuk kecepatan, privasi, dan efisiensi maksimal.

---

## ✨ Fitur Unggulan

### 🕵️ Stealth Scraping Engine (Hybrid Architecture)
Ditenagai oleh library `Scrapling` di sisi Python yang diintegrasikan secara native melalui `PyO3`.
- **Anti-Bot Bypass**: Menembus proteksi Cloudflare (403/429) secara otomatis.
- **Stealthy Sessions**: Mensimulasikan sidik jari browser manusia asli.
- **Non-blocking UI**: Operasi scraping dan pencarian berjalan di background tanpa membekukan antarmuka.

### 🔍 Custom Global Search (DuckDuckGo Integration)
Cari berita apapun di seluruh dunia secara real-time:
- **Global Search (`s`)**: Pencarian global menggunakan engine DuckDuckGo.
- **Instant Filtering**: Filter hasil secara instan langsung di terminal.

### 🌐 Konfigurasi Sumber Fleksibel
- **`sources.toml`**: Tambahkan atau hapus sumber berita dengan mudah melalui file konfigurasi eksternal.
- **Cakupan Masif**: Mendukung ratusan sumber berita global mulai dari Geopolitik, Finance, Tech, hingga Crypto.

### ⚡ Performa Workstation Modern
- **Rust/Tokio Core**: Arsitektur asinkron sepenuhnya.
- **SQLite WAL Mode**: Database teroptimasi untuk manajemen ribuan artikel.
- **Production-Ready Logging**: Pencatatan aktivitas sistem yang mendalam untuk kemudahan pemantauan.

---

## 🏛️ Arsitektur Sistem

Aplikasi ini menggunakan model hybrid yang menggabungkan kecepatan sistem **Rust** dengan fleksibilitas ekosistem scraping **Python**. Komunikasi antar bahasa dilakukan melalui `PyO3`, dan UI dibangun di atas `Ratatui` dengan model event asinkron.

---

## 🛠️ Panduan DevOps (One-Command)

Aplikasi ini mendukung otomatisasi penuh untuk siklus hidup aplikasi:

### 1. Instalasi Satu Perintah
Mengunduh dependensi, mengatur environment Python (venv), mengompilasi kode Rust, dan memasang binary ke sistem.
```bash
./install.sh
```

### 2. Update Satu Perintah
Menarik pembaruan terbaru dari repositori dan melakukan pembangunan ulang otomatis.
```bash
./update.sh
```

### 3. Uninstal Satu Perintah
Menghapus binary dari sistem dengan opsi untuk membersihkan data dan konfigurasi.
```bash
./uninstall.sh
```

---

## ⌨️ Navigasi & Pintasan Keyboard

| Tombol | Aksi |
| :--- | :--- |
| `s` | **Global Search**: Cari topik berita apapun di dunia (via DDG) |
| `/` | **Local Search**: Filter berita di kategori yang sedang dibuka |
| `t` | **Theme**: Ganti tema (Black, White, DeepBlue, Matrix) |
| `o` | **Open**: Buka URL berita di Browser sistem default |
| `Enter` | **Read**: Baca detail artikel di terminal |
| `Esc / q` | **Back**: Kembali ke daftar berita atau keluar aplikasi |
| `h / l` | **Category**: Navigasi antar tab kategori |
| `j / k` | **Navigate**: Scroll daftar berita |
| `?` | **Help**: Tampilkan jendela bantuan |

---

## 📄 Lisensi
Proyek ini **100% Open Source & Gratis** selamanya.

---
*Built with ❤️ by Senior Rust Engineers for the global intelligence community.*
