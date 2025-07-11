cmake_minimum_required(VERSION 3.16)

idf_component_register(
    INCLUDE_DIRS include
)

if(CONFIG_ZB_ENABLED)

    set(ESP_ZIGBEE_API_LIBS "")

    if(CONFIG_ZB_ZCZR)
        if(CONFIG_ZB_DEBUG_MODE)
            add_prebuilt_library(esp_zigbee_api_lib "${CMAKE_CURRENT_SOURCE_DIR}/lib/esp32c6/libesp_zb_api_zczr.debug.a" REQUIRES esp-zboss-lib)
        else()
            add_prebuilt_library(esp_zigbee_api_lib "${CMAKE_CURRENT_SOURCE_DIR}/lib/esp32c6/libesp_zb_api_zczr.a" REQUIRES esp-zboss-lib)
        endif()
    elseif(CONFIG_ZB_ZED)
        if(CONFIG_ZB_DEBUG_MODE)
            add_prebuilt_library(esp_zigbee_api_lib "${CMAKE_CURRENT_SOURCE_DIR}/lib/esp32c6/libesp_zb_api_ed.debug.a" REQUIRES esp-zboss-lib)
        else()
            add_prebuilt_library(esp_zigbee_api_lib "${CMAKE_CURRENT_SOURCE_DIR}/lib/esp32c6/libesp_zb_api_ed.a" REQUIRES esp-zboss-lib)
        endif()
    elseif(CONFIG_ZB_ZGPD)
        add_prebuilt_library(esp_zigbee_api_lib "${CMAKE_CURRENT_SOURCE_DIR}/lib/esp32c6/libesp_zb_api_gpd.a" REQUIRES esp-zboss-lib)
    endif()

    list(APPEND ESP_ZIGBEE_API_LIBS esp_zigbee_api_lib)

    target_link_libraries(${COMPONENT_LIB} INTERFACE ${ESP_ZIGBEE_API_LIBS})
    target_compile_options(${COMPONENT_LIB} INTERFACE $<$<COMPILE_LANGUAGE:C>:-Wno-strict-prototypes>)
endif()
