#![no_std]
#![no_main]

use core::cmp::max_by_key;

use defmt::info;
use drv2625::Drv2625;
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, gpio::{Input, Level, Output, OutputDrive}, peripherals, twim::{self, Twim}, twis::{self, Twis}};
use embassy_time::Timer;
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};


bind_interrupts!(struct Irqs {
    SERIAL20 => twim::InterruptHandler<peripherals::SERIAL20>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    
    let mut config = twim::Config::default();
    static RAM_BUFFER: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
    let mut twi = Twim::new(p.SERIAL20, Irqs, p.P1_07, p.P1_08, config, RAM_BUFFER.take());


    let mut drv = Drv2625::new_i2c(twi, 0x5A);
    info!("Connected");

    let _ = drv.auto_calibrate(
        drv2625::ActuatorType::LRA, 
        27,
        37,
        14
    );

    let mut int = Output::new(p.P1_01, Level::Low, OutputDrive::Standard);

    loop {
        int.set_high();
        Timer::after_millis(10).await;
        int.set_low();
        Timer::after_millis(500).await;
    }
}