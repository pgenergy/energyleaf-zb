use anyhow::Result;
use esp_idf_sys::esp;
use log::info;
use std::ffi::{c_void, CString};

#[allow(unused)]
pub const DEVICE_MANUFACTURER: &str = "Energyleaf"; //CString = CString::new("Energyleaf").unwrap();

#[allow(unused)]
pub fn default_coordinator_config() -> esp_idf_svc::sys::esp_zb_cfg_t {
    esp_idf_svc::sys::esp_zb_cfg_t {
        esp_zb_role: esp_idf_svc::sys::esp_zb_nwk_device_type_t_ESP_ZB_DEVICE_TYPE_COORDINATOR,
        install_code_policy: false,
        nwk_cfg: esp_idf_svc::sys::esp_zb_cfg_s__bindgen_ty_1 {
            zczr_cfg: esp_idf_svc::sys::esp_zb_zczr_cfg_t {
                max_children: 10, //max connected deviced
            },
        },
    }
}

#[allow(unused)]
pub fn default_enddevice_config() -> esp_idf_svc::sys::esp_zb_cfg_t {
    esp_idf_svc::sys::esp_zb_cfg_t {
        esp_zb_role: esp_idf_svc::sys::esp_zb_nwk_device_type_t_ESP_ZB_DEVICE_TYPE_ED,
        install_code_policy: false,
        nwk_cfg: esp_idf_svc::sys::esp_zb_cfg_s__bindgen_ty_1 {
            zed_cfg: esp_idf_svc::sys::esp_zb_zed_cfg_t {
                ed_timeout: esp_idf_svc::sys::esp_zb_aging_timeout_t_ESP_ZB_ED_AGING_TIMEOUT_64MIN
                    as u8,
                keep_alive: 3000,
            },
        },
    }
}

#[allow(unused)]
pub fn default_endpoint_config() -> esp_idf_svc::sys::esp_zb_endpoint_config_t {
    esp_idf_svc::sys::esp_zb_endpoint_config_t {
        endpoint: 10,
        app_profile_id: esp_idf_svc::sys::esp_zb_af_profile_id_t_ESP_ZB_AF_SE_PROFILE_ID as u16,
        app_device_id: 0x0000,
        _bitfield_align_1: [],
        _bitfield_1: Default::default(),
    }
}

#[allow(unused)]
pub fn init(mut zb_cfg: esp_idf_svc::sys::esp_zb_cfg_t) -> Result<()> {
    info!("Starting the initialization of Zigbee");

    info!("Starting the initialization of nvs-flash");
    match esp! { unsafe { esp_idf_svc::sys::nvs_flash_init() } } {
        Ok(_) => {
            info!("Successfully initialized nvs-flash")
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not initialize nvs-flash"));
        }
    }

    info!("Creating platform-config");
    let mut zb_platform_config = esp_idf_svc::sys::esp_zb_platform_config_t {
        radio_config: esp_idf_svc::sys::esp_zb_radio_config_t {
            radio_mode: esp_idf_svc::sys::esp_zb_radio_mode_t_ZB_RADIO_MODE_NATIVE,
            radio_uart_config: Default::default(),
        },
        host_config: esp_idf_svc::sys::esp_zb_host_config_t {
            host_connection_mode:
                esp_idf_svc::sys::esp_zb_host_connection_mode_t_ZB_HOST_CONNECTION_MODE_NONE,
            host_uart_config: Default::default(),
        },
    };

    info!("Setting platform-config");
    match esp! { unsafe { esp_idf_svc::sys::esp_zb_platform_config(&mut zb_platform_config as *mut esp_idf_svc::sys::esp_zb_platform_config_t) } }
    {
        Ok(_) => {
            info!("Successfully set platform-config");
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not set platform-config"));
        }
    }

    info!("Calling intern initialization of zigbee");
    unsafe {
        esp_idf_svc::sys::esp_zb_init(&mut zb_cfg as *mut esp_idf_svc::sys::esp_zb_cfg_t);
    }

    Ok(())
}

#[allow(unused)]
pub fn run() -> Result<()> {
    info!("Starting zigbee");
    match esp! { unsafe { esp_idf_svc::sys::esp_zb_start(false)}} {
        Ok(_) => {
            info!("Zigbee started");
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not start zigbee"));
        }
    }

    info!("Starting zigbee main loop");
    unsafe { esp_idf_svc::sys::esp_zb_main_loop_iteration() }

    Ok(())
}

