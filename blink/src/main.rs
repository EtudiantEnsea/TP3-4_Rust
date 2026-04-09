#![no_std]
#![no_main]

mod drivers;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use drivers::bargraph::{Bargraph, BARGRAPH_SIGNAL, BARGRAPH_LEVEL};
use drivers::encoder::Encoder;
use drivers::stepper::{Stepper, Direction, set_direction, get_direction, update_stepper, STEPPER_SIGNAL, STEPPER_SPEED};
use drivers::bsp_ensea::Board;
use core::sync::atomic::Ordering;

use defmt::*;
use crate::drivers::gamepad::Button;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: Bargraph<8>) {
    bargraph.wait_and_update().await;
}

#[embassy_executor::task]
async fn encoder_task(mut encoder: Encoder) {
    loop {
        let pos = encoder.position();

        Bargraph::<8>::update_value(pos as u32);

        let speed = (pos.abs() as u32) * 10; // simple mapping
        let direction = if pos >= 0 {
            Direction::Forward
        } else {
            Direction::Backward
        };

        update_stepper(speed, direction);

        Timer::after(Duration::from_millis(200)).await;
    }
}

#[embassy_executor::task]
async fn stepper_update_task(mut stepper: Stepper) {
    loop {
        STEPPER_SIGNAL.wait().await;

        let speed = STEPPER_SPEED.load(Ordering::Relaxed);
        let direction = get_direction();

        stepper.set_speed(speed, direction);

        stepper.run().await;
    }
}

#[embassy_executor::task]
async fn emergency_stop_task(button: embassy_stm32::gpio::Input<'static>) {
    loop {
        if button.is_low() {
            let tim2 = embassy_stm32::pac::TIM2;
            tim2.cr1().modify(|w| w.set_cen(false));
            tim2.cnt().write_value(0);
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);

    let bargraph = Bargraph::<8>::new(board.bargraph.into_array());

    // encoder garde SON bouton (pas de conflit)
    let encoder = Encoder::new(board.encoder.button);

    let stepper = Stepper::new(
        board.stepper.dir,
        board.stepper.step,
        board.stepper.en,
        board.stepper.ms1,
        board.stepper.ms2,
    );

    let emergency_button = embassy_stm32::gpio::Input::new(board.gamepad.center, embassy_stm32::gpio::Pull::Up);

    spawner.spawn(bargraph_task(bargraph)).unwrap();
    spawner.spawn(encoder_task(encoder)).unwrap();
    spawner.spawn(stepper_update_task(stepper)).unwrap();
    spawner.spawn(emergency_stop_task(emergency_button)).unwrap();

    loop {
        info!("main alive");
        Timer::after(Duration::from_secs(1)).await;
    }
}