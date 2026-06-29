# Live News TUI 🚀

Live News TUI adalah aplikasi Terminal User Interface (TUI) berbasis Rust yang menyediakan feed berita real-time secara gratis, cepat, dan efisien. Dirancang untuk performa maksimal dengan penggunaan sumber daya minimal.

## ✨ Fitur Utama

- **Real-Time News**: Mengambil berita terbaru dari berbagai sumber RSS/Atom secara otomatis.
- **Efisiensi Tinggi**: Menggunakan SQLite untuk caching data dan optimasi rendering UI berbasis event.
- **Production-Ready**: Dilengkapi dengan manajemen retensi data, sinkronisasi latar belakang, dan konfigurasi fleksibel.
- **Hemat Sumber Daya**: Arsitektur asinkron (Tokio) memastikan penggunaan CPU dan RAM yang sangat rendah.
- **Gratis & Terbuka**: Sepenuhnya gratis untuk digunakan selamanya.

## 🏛️ Arsitektur Sistem

- **Bahasa**: Rust (Edisi 2024)
- **Database**: SQLite (Rusqlite) untuk penyimpanan persisten dan performa query cepat.
- **UI Framework**: Ratatui & Crossterm untuk pengalaman terminal yang modern dan responsif.
- **Concurrency**: Tokio runtime untuk pengambilan berita di latar belakang tanpa mengganggu UI.
- **Konfigurasi**: Format TOML untuk pengaturan yang mudah dibaca manusia.

## 🛠️ Panduan DevOps (Instalasi & Manajemen)

### 📥 Instalasi (Satu Perintah)
Gunakan skrip instalasi otomatis untuk mengunduh dependensi, mengompilasi, dan memasang biner:
```bash
./install.sh
```
*Catatan: Pastikan Anda memiliki Rust (Cargo) atau biarkan skrip memasangkannya untuk Anda.*

### 🔄 Update (Satu Perintah)
Perbarui aplikasi ke versi terbaru langsung dari repositori:
```bash
./update.sh
```

### 🗑️ Uninstall (Satu Perintah)
Hapus biner aplikasi dari sistem Anda:
```bash
./uninstall.sh
```

## ⚙️ Konfigurasi

File konfigurasi otomatis dibuat pada saat pertama kali dijalankan di lokasi standar OS Anda (misalnya `~/.config/live_news_tui/config.toml`).

Contoh Konfigurasi:
```toml
retention = "Daily"
fetch_interval_active_seconds = 60
fetch_interval_idle_seconds = 300
active_hours_start = 6
active_hours_end = 22
worker_threads = 4
```

## ⌨️ Navigasi UI

- **q / Esc**: Keluar atau Kembali.
- **Enter**: Membaca detail artikel.
- **j / k / ⬆ / ⬇**: Navigasi daftar berita.
- **h / l / ⬅ / ➡**: Berpindah kategori berita.

## 📄 Lisensi

Proyek ini sepenuhnya gratis untuk digunakan.