#[allow(unused)]
/// # Safety
/// Using raw pointer for cluster_list
pub unsafe fn add_default_clusters(
    cluster_list: *mut esp_idf_svc::sys::esp_zb_cluster_list_t,
    device_model: String,
) -> Result<()> {
    info!("Adding default clusters to cluster-list");
    let mut basic_cfg = esp_idf_svc::sys::esp_zb_basic_cluster_cfg_t {
        zcl_version: 0x08,
        power_source: 0x00,
    };

    info!("Creating basic-cluster");
    let basic_cluster = unsafe {
        esp_idf_svc::sys::esp_zb_basic_cluster_create(
            &mut basic_cfg as *mut esp_idf_svc::sys::esp_zb_basic_cluster_cfg_t,
        )
    };

    info!("Adding manufacturer attribute to basic-cluster");
    match esp! { unsafe { esp_idf_svc::sys::esp_zb_basic_cluster_add_attr(basic_cluster,
    esp_idf_svc::sys::esp_zb_zcl_basic_attr_t_ESP_ZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID as u16,
    DEVICE_MANUFACTURER.as_ptr() as *mut c_void) }}
    {
        Ok(_) => {
            info!("Manufacturer added as attribute");
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not add manufacturer as attribute"));
        }
    }

    info!("Adding model attribute to basic-cluster");
    let model = CString::new(device_model).unwrap();
    match esp! { unsafe {esp_idf_svc::sys::esp_zb_basic_cluster_add_attr(basic_cluster,
    esp_idf_svc::sys::esp_zb_zcl_basic_attr_t_ESP_ZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID as u16,
    model.as_ptr() as *mut c_void) } }
    {
        Ok(_) => {
            info!("Model added as attribute");
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not add model as attribute"));
        }
    }

    let mut identify_cfg = esp_idf_svc::sys::esp_zb_identify_cluster_cfg_t {
        identify_time: esp_idf_svc::sys::ESP_ZB_ZCL_IDENTIFY_IDENTIFY_TIME_DEFAULT_VALUE as u16,
    };

    info!("Creating and adding identify-cluster");
    match esp! { unsafe { esp_idf_svc::sys::esp_zb_cluster_list_add_identify_cluster(cluster_list, esp_idf_svc::sys::esp_zb_identify_cluster_create(&mut identify_cfg),esp_idf_svc::sys::esp_zb_zcl_cluster_role_t_ESP_ZB_ZCL_CLUSTER_SERVER_ROLE as u8) } }
    {
        Ok(_) => {
            info!("Identify-cluster was added");
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not create or add identify-cluster"));
        }
    }

    Ok(())
}

