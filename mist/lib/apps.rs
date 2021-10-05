use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use crate::types::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MistDlcData {
    pub app_id: AppId,
    pub avaliable: bool,
    pub name: *const c_char,
}

/// Get the metadata for the dlc by dlc index
/// Returns false on error
/// dlc_data is only guaranteed to be valid til the next time the function is called
#[no_mangle]
pub extern "C" fn mist_apps_get_dlc_data_by_index(dlc: i32, dlc_data: *mut MistDlcData) -> bool {
    let subprocess = get_subprocess!();
    let dlc = unwrap_client_result!(subprocess
        .client()
        .apps()
        .get_dlc_data_by_index(dlc)
        .flatten());

    static mut DLC_DATA_NAME: Option<CString> = None;
    static mut DLC_DATA: Option<MistDlcData> = None;

    unsafe {
        DLC_DATA_NAME = Some(CString::new(dlc.name).unwrap_or_default());
        DLC_DATA = Some(MistDlcData {
            app_id: dlc.app_id,
            avaliable: dlc.avaliable,
            name: DLC_DATA_NAME.as_ref().unwrap().as_ptr(),
        });

        *dlc_data = DLC_DATA.unwrap();
    }

    true
}

/// Checks if an app with the appid is installed
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_app_installed(app_id: AppId, installed: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe {
        *installed = unwrap_client_result!(subprocess.client().apps().is_app_installed(app_id))
    };
    true
}

/// Checks if the app is running in a cybercafe
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_cybercafe(is_cybercafe: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *is_cybercafe = unwrap_client_result!(subprocess.client().apps().is_cybercafe()) };
    true
}

/// Checks if a dlc with the appid is installed
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_dlc_installed(app_id: AppId, installed: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe {
        *installed = unwrap_client_result!(subprocess.client().apps().is_app_installed(app_id))
    };
    true
}

/// Checks if low violence mode is set
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_low_violence(is_low_violence: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *is_low_violence = unwrap_client_result!(subprocess.client().apps().is_cybercafe()) };
    true
}

/// Checks if the active user is subscribed to the current app
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_subscribed(is_subscribed: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *is_subscribed = unwrap_client_result!(subprocess.client().apps().is_subscribed()) };
    true
}

/// Checks if the active user is subscribed to the app id
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_subscribed_app(app_id: AppId, is_subscribed: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe {
        *is_subscribed = unwrap_client_result!(subprocess.client().apps().is_subscribed_app(app_id))
    };
    true
}

/// Checks if the active user is subscribed from family sharing
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_subscribed_from_family_sharing(
    is_subscribed_from_family_sharing: *mut bool,
) -> bool {
    let subprocess = get_subprocess!();
    unsafe {
        *is_subscribed_from_family_sharing = unwrap_client_result!(subprocess
            .client()
            .apps()
            .is_subscribed_from_family_sharing())
    };
    true
}

/// Checks if the active user is subscribed from free weekend
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_subscribed_from_free_weekend(
    is_subscribed_from_free_weekend: *mut bool,
) -> bool {
    let subprocess = get_subprocess!();
    unsafe {
        *is_subscribed_from_free_weekend =
            unwrap_client_result!(subprocess.client().apps().is_subscribed_from_free_weekend())
    };
    true
}

/// Checks if the user has a VAC ban
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_is_vac_banned(is_vac_banned: *mut bool) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *is_vac_banned = unwrap_client_result!(subprocess.client().apps().is_vac_banned()) };
    true
}

/// Get the current build id of the application
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_app_build_id(build_id: *mut BuildId) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *build_id = unwrap_client_result!(subprocess.client().apps().get_app_build_id()) };
    true
}

/// Get the install dir of the app to the app id provided
/// Returns false on error
/// app_install_dir is only guaranteed to be valid til the next time the function is called
#[no_mangle]
pub extern "C" fn mist_apps_get_app_install_dir(
    app_id: AppId,
    app_install_dir: *mut *const c_char,
) -> bool {
    let subprocess = get_subprocess!();
    let install_dir = unwrap_client_result!(subprocess.client().apps().get_app_install_dir(app_id));

    static mut APP_INSTALL_DIR: Option<CString> = None;

    match install_dir {
        Some(install) => {
            unsafe {
                APP_INSTALL_DIR = Some(CString::new(install).unwrap_or_default());
                *app_install_dir = APP_INSTALL_DIR.as_ref().unwrap().as_ptr();
            }

            true
        }
        None => {
            unsafe {
                APP_INSTALL_DIR = None;
            }
            mist_set_error!(&format!("Invalid app id to get install dir: {}", app_id));
            false
        }
    }
}

/// Get the steam id of the owner of the application
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_app_owner(steam_id: *mut SteamId) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *steam_id = unwrap_client_result!(subprocess.client().apps().get_app_owner()) };
    true
}

/// Get a comma seperated list of the avaliable game languages
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_available_game_languages(
    avaliable_languages: *mut *const c_char,
) -> bool {
    let subprocess = get_subprocess!();
    let game_languages =
        unwrap_client_result!(subprocess.client().apps().get_available_game_languages());

    static mut AVALIABLE_LANGUAGES: Option<CString> = None;

    unsafe {
        AVALIABLE_LANGUAGES = Some(CString::new(game_languages).unwrap_or_default());
        *avaliable_languages = AVALIABLE_LANGUAGES.as_ref().unwrap().as_ptr();
    }

    true
}

