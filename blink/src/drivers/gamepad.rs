use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::Peri;

#[derive(Copy, Clone)]
pub enum Button {
   Top,
   Bottom,
   Left,
   Right,
   Center,
}

#[derive(Default, Copy, Clone)]
pub struct GamepadState {
   pub top: bool,
   pub bottom: bool,
   pub left: bool,
   pub right: bool,
   pub center: bool,
}

pub struct Gamepad {
   top: Input<'static>,
   bottom: Input<'static>,
   left: Input<'static>,
   right: Input<'static>,
   center: Input<'static>,
}

impl Gamepad {
   pub fn new(pins: crate::drivers::bsp_ensea::GamepadPins) -> Self {
      Self {
         top: Input::new(pins.top, Pull::Up),
         bottom: Input::new(pins.bottom, Pull::Up),
         left: Input::new(pins.left, Pull::Up),
         right: Input::new(pins.right, Pull::Up),
         center: Input::new(pins.center, Pull::Up),
      }
   }

   pub fn is_pressed(&self, button: Button) -> bool {
      match button {
         Button::Top => self.top.is_low(),
         Button::Bottom => self.bottom.is_low(),
         Button::Left => self.left.is_low(),
         Button::Right => self.right.is_low(),
         Button::Center => self.center.is_low(),
      }
   }

   pub fn poll(&self) -> GamepadState {
      GamepadState {
         top: self.top.is_low(),
         bottom: self.bottom.is_low(),
         left: self.left.is_low(),
         right: self.right.is_low(),
         center: self.center.is_low(),
      }
   }
}