#[allow(unused)]
fn signal_to_string(signal: u32) -> String {
    match signal {
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_DEFAULT_START => "ZDO_SIGNAL_DEFAULT_START".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_SKIP_STARTUP => "ZDO_SIGNAL_SKIP_STARTUP".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_DEVICE_ANNCE => "ZDO_SIGNAL_DEVICE_ANNCE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_LEAVE => "ZB_ZDO_SIGNAL_LEAVE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_ERROR => "ZB_ZDO_SIGNAL_ERROR".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_DEVICE_FIRST_START => "ZB_BDB_SIGNAL_DEVICE_FIRST_START".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_DEVICE_REBOOT => "ZB_BDB_SIGNAL_DEVICE_REBOOT".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK_NWK_STARTED => "ZB_BDB_SIGNAL_TOUCHLINK_NWK_STARTED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK_NWK_JOINED_ROUTER => "ZB_BDB_SIGNAL_TOUCHLINK_NWK_JOINED_ROUTER ".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK => "ZB_BDB_SIGNAL_TOUCHLINK".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_STEERING => "ZB_BDB_SIGNAL_STEERING".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_FORMATION => "ZB_BDB_SIGNAL_FORMATION".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_FINDING_AND_BINDING_TARGET_FINISHED => "ZB_BDB_SIGNAL_FINDING_AND_BINDING_TARGET_FINISHED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_FINDING_AND_BINDING_INITIATOR_FINISHED => "ZB_BDB_SIGNAL_FINDING_AND_BINDING_INITIATOR_FINISHED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK_TARGET => "ZB_BDB_SIGNAL_TOUCHLINK_TARGET".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK_NWK => "ZB_BDB_SIGNAL_TOUCHLINK_NWK".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK_TARGET_FINISHED => "ZB_BDB_SIGNAL_TOUCHLINK_TARGET_FINISHED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TOUCHLINK_ADD_DEVICE_TO_NWK => "ZB_BDB_SIGNAL_TOUCHLINK_ADD_DEVICE_TO_NWK".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_NWK_SIGNAL_DEVICE_ASSOCIATED => "ZB_NWK_SIGNAL_DEVICE_ASSOCIATED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_LEAVE_INDICATION => "ZB_ZDO_SIGNAL_LEAVE_INDICATION".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_WWAH_REJOIN_STARTED => "ZB_BDB_SIGNAL_WWAH_REJOIN_STARTED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZGP_SIGNAL_COMMISSIONING => "ZB_ZGP_SIGNAL_COMMISSIONING".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_COMMON_SIGNAL_CAN_SLEEP => "ZB_COMMON_SIGNAL_CAN_SLEEP".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_PRODUCTION_CONFIG_READY => "ZB_ZDO_SIGNAL_PRODUCTION_CONFIG_READY".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_NWK_SIGNAL_NO_ACTIVE_LINKS_LEFT => "ZB_NWK_SIGNAL_NO_ACTIVE_LINKS_LEFT".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SKIP_JOIN => "ZB_SE_SIGNAL_SKIP_JOIN".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_REJOIN => "ZB_SE_SIGNAL_REJOIN".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_CHILD_REJOIN => "ZB_SE_SIGNAL_CHILD_REJOIN".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_TC_SIGNAL_CHILD_JOIN_CBKE => "ZB_SE_TC_SIGNAL_CHILD_JOIN_CBKE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_TC_SIGNAL_CHILD_JOIN_NON_CBKE => "ZB_SE_TC_SIGNAL_CHILD_JOIN_NON_CBKE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_CBKE_FAILED => "ZB_SE_SIGNAL_CBKE_FAILED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_CBKE_OK => "ZB_SE_SIGNAL_CBKE_OK".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_START => "ZB_SE_SIGNAL_SERVICE_DISCOVERY_START".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_DO_BIND => "ZB_SE_SIGNAL_SERVICE_DISCOVERY_DO_BIND".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_BIND_OK => "ZB_SE_SIGNAL_SERVICE_DISCOVERY_BIND_OK".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_BIND_FAILED => "ZB_SE_SIGNAL_SERVICE_DISCOVERY_BIND_FAILED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_BIND_INDICATION => "B_SE_SIGNAL_SERVICE_DISCOVERY_BIND_INDICATION".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_OK => "ZB_SE_SIGNAL_SERVICE_DISCOVERY_OK".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_SERVICE_DISCOVERY_FAILED => "ZB_SE_SIGNAL_SERVICE_DISCOVERY_FAILED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_APS_KEY_READY => "ZB_SE_SIGNAL_APS_KEY_READY".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SE_SIGNAL_APS_KEY_FAIL => "ZB_SE_SIGNAL_APS_KEY_FAIL".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SIGNAL_SUBGHZ_SUSPEND => "ZB_SIGNAL_SUBGHZ_SUSPEND".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SIGNAL_SUBGHZ_RESUME => "ZB_SIGNAL_SUBGHZ_RESUME".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_MACSPLIT_DEVICE_BOOT => "ZB_MACSPLIT_DEVICE_BOOT".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_MACSPLIT_DEVICE_READY_FOR_UPGRADE => "ZB_MACSPLIT_DEVICE_READY_FOR_UPGRADE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_MACSPLIT_DEVICE_FW_UPGRADE_EVENT => "ZB_MACSPLIT_DEVICE_FW_UPGRADE_EVENT".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SIGNAL_NWK_INIT_DONE => "ZB_SIGNAL_NWK_INIT_DONE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_DEVICE_AUTHORIZED => "ZB_ZDO_SIGNAL_DEVICE_AUTHORIZED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_SIGNAL_DEVICE_UPDATE => "ZB_ZDO_SIGNAL_DEVICE_UPDATE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_NWK_SIGNAL_PANID_CONFLICT_DETECTED => "ZB_NWK_SIGNAL_PANID_CONFLICT_DETECTED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_NLME_STATUS_INDICATION => "ZB_NLME_STATUS_INDICATION".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_TCSWAP_DB_BACKUP_REQUIRED_SIGNAL => "ZB_TCSWAP_DB_BACKUP_REQUIRED_SIGNAL".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_TC_SWAPPED_SIGNAL => "ZB_TC_SWAPPED_SIGNAL".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_TC_REJOIN_DONE => "ZB_BDB_SIGNAL_TC_REJOIN_DONE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_NWK_SIGNAL_PERMIT_JOIN_STATUS => "ZB_NWK_SIGNAL_PERMIT_JOIN_STATUS".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_STEERING_CANCELLED => "ZB_BDB_SIGNAL_STEERING_CANCELLED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_BDB_SIGNAL_FORMATION_CANCELLED => "ZB_BDB_SIGNAL_FORMATION_CANCELLED".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SIGNAL_READY_TO_SHUT => "ZB_SIGNAL_READY_TO_SHUT".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SIGNAL_INTERPAN_PREINIT => "ZB_SIGNAL_INTERPAN_PREINIT".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZGP_SIGNAL_MODE_CHANGE => "ZB_ZGP_SIGNAL_MODE_CHANGE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZDO_DEVICE_UNAVAILABLE => "ZB_ZDO_DEVICE_UNAVAILABLE".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_ZGP_SIGNAL_APPROVE_COMMISSIONING => "ZB_ZGP_SIGNAL_APPROVE_COMMISSIONING".to_string(),
        esp_idf_svc::sys::esp_zb_app_signal_type_t_ESP_ZB_SIGNAL_END => "ZB_SIGNAL_END".to_string(),
        _ => {
            "unknown".to_string()
        }
    }
}
