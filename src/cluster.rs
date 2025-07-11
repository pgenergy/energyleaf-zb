use anyhow::anyhow;
use esp_idf_sys::esp;
use log::debug;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::ffi::c_void;

#[allow(unused)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClusterId {
    GasAnalog = 65280,  //0xff00
    ElectricityAnalog,  //65281
    ElectricityDigital, //65282
}

#[allow(unused)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ElectricityDigitalClusterAttributes {
    ReadingIn,
    ReadingOut,
    PowerCurrent,
}

pub fn create_electricity_digital_cluster(
) -> anyhow::Result<*mut esp_idf_svc::sys::esp_zb_attribute_list_t> {
    let digital_electricity_cluster = unsafe {
        esp_idf_svc::sys::esp_zb_zcl_attr_list_create(ClusterId::ElectricityDigital as u16)
    };

    let tmp_value: f64 = 0f64;

    match esp! { unsafe { esp_idf_svc::sys::esp_zb_custom_cluster_add_custom_attr(
        digital_electricity_cluster,
        ElectricityDigitalClusterAttributes::ReadingIn as u16,
        esp_idf_svc::sys::esp_zb_zcl_attr_type_t_ESP_ZB_ZCL_ATTR_TYPE_DOUBLE as u8,
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_READ_ONLY as u8 |
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_REPORTING as u8,
        &tmp_value as *const f64 as *mut c_void,
    ) } } {
        Ok(_) => {
            debug!("Attribute ReadingIn added to digital electricity cluster")
        }
        Err(_) => Err(anyhow!(
            "Could not add attribute ReadingIn added to digital electricity cluster"
        ))?,
    }

    match esp! { unsafe { esp_idf_svc::sys::esp_zb_custom_cluster_add_custom_attr(
        digital_electricity_cluster,
        ElectricityDigitalClusterAttributes::ReadingOut as u16,
        esp_idf_svc::sys::esp_zb_zcl_attr_type_t_ESP_ZB_ZCL_ATTR_TYPE_DOUBLE as u8,
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_READ_ONLY as u8 |
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_REPORTING as u8,
        &tmp_value as *const f64 as *mut c_void,
    ) } } {
        Ok(_) => {
            debug!("Attribute ReadingOut added to digital electricity cluster")
        }
        Err(_) => Err(anyhow!(
            "Could not add attribute ReadingOut added to digital electricity cluster"
        ))?,
    }

    match esp! { unsafe { esp_idf_svc::sys::esp_zb_custom_cluster_add_custom_attr(
        digital_electricity_cluster,
        ElectricityDigitalClusterAttributes::PowerCurrent as u16,
        esp_idf_svc::sys::esp_zb_zcl_attr_type_t_ESP_ZB_ZCL_ATTR_TYPE_DOUBLE as u8,
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_READ_ONLY as u8 |
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_REPORTING as u8,
        &tmp_value as *const f64 as *mut c_void,
    ) } } {
        Ok(_) => {
            debug!("Attribute PowerCurrent added to digital electricity cluster")
        }
        Err(_) => Err(anyhow!(
            "Could not add attribute PowerCurrent added to digital electricity cluster"
        ))?,
    }

    Ok(digital_electricity_cluster)
}

#[allow(unused)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ElectricityAnalogClusterAttributes {
    Rotation,
}

pub fn create_electricity_analog_cluster(
) -> anyhow::Result<*mut esp_idf_svc::sys::esp_zb_attribute_list_t> {
    let analog_electricity_cluster = unsafe {
        esp_idf_svc::sys::esp_zb_zcl_attr_list_create(ClusterId::ElectricityAnalog as u16)
    };

    match esp! { unsafe { esp_idf_svc::sys::esp_zb_custom_cluster_add_custom_attr(
        analog_electricity_cluster,
        ElectricityAnalogClusterAttributes::Rotation as u16,
        esp_idf_svc::sys::esp_zb_zcl_attr_type_t_ESP_ZB_ZCL_ATTR_TYPE_NULL as u8,
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_READ_ONLY as u8 |
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_REPORTING as u8,
        std::ptr::null_mut(),
    ) } } {
        Ok(_) => {
            debug!("Attribute Rotation added to analog electricity cluster")
        }
        Err(_) => Err(anyhow!(
            "Could not add Rotation ReadingIn added to analog electricity cluster"
        ))?,
    }

    Ok(analog_electricity_cluster)
}

#[allow(unused)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum GasAnalogClusterAttributed {
    Rotation,
}

pub fn create_gas_analog_cluster() -> anyhow::Result<*mut esp_idf_svc::sys::esp_zb_attribute_list_t>
{
    let analog_gas_cluster =
        unsafe { esp_idf_svc::sys::esp_zb_zcl_attr_list_create(ClusterId::GasAnalog as u16) };

    match esp! { unsafe { esp_idf_svc::sys::esp_zb_custom_cluster_add_custom_attr(
        analog_gas_cluster,
        GasAnalogClusterAttributed::Rotation as u16,
        esp_idf_svc::sys::esp_zb_zcl_attr_type_t_ESP_ZB_ZCL_ATTR_TYPE_NULL as u8,
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_READ_ONLY as u8 |
        esp_idf_svc::sys::esp_zb_zcl_attr_access_t_ESP_ZB_ZCL_ATTR_ACCESS_REPORTING as u8,
        std::ptr::null_mut(),
    ) } } {
        Ok(_) => {
            debug!("Attribute Rotation added to analog gas cluster")
        }
        Err(_) => Err(anyhow!(
            "Could not add Rotation ReadingIn added to analog gas cluster"
        ))?,
    }

    Ok(analog_gas_cluster)
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn set_and_report_attribute_value(
    cluster: u16,
    attribute: u16,
    role: u8,
    check: bool,
    data: *mut c_void,
) -> anyhow::Result<()> {
    match esp! { unsafe { esp_idf_svc::sys::esp_zb_zcl_set_attribute_val(
        crate::default_endpoint_config().endpoint,
        cluster,
        role,
        attribute,
        data,
        check,
    ) } } {
        Ok(_) => {
            debug!("Set attribute {} for {}", attribute, cluster)
        }
        Err(_) => Err(anyhow!(
            "Could not set attribute {} for {}",
            attribute,
            cluster
        ))?,
    }

    let mut report = esp_idf_svc::sys::esp_zb_zcl_report_attr_cmd_t {
        zcl_basic_cmd: esp_idf_svc::sys::esp_zb_zcl_basic_cmd_t {
            dst_addr_u: esp_idf_svc::sys::esp_zb_addr_u { addr_short: 0x0000 }, //Coordinator
            dst_endpoint: crate::default_endpoint_config().endpoint,
            src_endpoint: crate::default_endpoint_config().endpoint,
        },
        address_mode:
            esp_idf_svc::sys::esp_zb_aps_address_mode_t_ESP_ZB_APS_ADDR_MODE_16_ENDP_PRESENT,
        clusterID: cluster,
        cluster_role: role,
        attributeID: attribute,
    };

    match esp! { unsafe { esp_idf_svc::sys::esp_zb_zcl_report_attr_cmd_req(&mut report) } } {
        Ok(_) => {
            debug!(
                "Set and Reported value (Cluster: {}; Attribute: {})",
                cluster, attribute
            );
            Ok(())
        }
        Err(_) => Err(anyhow!(
            "Could not set or report value (Cluster: {}; Attribute: {})",
            cluster,
            attribute
        ))?,
    }
}
