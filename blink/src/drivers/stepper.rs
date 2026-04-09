use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri;
use embassy_time::{Timer, Duration};

#[derive(Clone, Copy)]
pub enum Direction {
   Forward,
   Backward,
}

#[derive(Clone, Copy)]
pub enum MicrosteppingMode {
   Full,
   Half,
   Quarter,
   Eighth,
}

pub struct Stepper {
   dir: Output<'static>,
   step: Output<'static>,
   en: Output<'static>,
   ms1: Output<'static>,
   ms2: Output<'static>,
   speed: u32,
}

impl Stepper {
   pub fn new(
      dir: Peri<'static, AnyPin>,
      step: Peri<'static, AnyPin>,
      en: Peri<'static, AnyPin>,
      ms1: Peri<'static, AnyPin>,
      ms2: Peri<'static, AnyPin>,
   ) -> Self {
      Self {
         dir: Output::new(dir, Level::Low, Speed::Low),
         step: Output::new(step, Level::Low, Speed::Low),
         en: Output::new(en, Level::High, Speed::Low), // désactivé par défaut
         ms1: Output::new(ms1, Level::Low, Speed::Low),
         ms2: Output::new(ms2, Level::Low, Speed::Low),
         speed: 0,
      }
   }

   pub fn enable(&mut self) {
      self.en.set_low(); // ENN actif bas
   }

   pub fn disable(&mut self) {
      self.en.set_high();
   }

   pub fn set_direction(&mut self, direction: Direction) {
      match direction {
         Direction::Forward => self.dir.set_high(),
         Direction::Backward => self.dir.set_low(),
      }
   }

   pub fn set_speed(&mut self, speed: u32, direction: Direction) {
      self.speed = speed;
      self.set_direction(direction);
   }

   pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
      match mode {
         MicrosteppingMode::Full => {
               self.ms1.set_low();
               self.ms2.set_low();
         }
         MicrosteppingMode::Half => {
               self.ms1.set_high();
               self.ms2.set_low();
         }
         MicrosteppingMode::Quarter => {
               self.ms1.set_low();
               self.ms2.set_high();
         }
         MicrosteppingMode::Eighth => {
               self.ms1.set_high();
               self.ms2.set_high();
         }
      }
   }

   pub async fn run(&mut self) {
      if self.speed == 0 {
         return;
      }

      let period_us: u64 = 1_000_000 / self.speed as u64;

      loop {
         self.step.set_high();
         Timer::after(Duration::from_micros(period_us / 2)).await;
         self.step.set_low();
         Timer::after(Duration::from_micros(period_us / 2)).await;
      }
   }
}


use core::sync::atomic::{AtomicU32, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

pub static STEPPER_SPEED: AtomicU32 = AtomicU32::new(0);
pub static STEPPER_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();


pub static STEPPER_DIRECTION: AtomicU32 = AtomicU32::new(0);

pub fn set_direction(dir: Direction) {
   let val = match dir {
      Direction::Forward => 0,
      Direction::Backward => 1,
   };
   STEPPER_DIRECTION.store(val, Ordering::Relaxed);
}

pub fn get_direction() -> Direction {
   match STEPPER_DIRECTION.load(Ordering::Relaxed) {
      0 => Direction::Forward,
      _ => Direction::Backward,
   }
}

pub fn update_stepper(speed: u32, direction: Direction) {
    STEPPER_SPEED.store(speed, Ordering::Relaxed);
    set_direction(direction);
    STEPPER_SIGNAL.signal(());
}