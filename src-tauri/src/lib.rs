mod commands;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.maximize();
            }
            
            // Initialize History Database
            match commands::history::HistoryDb::init() {
                Ok(db) => {
                    app.manage(db);
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {}", e);
                }
            }

            // Initialize Flash State
            app.manage(commands::flash_as_code::FlashState::default());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(commands::log_parser::SentinelState::default())
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::adb::get_connected_devices,
            commands::adb::get_telemetry,
            commands::adb::kill_server,
            commands::adb::reboot_device,
            commands::adb::adb_sideload,
            commands::adb::adb_backup,
            commands::adb::check_battery_level,
            commands::adb::install_apk,
            commands::adb::push_file,
            commands::adb::pull_file,
            commands::adb::delete_file,
            commands::adb::create_directory,
            commands::adb::list_packages,
            commands::adb::get_storage_info,
            commands::adb::wait_for_device,
            commands::adb::wait_for_recovery,
            commands::adb::take_screenshot,
            commands::adb::list_directory,
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
            commands::fastboot::get_partition_layout,
            commands::fastboot::resize_logical_partition,
            commands::fastboot::set_active_slot,
            commands::fastboot::check_bootloader_unlocked,
            commands::devices::oneplus::generate_op7pro_flash_config,
            commands::fastboot::detect_ab_slots,
            commands::fastboot::flash_partition,
            commands::fastboot::flash_partition_stream,
            commands::fastboot::erase_partition,
            commands::fastboot::disable_verity,
            commands::fastboot::format_data,
            commands::fastboot::wait_for_bootloader,
            commands::fastboot::fastboot_boot,
            commands::fastboot::get_partition_info,
            commands::fastboot::check_dynamic_partitions,
            commands::fastboot::flash_all_partitions,
            commands::fastboot::set_active_slot_and_reboot,
            commands::fastboot::reboot,
            commands::predictive_core::calculate_success_score,
            commands::gemini::ask_gemini,
            commands::automation::start_flash_process,
            commands::usb_monitor::start_usb_monitor,
            commands::file_parser::parse_rom_file,
            commands::file_parser::verify_file_checksum,
            commands::file_parser::verify_file_md5,
            commands::file_parser::extract_zip_file,
            commands::prop_reader::get_device_props,
            commands::wipe_logic::factory_reset,
            commands::log_parser::get_logcat_dump,
            commands::log_parser::start_log_sentinel,
            commands::log_parser::stop_log_sentinel,
            commands::log_parser::analyze_log_with_ai,
            commands::flash_as_code::validate_flash_config,
            commands::flash_as_code::execute_flash_plan,
            commands::flash_as_code::cancel_flash_process,
            commands::flash_as_code::pause_flash_process,
            commands::flash_as_code::resume_flash_process,
            commands::flash_as_code::convert_script_to_config,
            commands::fastboot::bootloader_unlock,
            commands::scrcpy_manager::check_scrcpy_installed,
            commands::scrcpy_manager::start_scrcpy,
            commands::adb::take_screenshot,
            commands::adb::list_directory,
            commands::scheduler::save_schedule,
            commands::scrcpy_manager::check_scrcpy_installed,
            commands::scrcpy_manager::start_scrcpy,
            commands::fastboot::bootloader_lock,
            commands::hardware::test_cable_speed,
            commands::hardware::get_power_stats,
            commands::hardware::get_storage_health,
            commands::hardware::get_device_config_summary,
            commands::hardware::get_thermal_stats,
            commands::social::fetch_rss_feed,
            commands::flash_recovery::flash_recovery,
            commands::flash_recovery::boot_recovery,
            commands::image_patcher::patch_boot_image,
            commands::http_client::download_file,
            commands::http_client::download_file_auto,
            commands::config::load_settings,
            commands::config::save_settings,
            commands::sys_driver::check_drivers,
            commands::sys_driver::fix_drivers,
            commands::sys_driver::check_internet_connection,
            commands::os_perm::check_permissions,
            commands::history::add_activity,
            commands::history::get_activity_log,
            commands::history::clear_activity_log,
            commands::os_perm::request_permissions,
            commands::shell_stream::start_shell_stream,
            commands::automation::get_required_downloads,
            commands::rescue::trigger_rescue_mode,
            commands::nlp_cli::parse_natural_language,
            commands::nlp_cli::execute_cli_command,
            commands::rom_recommender::recommend_roms,
            commands::context_bot::analyze_error_context,
            commands::config_gen::generate_config_from_url,
            commands::brick_prevention::check_brick_risk,
            commands::rom_tools::get_device_inventory,
            commands::rom_tools::extract_file_from_zip,
            commands::rom_tools::get_gapps_url,
            commands::rom_tools::check_firmware_compliance,
            commands::sentiment::analyze_sentiment,
            commands::ocr::translate_image,
            commands::hardware::test_cable_speed,
            commands::hardware::get_power_stats,
            commands::hardware::get_storage_health,
            commands::hardware::get_device_config_summary,
            commands::hardware::run_screen_test,
            commands::hardware::toggle_touch_test,
            commands::hardware::calculate_device_health_score,
            commands::hardware::generate_health_report,
            commands::social::fetch_community_repos,
            commands::social::generate_share_link,
            commands::social::decode_share_link,
            commands::social::share_config,
            commands::tuner::analyze_performance,
            commands::tuner::apply_tuning,
            commands::battery_ai::analyze_battery,
            commands::network_opt::optimize_mirrors,
            commands::zero_touch::get_zero_touch_state,
            commands::zero_touch::set_zero_touch_state,
            commands::zero_touch::check_zero_touch_trigger,
            commands::zero_touch::start_zero_touch_service,
            commands::zero_touch::cancel_zero_touch,
            commands::cloud_sync::list_github_artifacts,
            commands::cloud_sync::list_recent_runs,
            commands::multi_device::execute_batch_action,
            commands::recovery_script::generate_ors_script,
            commands::recovery_script::save_ors_script,
            commands::app_manager::get_debloat_lists,
            commands::app_manager::batch_uninstall,
            commands::app_manager::batch_install_apks,
            commands::module_manager::list_magisk_modules,
            commands::module_manager::toggle_module,
            commands::module_manager::remove_module,
            commands::module_manager::install_module_zip,
            commands::module_manager::get_denylist,
            commands::module_manager::toggle_denylist,
            commands::module_manager::backup_modules,
            commands::module_manager::remove_all_modules,
            commands::prop_editor::get_all_props,
            commands::prop_editor::set_prop,
            commands::prop_editor::delete_prop,
            commands::prop_editor::generate_prop_module,
            commands::rom_tools::get_gapps_url,
            commands::rom_tools::check_firmware_compliance,
            commands::rom_tools::get_device_firmware_info,
            commands::rom_tools::extract_file_from_zip,
            commands::security::verify_file_hash,
            commands::security::calculate_file_hash,
            commands::security::backup_efs_partitions,
            commands::security::install_safetynet_fix,
            commands::security::audit_permissions,
            commands::security::revoke_permission,
            commands::security::encrypt_file,
            commands::security::decrypt_file,
            commands::security::lock_bootloader,
            commands::security::kill_switch,
            commands::social::fetch_community_repos,
            commands::social::share_config,
            commands::social::sync_dev_profile,
            commands::social::generate_share_link,
            commands::social::decode_share_link,
            commands::social::download_feed_item,
            commands::social::search_community,
            commands::cloud_sync::backup_app_data,
            commands::cloud_sync::restore_app_data,
            commands::system_ops::resize_partition,
            commands::system_ops::parse_payload_bin,
            commands::system_ops::extract_payload_partition,
            commands::system_ops::install_kernelsu,
            commands::system_ops::switch_dual_boot_slot,
            commands::system_ops::stream_payload_extraction,
            commands::system_ops::install_dsu,
            commands::system_ops::flash_kernel_image,
            commands::hardware::test_cable_speed,
            commands::hardware::get_power_stats,
            commands::hardware::get_thermal_stats,
            commands::hardware::get_storage_health,
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