/// Get the name of the current beta, sets it to NULL if on the default beta/branch
/// current_beta_name is only guaranteed to be valid til the next time the function is called
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_current_beta_name(current_beta_name: *mut *const c_char) -> bool {
    let subprocess = get_subprocess!();
    let beta = unwrap_client_result!(subprocess.client().apps().get_current_beta_name());

    static mut BETA_NAME: Option<CString> = None;

    match beta {
        Some(beta) => unsafe {
            BETA_NAME = Some(CString::new(beta).unwrap_or_default());
            *current_beta_name = BETA_NAME.as_ref().unwrap().as_ptr();
        },
        None => unsafe {
            BETA_NAME = None;
            *current_beta_name = std::ptr::null_mut();
        },
    }

    true
}

/// Get the current game language
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_current_game_language(
    current_game_language: *mut *const c_char,
) -> bool {
    let subprocess = get_subprocess!();
    let current_language =
        unwrap_client_result!(subprocess.client().apps().get_current_game_language());

    static mut CURRENT_LANGUAGE: Option<CString> = None;

    unsafe {
        CURRENT_LANGUAGE = Some(CString::new(current_language).unwrap_or_default());
        *current_game_language = CURRENT_LANGUAGE.as_ref().unwrap().as_ptr();
    }

    true
}

/// Get the dlc count used for getting the dlc info by index
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_dlc_count(dlc_count: *mut i32) -> bool {
    let subprocess = get_subprocess!();
    unsafe { *dlc_count = unwrap_client_result!(subprocess.client().apps().get_dlc_count()) };
    true
}

/// Get the download progress of a dlc
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_dlc_download_progress(
    app_id: AppId,
    downloading: *mut bool,
    bytes_downloaded: *mut u64,
    bytes_total: *mut u64,
) -> bool {
    let subprocess = get_subprocess!();
    let download_progress =
        unwrap_client_result!(subprocess.client().apps().get_dlc_download_progress(app_id));

    if let Some((downloaded, total)) = download_progress {
        unsafe {
            *downloading = true;
            *bytes_downloaded = downloaded;
            *bytes_total = total;
        }
    } else {
        unsafe {
            *downloading = false;
        }
    }

    true
}

/// Get earliest purchase time for the application in unix time
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_earliest_purchase_unix_time(
    app_id: AppId,
    purchase_time: *mut u32,
) -> bool {
    let subprocess = get_subprocess!();
    unsafe {
        *purchase_time = unwrap_client_result!(subprocess
            .client()
            .apps()
            .get_earliest_purchase_unix_time(app_id))
    };
    true
}

//#[no_mangle]
//pub extern "C" fn mist_apps_get_file_details(file_name: String) -> ();

/// Writes the installed depots into a pre-allocated array named depots, sets installed_depots to the amount of depots written
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_installed_depots(
    app_id: AppId,
    depots: *mut DepotId,
    depots_size: u32,
    installed_depots: *mut u32,
) -> bool {
    let subprocess = get_subprocess!();
    let depot_ids = unwrap_client_result!(subprocess.client().apps().get_installed_depots(app_id));

    unsafe {
        let count = depots_size.min(depot_ids.len() as u32);
        std::ptr::copy(depot_ids.as_ptr(), depots, count as usize);
        *installed_depots = count;
    }

    true
}

#[no_mangle]
pub extern "C" fn mist_apps_get_launch_command_line(
    launch_command_line: *mut *const c_char,
) -> bool {
    let subprocess = get_subprocess!();
    let launch_command =
        unwrap_client_result!(subprocess.client().apps().get_current_game_language());

    static mut LAUNCH_COMMAND_LINE: Option<CString> = None;

    unsafe {
        LAUNCH_COMMAND_LINE = Some(CString::new(launch_command).unwrap_or_default());
        *launch_command_line = LAUNCH_COMMAND_LINE.as_ref().unwrap().as_ptr();
    }

    true
}

/// Get the value of the launch query param, sets it to NULL if it does not exist
/// The value is only guaranteed to be valid til the next time the function is called
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_get_launch_query_param(
    key: *const c_char,
    value: *mut *const c_char,
) -> bool {
    let subprocess = get_subprocess!();
    let key = unsafe { CStr::from_ptr(key) }.to_string_lossy().to_string();
    let param_value = unwrap_client_result!(subprocess.client().apps().get_launch_query_param(key));

    static mut QUERY_LAUNCH_PARAM: Option<CString> = None;

    match param_value {
        Some(param_value) => unsafe {
            QUERY_LAUNCH_PARAM = Some(CString::new(param_value).unwrap_or_default());
            *value = QUERY_LAUNCH_PARAM.as_ref().unwrap().as_ptr();
        },
        None => unsafe {
            QUERY_LAUNCH_PARAM = None;
            *value = std::ptr::null_mut();
        },
    }

    true
}

/// Request the dlc for the app id to be installed
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_install_dlc(app_id: AppId) -> bool {
    let subprocess = get_subprocess!();
    subprocess.client().apps().install_dlc(app_id);
    true
}

/// Request a force verify of the game
/// Set missing files only to signal that a update might have been pushed
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_mark_content_corrupt(missing_files_only: bool) -> bool {
    let subprocess = get_subprocess!();
    unwrap_client_result!(subprocess
        .client()
        .apps()
        .mark_content_corrupt(missing_files_only))
}

/// Request the dlc for the app id to be uninstalled
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_apps_uninstall_dlc(app_id: AppId) -> bool {
    let subprocess = get_subprocess!();
    subprocess.client().apps().uninstall_dlc(app_id);
    true
}
