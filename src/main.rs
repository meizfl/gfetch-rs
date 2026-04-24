use sysinfo::{System, Disks};
use std::env;
use xcap::Monitor;

fn main() {
    // 1. Инициализация системы
    let mut sys = System::new_all();
    sys.refresh_all();

    // 2. USER @ HOSTNAME
    let user = whoami::username().unwrap_or_else(|_| "unknown".to_string());
    let host = whoami::hostname().unwrap_or_else(|_| "unknown".to_string());

    // 3. OS, KERNEL & ARCH
    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel_version = get_kernel_version(); // Теперь берем версию ядра
    let arch = System::cpu_arch();

    // Формат: "Arch Linux 6.6.10-arch1-1 (x86_64)"
    let os_info = format!("{} {} ({})", os_name, kernel_version, arch);

    // 4. SHELL
    let shell = env::var("SHELL").unwrap_or_else(|_| {
        env::var("COMSPEC").unwrap_or_else(|_| "cmd/powershell".to_string())
    });

    // 5. UPTIME
    let uptime_secs = System::uptime();
    let uptime_str = format_uptime(uptime_secs);

    // 6. RAM (MiB)
    let ram_total = sys.total_memory() / 1024 / 1024;
    let ram_used = sys.used_memory() / 1024 / 1024;

    // 7. CPU
    let cpu_name = sys.cpus().first()
    .map(|cpu| cpu.brand().to_string())
    .unwrap_or_else(|| "Unknown CPU".to_string());

    // 8. RESOLUTION
    let resolution = get_resolution();

    // 9. FILESYSTEM & STORAGE
    fn find_best_disk<'a>(disks: &'a Disks) -> Option<&'a sysinfo::Disk> {
        let exe_path = env::current_exe().ok();
        let cwd_path = env::current_dir().ok();

        let mut candidates = Vec::new();

        if let Some(path) = exe_path {
            candidates.push(path);
        }
        if let Some(path) = cwd_path {
            candidates.push(path);
        }

        // 1. Пытаемся найти диск, который является самым точным префиксом пути
        for path in &candidates {
            let best = disks.iter()
            .filter(|d| path.starts_with(d.mount_point()))
            .max_by_key(|d| d.mount_point().as_os_str().len());

            if best.is_some() {
                return best;
            }
        }

        // 2. fallback: просто первый диск
        disks.iter().next()
    }

    let mut fs_type = "N/A".to_string();
    let mut storage_str = "N/A".to_string();

    let disks = Disks::new_with_refreshed_list();

    if let Some(disk) = find_best_disk(&disks) {
        fs_type = disk.file_system().to_string_lossy().to_string();

        let total = disk.total_space() / 1024 / 1024 / 1024;
        let available = disk.available_space() / 1024 / 1024 / 1024;
        let used = total.saturating_sub(available);

        storage_str = format!("{} / {} GB", used, total);
    }

    // ВЫВОД (с выровненными колонками)
    let ascii = vec![
        "    (\\(\\",
        "   j\". ..",
        "   (  . .)",
        "   |   ° ¡",
        "   ¿     ;",
        "   c?\".UJ",
    ];

    let info = vec![
        format!("{}@{}", user, host),
        "-----------".to_string(),
        format!("os: {}", os_info),
        format!("shell: {}", shell),
        format!("uptime: {}", uptime_str),
        format!("ram: {}/{} MiB", ram_used, ram_total),
        format!("cpu: {}", cpu_name),
        format!("resolution: {}", resolution),
        format!("fs: {}", fs_type),
        format!("storage: {}", storage_str),
    ];

    print_gfetch_fast(ascii, info);
}
/// Получает версию ядра
fn get_kernel_version() -> String {
    sysinfo::System::kernel_version()
    .unwrap_or_else(|| "Unknown".to_string())
}

/// Пытается получить разрешение экрана
pub fn get_resolution() -> String {
    let Ok(monitors) = Monitor::all() else {
        return "N/A".to_string();
    };

    let mut res = Vec::new();

    for m in monitors {
        let w = m.width().unwrap_or(0);
        let h = m.height().unwrap_or(0);

        if w > 0 && h > 0 {
            res.push(format!("{}x{}", w, h));
        }
    }

    if res.is_empty() {
        "N/A".to_string()
    } else {
        res.join(" | ")
    }
}
/// Форматирует секунды в читаемый вид
fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
fn print_gfetch_fast(
    ascii: Vec<&str>,
    info: Vec<String>,
) {
    let ascii_width = 13;
    for i in 0..info.len() {
        let left = ascii.get(i).copied().unwrap_or("");
        let right = &info[i];

        println!("{:<width$}  {}", left, right, width = ascii_width);
    }
}
