use embassy_stm32::Peri;
use embassy_stm32::gpio::AnyPin;
use embassy_stm32::Peripherals;
use embassy_stm32::peripherals::PA0;
use embassy_stm32::peripherals::PA1;

pub struct Board {
   pub bargraph: BargraphPins,
   pub gps: GpsPins,
   pub gpio: GpioPins,
   pub gamepad: GamepadPins,
   pub magneto: MagnetoPins,
   pub encoder: EncoderPins,
   pub stepper: StepperPins,
   pub uart1: Uart1Pins,
   pub uart2: Uart2Pins,
   pub spi2: Spi2Pins,
   pub i2c1: I2c1Pins,
   pub connector: ConnectorPins,
}

pub struct BargraphPins {
   pub led0: Peri<'static, AnyPin>, // PC7
   pub led1: Peri<'static, AnyPin>, // PB2
   pub led2: Peri<'static, AnyPin>, // PA8
   pub led3: Peri<'static, AnyPin>, // PB1
   pub led4: Peri<'static, AnyPin>, // PB15
   pub led5: Peri<'static, AnyPin>, // PB4
   pub led6: Peri<'static, AnyPin>, // PB14
   pub led7: Peri<'static, AnyPin>, // PB5
}


pub struct GpsPins {
   pub enable: Peri<'static, AnyPin>, // PB13
}


pub struct GpioPins {
   pub user_led: Peri<'static, AnyPin>, //PA5
   pub user_button: Peri<'static, AnyPin>, //PC13
}


pub struct GamepadPins {
   pub top: Peri<'static, AnyPin>, // PC8
   pub bottom: Peri<'static, AnyPin>, // PB11
   pub right: Peri<'static, AnyPin>, // PC9
   pub left: Peri<'static, AnyPin>, // PC6
   pub center: Peri<'static, AnyPin>, // PC5
}

pub struct MagnetoPins {
   pub status: Peri<'static, AnyPin>, // PC1
   pub interrupt: Peri<'static, AnyPin>, // PB0
}

pub struct EncoderPins {
   pub button: Peri<'static, AnyPin>, // PA15
   pub a: Peri<'static, PA0>,      // PA0
   pub b: Peri<'static, PA1>,      // PA1
}


pub struct StepperPins {
   pub dir: Peri<'static, AnyPin>,   // PA7
   pub step: Peri<'static, AnyPin>,  // PA6
   pub en: Peri<'static, AnyPin>,    // PA12
   pub ms1: Peri<'static, AnyPin>,   // PA11
   pub ms2: Peri<'static, AnyPin>,   // PB12
}


pub struct Uart1Pins {
   pub tx: Peri<'static, AnyPin>, // PA9
   pub rx: Peri<'static, AnyPin>, // PA10
}


pub struct Uart2Pins {
   pub tx: Peri<'static, AnyPin>, // PA2
   pub rx: Peri<'static, AnyPin>, // PA3
}


pub struct Spi2Pins {
   pub sck: Peri<'static, AnyPin>,  // PB10
   pub mosi: Peri<'static, AnyPin>, // PC3
   pub miso: Peri<'static, AnyPin>, // PC2
   pub cs: Peri<'static, AnyPin>,   // PC0
}


pub struct I2c1Pins {
   pub scl: Peri<'static, AnyPin>, // PB6
   pub sda: Peri<'static, AnyPin>, // PB7
}


pub struct ConnectorPins {
   pub pc10: Peri<'static, AnyPin>,
   pub pc11: Peri<'static, AnyPin>,
   pub pc12: Peri<'static, AnyPin>,
   pub pb8: Peri<'static, AnyPin>,
   pub pb9: Peri<'static, AnyPin>,
   pub pd2: Peri<'static, AnyPin>,
}


impl Board {
   pub fn new(p: Peripherals) -> Self {
      Self {
         bargraph: BargraphPins {
               led0: p.PC7.into(),
               led1: p.PB2.into(),
               led2: p.PA8.into(),
               led3: p.PB1.into(),
               led4: p.PB15.into(),
               led5: p.PB4.into(),
               led6: p.PB14.into(),
               led7: p.PB5.into(),
         },

         gps: GpsPins {
               enable: p.PB13.into(),
         },

         gpio: GpioPins {
               user_led: p.PA5.into(),
               user_button: p.PC13.into(),
         },

         gamepad: GamepadPins {
               top: p.PC8.into(),
               bottom: p.PB11.into(),
               right: p.PC9.into(),
               left: p.PC6.into(),
               center: p.PC5.into(),
         },

         magneto: MagnetoPins {
               status: p.PC1.into(),
               interrupt: p.PB0.into(),
         },

         encoder: EncoderPins {
               button: p.PA15.into(),
               a: p.PA0.into(),
               b: p.PA1.into(),
         },

         stepper: StepperPins {
               dir: p.PA7.into(),
               step: p.PA6.into(),
               en: p.PA12.into(),
               ms1: p.PA11.into(),
               ms2: p.PB12.into(),
         },

         uart1: Uart1Pins {
               tx: p.PA9.into(),
               rx: p.PA10.into(),
         },

         uart2: Uart2Pins {
               tx: p.PA2.into(),
               rx: p.PA3.into(),
         },

         spi2: Spi2Pins {
               sck: p.PB10.into(),
               mosi: p.PC3.into(),
               miso: p.PC2.into(),
               cs: p.PC0.into(),
         },

         i2c1: I2c1Pins {
               scl: p.PB6.into(),
               sda: p.PB7.into(),
         },

         connector: ConnectorPins {
               pc10: p.PC10.into(),
               pc11: p.PC11.into(),
               pc12: p.PC12.into(),
               pb8: p.PB8.into(),
               pb9: p.PB9.into(),
               pd2: p.PD2.into(),
         },
      }
   }
}

impl BargraphPins {
   pub fn into_array(self) -> [Peri<'static, AnyPin>; 8] {
      [
         self.led0,
         self.led1,
         self.led2,
         self.led3,
         self.led4,
         self.led5,
         self.led6,
         self.led7,
      ]
   }
}