#![no_std]
#![no_main]

#[path = "../drivers/mod.rs"]
mod drivers;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use drivers::stepper::{Stepper, Direction, MicrosteppingMode};
use drivers::bsp_ensea::Board;

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);

    let mut stepper = Stepper::new(
        board.stepper.dir,
        board.stepper.step,
        board.stepper.en,
        board.stepper.ms1,
        board.stepper.ms2,
    );

    stepper.enable();
    stepper.set_microstepping(MicrosteppingMode::Full);
    stepper.set_speed(200, Direction::Forward);

    info!("stepper start");

    stepper.run().await;
}