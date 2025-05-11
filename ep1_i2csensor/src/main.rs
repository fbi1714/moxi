#![no_std]
#![no_main]

mod sense;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use sense::sense_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());
    spawner.must_spawn(sense_task(p.TWISPI0, p.P1_00, p.P0_26));
}
