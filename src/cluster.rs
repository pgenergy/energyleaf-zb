#[allow(unused)]
#[repr(u16)]
enum ClusterId {
    GasAnalog = 65280,  //0xff00
    ElectricityAnalog,  //65281
    ElectricityDigital, //65282
}
