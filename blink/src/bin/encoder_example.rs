#![no_std]
#![no_main]

#[path = "../drivers/mod.rs"]
mod drivers;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use drivers::encoder::Encoder;
use drivers::bsp_ensea::Board;

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
   let p = embassy_stm32::init(Default::default());
   let board = Board::new(p);

   let mut encoder = Encoder::new(board.encoder.button);

   info!("encoder start");

   loop {
      let pos = encoder.position();
      let pressed = encoder.is_pressed();

      info!("pos: {} btn: {}", pos, pressed);

      Timer::after(Duration::from_millis(200)).await;
   }
}

