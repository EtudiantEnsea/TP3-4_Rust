use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::peripherals::{PA0, PA1, TIM2};
use embassy_stm32::timer::qei::Qei;
use embassy_stm32::Peri;

pub struct Encoder {
   button: Input<'static>,
}

impl Encoder {
   pub fn new(button: embassy_stm32::Peri<'static, AnyPin>) -> Self {
      let button = Input::new(button, Pull::Up);

      let tim2 = embassy_stm32::pac::TIM2;
      tim2.arr().write_value(10_000);
      tim2.cnt().write_value(0);

      Self { button }
   }

   pub fn position(&self) -> i32 {
      let tim2 = embassy_stm32::pac::TIM2;
      tim2.cnt().read() as i32
   }

   pub fn is_pressed(&self) -> bool {
      self.button.is_low()
   }

   pub fn set_position(&mut self, position: i32) {
      let tim2 = embassy_stm32::pac::TIM2;
      tim2.cnt().write_value(position as u32);
   }

   pub fn reset(&mut self) {
      self.set_position(0);
   }
}