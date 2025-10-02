use hal::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use opt300x::{Opt300x, SlaveAddr};

pub const DEV_ADDR: u8 = 0b100_0100;
pub const CFG_DEFAULT: u16 = 0xC810;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const RESULT: u8 = 0x00;
    pub const CONFIG: u8 = 0x01;
    pub const LOW_LIMIT: u8 = 0x02;
    pub const HIGH_LIMIT: u8 = 0x03;
    pub const MANUFACTURER_ID: u8 = 0x7E;
    pub const DEVICE_ID: u8 = 0x7F;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const CT: u16 = 1 << 11;
    pub const MODE1: u16 = 1 << 10;
    pub const MODE0: u16 = 1 << 9;
    pub const OVF: u16 = 1 << 8;
    pub const CRF: u16 = 1 << 7;
    pub const FH: u16 = 1 << 6;
    pub const FL: u16 = 1 << 5;
    pub const L: u16 = 1 << 4;
    pub const POL: u16 = 1 << 3;
    pub const ME: u16 = 1 << 2;
}

#[allow(unused)]
pub fn new_opt(transactions: &[I2cTrans]) -> Opt300x<I2cMock> {
    Opt300x::new(I2cMock::new(transactions), SlaveAddr::default())
}

pub fn destroy(sensor: Opt300x<I2cMock>) {
    sensor.destroy().done();
}
