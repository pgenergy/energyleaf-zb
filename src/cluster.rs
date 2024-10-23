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
