#![no_std]
#![no_main]

#[path = "../drivers/mod.rs"]
mod drivers;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use drivers::bargraph::Bargraph;
use drivers::bsp_ensea::Board;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);

    let mut bargraph = Bargraph::<8>::new(board.bargraph.into_array());

    bargraph.set_range(0, 100);

    loop {
        for value in 0..=100 {
            bargraph.set_value(value);
            Timer::after(Duration::from_millis(50)).await;
        }
    }
}