
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{gpio, i2c::{self, Config}};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use embedded_hal_1::i2c::I2c;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_16, Level::Low);

    let sda = p.PIN_20;
    let scl = p.PIN_21;
    let mut i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());

    let vl53l1x_addr = 0b0101001;  
    let new_addr = 0b0101000;
    let chip_addr_reg_addr = 0x0001_u16.to_be_bytes();
    let mut buf = [0; 2];

    let res = i2c.write_read(vl53l1x_addr, &chip_addr_reg_addr, &mut buf);
    info!("res: {:?}: {:x}", res, buf);
    let res = i2c.write(vl53l1x_addr, &chip_addr_reg_addr);
    info!("res: {:?}: {:x}", res, buf);
    let res = i2c.write(vl53l1x_addr, &[new_addr]);
    info!("res: {:?}: {:x}", res, buf);
    let res = i2c.write_read(new_addr, &chip_addr_reg_addr, &mut buf);
    info!("res: {:?}: {:x}", res, buf);
    let res = i2c.write_read(vl53l1x_addr, &chip_addr_reg_addr, &mut buf);
    info!("res: {:?}: {:x}", res, buf);

    /*loop {
        info!("led on");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;
        info!("led off");
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }*/
}

