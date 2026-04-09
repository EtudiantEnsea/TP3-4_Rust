use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::gpio::AnyPin;
use embassy_stm32::Peri;

pub struct Bargraph<const N: usize> {
   leds: [Output<'static>; N],
   min: u32,
   max: u32,
}

impl<const N: usize> Bargraph<N> {
   pub fn new(pins: [Peri<'static, AnyPin>; N]) -> Self {
      let leds = pins.map(|p| Output::new(p, Level::Low, Speed::Low));

      Self {
         leds,
         min: 0,
         max: 100,
      }
   }

   pub fn set_range(&mut self, min: u32, max: u32) {
      self.min = min;
      self.max = max;
   }

   pub fn set_value(&mut self, value: u32) {
      let value = value.clamp(self.min, self.max);

      let range = self.max.saturating_sub(self.min);
      if range == 0 {
         return;
      }

      let relative = value - self.min;

      let leds_to_light =
         (relative.saturating_mul(N as u32)) / range;

      for (i, led) in self.leds.iter_mut().enumerate() {
         if (i as u32) < leds_to_light {
            led.set_high();
         } else {
            led.set_low();
         }
      }
   }

}

// drivers/bargraph.rs (ajouts)

use core::sync::atomic::{AtomicU32, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

pub static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
pub static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

impl<const N: usize> Bargraph<N> {
    pub async fn wait_and_update(&mut self) {
        loop {
            BARGRAPH_SIGNAL.wait().await;

            let value = BARGRAPH_LEVEL.load(Ordering::Relaxed);
            self.set_value(value);
        }
    }

    pub fn update_value(value: u32) {
        BARGRAPH_LEVEL.store(value, Ordering::Relaxed);
        BARGRAPH_SIGNAL.signal(());
    }
}