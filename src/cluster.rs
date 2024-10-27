use esp_idf_sys::{esp, ESP_OK};
use log::info;
use std::ffi::c_void;

#[allow(unused)]
#[repr(u16)]
pub enum ClusterId {
    GasAnalog = 65280,  //0xff00
    ElectricityAnalog,  //65281
    ElectricityDigital, //65282
}

pub enum ClusterCommandIdElectricityAnalog {
    EvtNewRotation,
}

#[allow(unused)]
pub struct ElectricitcyDigitalClusterConfig {
    pub value_total: f64,
    pub value_current: f64,
    pub value_out: f64,
}

#[allow(unused)]
/// # Safety
///
/// Using raw pointer for cluster_list
pub unsafe fn add_ota_cluster(
    cluster_list: *mut esp_idf_svc::sys::esp_zb_cluster_list_t,
) -> anyhow::Result<()> {
    info!("Adding ota cluster to cluster-list");

    info!("Creating ota cluster");
    let ota_cluster = unsafe {
        esp_idf_svc::sys::esp_zb_zcl_attr_list_create(
            esp_idf_svc::sys::esp_zb_zcl_cluster_id_t_ESP_ZB_ZCL_CLUSTER_ID_OTA_UPGRADE as u16,
        )
    };

    let mut ota_variable = esp_idf_svc::sys::esp_zb_zcl_ota_upgrade_server_variable_t {
        query_jitter: 0x64,
        current_time: 0x12345, //hmm, based on documentation the zcl time cluster is not supported
        file_count: 0,
    };

    let ota_variable_ptr: *mut c_void = &mut ota_variable
        as *mut esp_idf_svc::sys::esp_zb_zcl_ota_upgrade_server_variable_t
        as *mut c_void;

    match esp! { unsafe {
            esp_idf_svc::sys::esp_zb_ota_cluster_add_attr(
                ota_cluster,
                esp_idf_svc::sys::esp_zb_zcl_ota_upgrade_attr_t_ESP_ZB_ZCL_ATTR_OTA_UPGRADE_SERVER_DATA_ID as u16,
                ota_variable_ptr
            )
        }
    } {
        Ok(_) => {
            info!("Variables for ota cluster added as attributes");
        }
        Err(_) => {
            return Err(anyhow::anyhow!(
                "Could not add variables for ota cluster as attribute"
            ));
        }
    }

    match esp! { unsafe {
            esp_idf_svc::sys::esp_zb_cluster_list_add_ota_cluster(
                cluster_list,
                ota_cluster,
                esp_idf_svc::sys::esp_zb_zcl_cluster_role_t_ESP_ZB_ZCL_CLUSTER_SERVER_ROLE as u8
            )
        }
    } {
        Ok(_) => {
            info!("Added ota cluster to cluster-list");
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Could not add ota cluster"));
        }
    }

    Ok(())
}

#[allow(unused)]
// Currently i think it is not needed, because why did the sensors itself need time information?
pub fn create_time_cluster_attribute() -> *mut esp_idf_svc::sys::esp_zb_attribute_list_t {
    let mut time_default: u32 = 0;
    let time_default_ptr: *mut c_void = &mut time_default as *mut u32 as *mut c_void;
    let mut time_status_default: u8 = 0;
    let time_status_default_ptr: *mut c_void = &mut time_status_default as *mut u8 as *mut c_void;

    let time_cluster = unsafe {
        esp_idf_svc::sys::esp_zb_zcl_attr_list_create(
            esp_idf_svc::sys::esp_zb_zcl_cluster_id_t_ESP_ZB_ZCL_CLUSTER_ID_TIME as u16,
        )
    };
    if unsafe {
        esp_idf_svc::sys::esp_zb_time_cluster_add_attr(
            time_cluster,
            esp_idf_svc::sys::esp_zb_zcl_time_attr_t_ESP_ZB_ZCL_ATTR_TIME_TIME_ID as u16,
            time_default_ptr,
        )
    } != ESP_OK
    {
        panic!("Could not add time_cluster")
    }
    if unsafe {
        esp_idf_svc::sys::esp_zb_time_cluster_add_attr(
            time_cluster,
            esp_idf_svc::sys::esp_zb_zcl_time_attr_t_ESP_ZB_ZCL_ATTR_TIME_TIME_STATUS_ID as u16,
            time_status_default_ptr,
        )
    } != ESP_OK
    {
        panic!("Could not add time_cluster")
    }

    time_cluster
}
