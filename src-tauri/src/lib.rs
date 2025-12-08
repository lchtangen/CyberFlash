mod commands;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::adb::get_connected_devices,
            commands::adb::kill_server,
            commands::adb::reboot_device,
            commands::adb::adb_sideload,
            commands::adb::adb_backup,
            commands::adb::check_battery_level,
            commands::adb::install_apk,
            commands::adb::push_file,
            commands::adb::pull_file,
            commands::adb::list_packages,
            commands::adb::get_storage_info,
            commands::adb::wait_for_device,
            commands::adb::wait_for_recovery,
            commands::adb::adb_screenshot,
            commands::adb::adb_screen_record,
            commands::adb::check_root_status,
            commands::adb::get_display_density,
            commands::adb::get_ip_address,
            commands::adb::enable_wireless_debugging,
            commands::adb::connect_wireless,
            commands::adb::backup_partition_image,
            commands::adb::remove_lock_files,
            commands::adb::enable_camera2,
            commands::adb::get_prop_value,
            commands::adb::set_prop_value,
            commands::adb::list_third_party_apps,
            commands::adb::uninstall_package,
            commands::adb::get_battery_details,
            commands::adb::run_adb_shell,
            commands::fastboot::get_fastboot_devices,
            commands::fastboot::get_var_all,
            commands::fastboot::set_active_slot,
            commands::fastboot::check_bootloader_unlocked,
            commands::fastboot::detect_ab_slots,
            commands::fastboot::flash_partition,
            commands::fastboot::erase_partition,
            commands::fastboot::disable_verity,
            commands::fastboot::format_data,
            commands::fastboot::wait_for_bootloader,
            commands::fastboot::fastboot_boot,
            commands::fastboot::get_partition_info,
            commands::fastboot::check_dynamic_partitions,
            commands::fastboot::flash_all_partitions,
            commands::fastboot::set_active_slot_and_reboot,
            commands::gemini::ask_gemini,
            commands::automation::start_flash_process,
            commands::usb_monitor::start_usb_monitor,
            commands::file_parser::parse_rom_file,
            commands::file_parser::verify_file_checksum,
            commands::file_parser::extract_zip_file,
            commands::prop_reader::get_device_props,
            commands::wipe_logic::factory_reset,
            commands::log_parser::get_logcat_dump,
            commands::fastboot::bootloader_unlock,
            commands::fastboot::bootloader_lock,
            commands::flash_recovery::flash_recovery,
            commands::flash_recovery::boot_recovery,
            commands::image_patcher::patch_boot_image,
            commands::http_client::download_file,
            commands::http_client::download_file_auto,
            commands::config::load_settings,
            commands::config::save_settings,
            commands::sys_driver::check_drivers,
            commands::sys_driver::check_internet_connection,
            commands::os_perm::check_permissions,
            commands::os_perm::request_permissions,
            commands::shell_stream::start_shell_stream,
            commands::automation::get_required_downloads
        ])
        .run(tauri::generate_context!())




        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("World");
        assert_eq!(result, "Hello, World! You've been greeted from Rust!");
    }
}
