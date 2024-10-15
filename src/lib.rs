mod energyleaf_zb {
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
                    ed_timeout:
                        esp_idf_svc::sys::esp_zb_aging_timeout_t_ESP_ZB_ED_AGING_TIMEOUT_64MIN as u8,
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
    pub fn add_default_clusters(
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
}
