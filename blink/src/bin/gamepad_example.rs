#![no_std]
#![no_main]

#[path = "../drivers/mod.rs"]
mod drivers;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use drivers::gamepad::{Gamepad, Button};
use drivers::bsp_ensea::Board;

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
   let p = embassy_stm32::init(Default::default());
   let board = Board::new(p);

   let gamepad = Gamepad::new(board.gamepad);
   info!("start");

   loop {
      let state = gamepad.poll();

      info!(
         "T:{} B:{} L:{} R:{} C:{}",
         state.top,
         state.bottom,
         state.left,
         state.right,
         state.center
      );

      Timer::after(Duration::from_millis(200)).await;
   }
}