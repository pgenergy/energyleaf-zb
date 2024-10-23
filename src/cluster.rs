use esp_idf_sys::ESP_OK;
use std::ffi::c_void;

#[allow(unused)]
#[repr(u16)]
enum ClusterId {
    GasAnalog = 65280,  //0xff00
    ElectricityAnalog,  //65281
    ElectricityDigital, //65282
}

#[allow(unused)]
pub struct ElectricitcyDigitalClusterConfig {
    pub value_total: f64,
    pub value_current: f64,
    pub value_out: f64,
}

#[allow(unused)]
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

    return time_cluster;
}
