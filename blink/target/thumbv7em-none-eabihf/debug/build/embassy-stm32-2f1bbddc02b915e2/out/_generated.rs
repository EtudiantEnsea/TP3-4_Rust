embassy_hal_internal::peripherals_definition!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD2,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    COMP1,
    COMP2,
    CRC,
    DAC1,
    DBGMCU,
    DMA1,
    DMA2,
    FLASH,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    LCD,
    LPTIM1,
    LPTIM2,
    LPUART1,
    OPAMP1,
    OPAMP2,
    PWR,
    QUADSPI,
    MCO,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SDMMC1,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TIM1,
    TIM15,
    TIM16,
    TIM17,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TSC,
    UART4,
    UART5,
    UID,
    USART1,
    USART2,
    USART3,
    USB_OTG_FS,
    VREFBUF,
    VREFINTCAL,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::peripherals_struct!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD2,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    COMP1,
    COMP2,
    CRC,
    DAC1,
    DBGMCU,
    DMA1,
    DMA2,
    FLASH,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    LCD,
    LPTIM1,
    LPTIM2,
    LPUART1,
    OPAMP1,
    OPAMP2,
    PWR,
    QUADSPI,
    MCO,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SDMMC1,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TIM1,
    TIM15,
    TIM16,
    TIM17,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TSC,
    UART4,
    UART5,
    UID,
    USART1,
    USART2,
    USART3,
    USB_OTG_FS,
    VREFBUF,
    VREFINTCAL,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD_PVM,
    TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_CHANNEL1,
    DMA1_CHANNEL2,
    DMA1_CHANNEL3,
    DMA1_CHANNEL4,
    DMA1_CHANNEL5,
    DMA1_CHANNEL6,
    DMA1_CHANNEL7,
    ADC1_2,
    CAN1_TX,
    CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK_TIM15,
    TIM1_UP_TIM16,
    TIM1_TRG_COM_TIM17,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    DFSDM1_FLT3,
    TIM8_BRK,
    TIM8_UP,
    TIM8_TRG_COM,
    TIM8_CC,
    ADC3,
    FMC,
    SDMMC1,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_CHANNEL1,
    DMA2_CHANNEL2,
    DMA2_CHANNEL3,
    DMA2_CHANNEL4,
    DMA2_CHANNEL5,
    DFSDM1_FLT0,
    DFSDM1_FLT1,
    DFSDM1_FLT2,
    COMP,
    LPTIM1,
    LPTIM2,
    OTG_FS,
    DMA2_CHANNEL6,
    DMA2_CHANNEL7,
    LPUART1,
    QUADSPI,
    I2C3_EV,
    I2C3_ER,
    SAI1,
    SAI2,
    SWPMI1,
    TSC,
    LCD,
    RNG,
    FPU,
);
#[cfg(feature = "rt")]
#[interrupt]
fn TIM2() {
    crate::time_driver::get_driver().on_interrupt();
}
pub const MAX_ERASE_SIZE: usize = 2048u32 as usize;
pub mod flash_regions {
    pub const BANK1_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134217728u32,
        size: 524288u32,
        erase_size: 2048u32,
        write_size: 8u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK2_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank2,
        base: 134742016u32,
        size: 524288u32,
        erase_size: 2048u32,
        write_size: 8u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank2Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region: Bank1Region<'d, MODE>,
        pub bank2_region: Bank2Region<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region: Bank1Region(
                    &BANK1_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank2_region: Bank2Region(
                    &BANK2_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 2usize] = [&BANK1_REGION, &BANK2_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pll1_q")
            },
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (19u8, 13u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pll1_q")
            },
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (19u8, 13u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "pll1_q")
            },
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (19u8, 13u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 25u8)),
            (22u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 12u8)),
            (18u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 29u8)),
            (22u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 0u8)),
            (18u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 1u8)),
            (18u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FLASH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 8u8)),
            (18u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FLASH {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c1sel() {
            crate::pac::rcc::vals::I2c1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
            },
            crate::pac::rcc::vals::I2c1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "sys")
            },
            crate::pac::rcc::vals::I2c1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 21u8)),
            (22u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c2sel() {
            crate::pac::rcc::vals::I2c2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
            },
            crate::pac::rcc::vals::I2c2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "sys")
            },
            crate::pac::rcc::vals::I2c2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 22u8)),
            (22u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c3sel() {
            crate::pac::rcc::vals::I2c3sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
            },
            crate::pac::rcc::vals::I2c3sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "sys")
            },
            crate::pac::rcc::vals::I2c3sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 23u8)),
            (22u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LCD {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LCD" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LCD" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 9u8)),
            (22u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LCD {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lptim1sel() {
            crate::pac::rcc::vals::Lptim1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim1sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lsi")
            },
            crate::pac::rcc::vals::Lptim1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "hsi")
            },
            crate::pac::rcc::vals::Lptim1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 31u8)),
            (22u8, 31u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lptim2sel() {
            crate::pac::rcc::vals::Lptim2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim2sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lsi")
            },
            crate::pac::rcc::vals::Lptim2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "hsi")
            },
            crate::pac::rcc::vals::Lptim2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((15u8, 5u8)),
            (23u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPUART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lpuart1sel() {
            crate::pac::rcc::vals::Lpuart1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk1")
            },
            crate::pac::rcc::vals::Lpuart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "sys")
            },
            crate::pac::rcc::vals::Lpuart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "hsi")
            },
            crate::pac::rcc::vals::Lpuart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPUART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((15u8, 0u8)),
            (23u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPUART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::OPAMP1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 30u8)),
            (22u8, 30u8),
            Some(1u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::OPAMP1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::OPAMP2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 30u8)),
            (22u8, 30u8),
            Some(1u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::OPAMP2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 28u8)),
            (22u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::QUADSPI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 8u8)),
            (20u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::QUADSPI {}
impl crate::rcc::SealedRccPeripheral for peripherals::RNG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "hclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 18u8)),
            (19u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RNG {}
impl crate::rcc::SealedRccPeripheral for peripherals::RTC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.bdcr().read().rtcsel() {
            crate::pac::rcc::vals::Rtcsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lse")
            },
            crate::pac::rcc::vals::Rtcsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lsi")
            },
            crate::pac::rcc::vals::Rtcsel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "RTC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (22u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Standby,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RTC {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().sai1sel() {
            crate::pac::rcc::vals::Sai1sel::PLLSAI1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pllsai1_p")
            },
            crate::pac::rcc::vals::Sai1sel::PLLSAI2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pllsai2_p")
            },
            crate::pac::rcc::vals::Sai1sel::PLL1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pll1_p")
            },
            crate::pac::rcc::vals::Sai1sel::SAI1_EXTCLK => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sai1_extclk . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "sai1_extclk")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 21u8)),
            (24u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().sai2sel() {
            crate::pac::rcc::vals::Sai2sel::PLLSAI1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pllsai1_p")
            },
            crate::pac::rcc::vals::Sai2sel::PLLSAI2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pllsai2_p")
            },
            crate::pac::rcc::vals::Sai2sel::PLL1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pll1_p")
            },
            crate::pac::rcc::vals::Sai2sel::SAI2_EXTCLK => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sai2_extclk . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "sai2_extclk")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 22u8)),
            (24u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDMMC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "hsi48")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pllsai1_q")
            },
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pll1_q")
            },
            crate::pac::rcc::vals::Clk48sel::MSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . msi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "msi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDMMC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 10u8)),
            (24u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDMMC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 12u8)),
            (24u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 14u8)),
            (22u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 15u8)),
            (22u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 0u8)),
            (24u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 11u8)),
            (24u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM15 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 16u8)),
            (24u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM15 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM16 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 17u8)),
            (24u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM16 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM17 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 18u8)),
            (24u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM17 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 0u8)),
            (22u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 1u8)),
            (22u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 2u8)),
            (22u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 3u8)),
            (22u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 4u8)),
            (22u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM7 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 5u8)),
            (22u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM8 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 13u8)),
            (24u8, 13u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TSC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TSC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TSC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 16u8)),
            (18u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TSC {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().uart4sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART4"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 19u8)),
            (22u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART5 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().uart5sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART5"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 20u8)),
            (22u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart1sel() {
            crate::pac::rcc::vals::Usart1sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
            },
            crate::pac::rcc::vals::Usart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "sys")
            },
            crate::pac::rcc::vals::Usart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "hsi")
            },
            crate::pac::rcc::vals::Usart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 14u8)),
            (24u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart2sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 17u8)),
            (22u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart3sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 18u8)),
            (22u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_FS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hsi48")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pllsai1_q")
            },
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pll1_q")
            },
            crate::pac::rcc::vals::Clk48sel::MSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . msi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "msi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_FS"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 12u8)),
            (19u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_FS {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (22u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 2usize] = [0u8, 0u8];
pub mod mux {
    pub use crate::pac::rcc::vals::Adcsel;
    pub use crate::pac::rcc::vals::Clk48sel;
    pub use crate::pac::rcc::vals::I2c1sel;
    pub use crate::pac::rcc::vals::I2c2sel;
    pub use crate::pac::rcc::vals::I2c3sel;
    pub use crate::pac::rcc::vals::Lptim1sel;
    pub use crate::pac::rcc::vals::Lptim2sel;
    pub use crate::pac::rcc::vals::Lpuart1sel;
    pub use crate::pac::rcc::vals::Rtcsel;
    pub use crate::pac::rcc::vals::Sai1sel;
    pub use crate::pac::rcc::vals::Sai2sel;
    pub use crate::pac::rcc::vals::Usart1sel;
    pub use crate::pac::rcc::vals::Usartsel;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub rtcsel: Rtcsel,
        pub adcsel: Adcsel,
        pub clk48sel: Clk48sel,
        pub i2c1sel: I2c1sel,
        pub i2c2sel: I2c2sel,
        pub i2c3sel: I2c3sel,
        pub lptim1sel: Lptim1sel,
        pub lptim2sel: Lptim2sel,
        pub lpuart1sel: Lpuart1sel,
        pub sai1sel: Sai1sel,
        pub sai2sel: Sai2sel,
        pub uart4sel: Usartsel,
        pub uart5sel: Usartsel,
        pub usart1sel: Usart1sel,
        pub usart2sel: Usartsel,
        pub usart3sel: Usartsel,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.bdcr().modify(|w| {
                w.set_rtcsel(self.rtcsel);
            });
            crate::pac::RCC.ccipr().modify(|w| {
                w.set_adcsel(self.adcsel);
                w.set_clk48sel(self.clk48sel);
                w.set_i2c1sel(self.i2c1sel);
                w.set_i2c2sel(self.i2c2sel);
                w.set_i2c3sel(self.i2c3sel);
                w.set_lptim1sel(self.lptim1sel);
                w.set_lptim2sel(self.lptim2sel);
                w.set_lpuart1sel(self.lpuart1sel);
                w.set_sai1sel(self.sai1sel);
                w.set_sai2sel(self.sai2sel);
                w.set_uart4sel(self.uart4sel);
                w.set_uart5sel(self.uart5sel);
                w.set_usart1sel(self.usart1sel);
                w.set_usart2sel(self.usart2sel);
                w.set_usart3sel(self.usart3sel);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub hclk1: crate::time::MaybeHertz,
    pub hclk2: crate::time::MaybeHertz,
    pub hclk3: crate::time::MaybeHertz,
    pub hse: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub hsi48: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub lsi: crate::time::MaybeHertz,
    pub msi: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub pll1_p: crate::time::MaybeHertz,
    pub pll1_q: crate::time::MaybeHertz,
    pub pllsai1_p: crate::time::MaybeHertz,
    pub pllsai1_q: crate::time::MaybeHertz,
    pub pllsai2_p: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sai1_extclk: crate::time::MaybeHertz,
    pub sai2_extclk: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_dma() {}
pub unsafe fn init_bdma() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma2en(true));
}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioeen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiofen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiogen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiohen(true));
    crate::pac::GPIOA.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOB.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOC.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOD.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOE.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOF.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOG.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
    crate::pac::GPIOH.moder().modify(|w| {
        w.set_moder(0, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(1, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(2, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(3, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(4, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(5, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(6, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(7, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(8, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(9, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(10, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(11, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(12, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(13, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(14, crate::pac::gpio::vals::Moder::ANALOG);
        w.set_moder(15, crate::pac::gpio::vals::Moder::ANALOG);
    });
}
impl_adc_pin!(ADC1, PA0, 5u8);
impl_adc_pin!(ADC1, PA1, 6u8);
impl_adc_pin!(ADC1, PA2, 7u8);
impl_adc_pin!(ADC1, PA3, 8u8);
impl_adc_pin!(ADC1, PA4, 9u8);
impl_adc_pin!(ADC1, PA5, 10u8);
impl_adc_pin!(ADC1, PA6, 11u8);
impl_adc_pin!(ADC1, PA7, 12u8);
impl_adc_pin!(ADC1, PB0, 15u8);
impl_adc_pin!(ADC1, PB1, 16u8);
impl_adc_pin!(ADC1, PC0, 1u8);
impl_adc_pin!(ADC1, PC1, 2u8);
impl_adc_pin!(ADC1, PC2, 3u8);
impl_adc_pin!(ADC1, PC3, 4u8);
impl_adc_pin!(ADC1, PC4, 13u8);
impl_adc_pin!(ADC1, PC5, 14u8);
impl_adc_pin!(ADC2, PA0, 5u8);
impl_adc_pin!(ADC2, PA1, 6u8);
impl_adc_pin!(ADC2, PA2, 7u8);
impl_adc_pin!(ADC2, PA3, 8u8);
impl_adc_pin!(ADC2, PA4, 9u8);
impl_adc_pin!(ADC2, PA5, 10u8);
impl_adc_pin!(ADC2, PA6, 11u8);
impl_adc_pin!(ADC2, PA7, 12u8);
impl_adc_pin!(ADC2, PB0, 15u8);
impl_adc_pin!(ADC2, PB1, 16u8);
impl_adc_pin!(ADC2, PC0, 1u8);
impl_adc_pin!(ADC2, PC1, 2u8);
impl_adc_pin!(ADC2, PC2, 3u8);
impl_adc_pin!(ADC2, PC3, 4u8);
impl_adc_pin!(ADC2, PC4, 13u8);
impl_adc_pin!(ADC2, PC5, 14u8);
impl_adc_pin!(ADC3, PC0, 1u8);
impl_adc_pin!(ADC3, PC1, 2u8);
impl_adc_pin!(ADC3, PC2, 3u8);
impl_adc_pin!(ADC3, PC3, 4u8);
pin_trait_impl!(
    crate::can::RxPin,
    CAN1,
    PA11,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN1,
    PA12,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::RxPin,
    CAN1,
    PB8,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN1,
    PB9,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::dac::DacPin<Ch1>, DAC1, PA4, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch2>, DAC1, PA5, 0u8);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB6,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB7,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB8,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB9,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C2,
    PB10,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PB11,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C2,
    PB13,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PB14,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C3,
    PC0,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C3,
    PC1,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA1, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PA10, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA15, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA2, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA3, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA6, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA7, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PA8, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PA9, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB0, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB1, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB10, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB11, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB12, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB13, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB14, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB15, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB3, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB4, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB5, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB7, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB8, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PB9, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC0, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC1, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PC10, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC10, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PC11, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC11, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PC12, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC12, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC2, 11u8);
pin_trait_impl!(crate::lcd::VlcdPin, LCD, PC3, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC4, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC5, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC6, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC7, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC8, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC9, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PD2, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PD2, 11u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PB2, 1u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PC1, 1u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PA4, 14u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PA8, 14u8);
pin_trait_impl!(
    crate::usart::RxPin,
    LPUART1,
    PB10,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    LPUART1,
    PB11,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    LPUART1,
    PB12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    LPUART1,
    PB12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    LPUART1,
    PB13,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    LPUART1,
    PC0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    LPUART1,
    PC1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
impl_opamp_vp_pin!(OPAMP1, PA0, 0u8);
impl_opamp_vn_pin!(OPAMP1, PA1, 0u8);
impl_opamp_vout_pin!(OPAMP1, PA3);
impl_opamp_vp_pin!(OPAMP2, PA6, 0u8);
impl_opamp_vn_pin!(OPAMP2, PA7, 0u8);
impl_opamp_vout_pin!(OPAMP2, PB0);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PA6, 10u8);
pin_trait_impl!(crate::qspi::BK1D2Pin, QUADSPI, PA7, 10u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PB0, 10u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PB1, 10u8);
pin_trait_impl!(crate::qspi::SckPin, QUADSPI, PB10, 10u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB11, 10u8);
pin_trait_impl!(crate::rcc::McoPin, MCO, PA8, 0u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI1, PA4, 13u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI1, PB10, 13u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI1, PB3, 13u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI1, PB4, 13u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI1, PB5, 13u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI1, PB6, 13u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI1, PB8, 13u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI1, PB9, 13u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PC3, 13u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PA15, 13u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI2, PB12, 13u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI2, PB13, 13u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI2, PB14, 13u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI2, PB15, 13u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI2, PC10, 13u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PC11, 13u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PC12, 13u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI2, PC6, 13u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PC7, 13u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDMMC1, PB8, 12u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDMMC1, PB9, 12u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC1, PC10, 12u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC1, PC11, 12u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC1, PC12, 12u8);
pin_trait_impl!(crate::sdmmc::D6Pin, SDMMC1, PC6, 12u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDMMC1, PC7, 12u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC1, PC8, 12u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC1, PC9, 12u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC1, PD2, 12u8);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PA5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PA6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PA7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PB3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PB4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PB5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB10,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PB14,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PB15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PC2,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PC3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA15,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PB3,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PB4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB5,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PC10,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PC11,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PC12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM1,
    PA10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM1,
    PA11,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn2>,
    TIM1,
    PA11,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM1,
    PA11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM1,
    PA12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PA6,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PA6,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PA7,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM1,
    PA8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM1,
    PA9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PB12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PB12,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PB13,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB14,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM15,
    PA1,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM15,
    PA2,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM15,
    PA3,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM15,
    PA9,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM15,
    PB12,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM15,
    PB13,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM15,
    PB14,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM15,
    PB15,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM16,
    PA6,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM16,
    PB5,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM16,
    PB6,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM16,
    PB8,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM17,
    PA10,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM17,
    PA7,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM17,
    PB4,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM17,
    PB7,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM17,
    PB9,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA0,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PA1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA15,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PA2,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PA3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA5,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PB10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PB11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PB3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PA6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PA7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PB0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PB1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PB4,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PB5,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PC6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PC7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PC8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PC9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM3,
    PD2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM4,
    PB6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM4,
    PB7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM4,
    PB8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM4,
    PB9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM5,
    PA0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM5,
    PA1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM5,
    PA2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM5,
    PA3,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM8,
    PA0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA5,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM8,
    PA6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM8,
    PA6,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB1,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB14,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB15,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM8,
    PB6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn2>,
    TIM8,
    PB6,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM8,
    PB7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn1>,
    TIM8,
    PB7,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM8,
    PC6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM8,
    PC7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM8,
    PC8,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM8,
    PC9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn2>,
    TIM8,
    PC9,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM8,
    PC9,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::tsc::G3IO1Pin, TSC, PA15, 9u8);
pin_trait_impl!(crate::tsc::G1IO1Pin, TSC, PB12, 9u8);
pin_trait_impl!(crate::tsc::G1IO2Pin, TSC, PB13, 9u8);
pin_trait_impl!(crate::tsc::G1IO3Pin, TSC, PB14, 9u8);
pin_trait_impl!(crate::tsc::G1IO4Pin, TSC, PB15, 9u8);
pin_trait_impl!(crate::tsc::G2IO1Pin, TSC, PB4, 9u8);
pin_trait_impl!(crate::tsc::G2IO2Pin, TSC, PB5, 9u8);
pin_trait_impl!(crate::tsc::G2IO3Pin, TSC, PB6, 9u8);
pin_trait_impl!(crate::tsc::G2IO4Pin, TSC, PB7, 9u8);
pin_trait_impl!(crate::tsc::G3IO2Pin, TSC, PC10, 9u8);
pin_trait_impl!(crate::tsc::G3IO3Pin, TSC, PC11, 9u8);
pin_trait_impl!(crate::tsc::G3IO4Pin, TSC, PC12, 9u8);
pin_trait_impl!(crate::tsc::G4IO1Pin, TSC, PC6, 9u8);
pin_trait_impl!(crate::tsc::G4IO2Pin, TSC, PC7, 9u8);
pin_trait_impl!(crate::tsc::G4IO3Pin, TSC, PC8, 9u8);
pin_trait_impl!(crate::tsc::G4IO4Pin, TSC, PC9, 9u8);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PA0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PA1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART4,
    PB7,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PC10,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PC11,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART5,
    PB4,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART5,
    PB4,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART5,
    PB5,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART5,
    PC12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART5,
    PD2,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PA10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART1,
    PA11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART1,
    PA8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PA9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART1,
    PB3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART1,
    PB3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART1,
    PB4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART1,
    PB5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PB6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PB7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART2,
    PA0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART2,
    PA2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART2,
    PA3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART2,
    PA4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PA6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PB0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PB1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PB1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PB10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PB11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PB12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PB13,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PC10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PC12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PC4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PD2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PD2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_FS, PA11, 10u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_FS, PA12, 10u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH1, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH3, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH2, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH4, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH3, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH5, 0u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH3, 6u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH4, 5u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH4, 3u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH5, 3u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH7, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH6, 5u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH7, 5u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH5, 3u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH2, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH3, 3u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH6, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH7, 4u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA1_CH5, 5u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA2_CH7, 3u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH1, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH2, 1u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH6, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH7, 1u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH6, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH7, 1u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH3, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH4, 1u8, {});
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC1, DMA2_CH4, 7u8, {});
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC1, DMA2_CH5, 7u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH2, 1u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH3, 1u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH3, 4u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH4, 4u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH4, 1u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH5, 1u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH1, 3u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH2, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH2, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH3, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH4, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH6, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH7, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH5, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH5, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH3, 4u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH3, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH6, 4u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH6, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH1, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH1, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH7, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH7, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH1, 4u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH2, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH5, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH7, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH7, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH2, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH3, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH3, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH6, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH1, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH4, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH5, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH7, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH1, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH2, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH2, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH4, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH5, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH3, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH4, 3u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH4, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH5, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH1, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH1, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH2, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH6, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH7, 7u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH3, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH5, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH1, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH2, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH4, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH5, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH6, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH7, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH6, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH7, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH2, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH3, 2u8, {});
impl core::ops::Div<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 1u32 / 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 512u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Mcopre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Mcopre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Mcopre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Mcopre::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Mcopre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Mcopre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Mcopre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllm> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllm) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllm::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllm::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllm::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllm::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllm::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllm::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllm::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllm> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllm) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllm::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Plln> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Plln) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Plln::MUL8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Plln::MUL9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Plln::MUL10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Plln::MUL11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Plln::MUL12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Plln::MUL13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Plln::MUL14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Plln::MUL15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Plln::MUL16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Plln::MUL17 => self * 1u32 / 17u32,
            crate::pac::rcc::vals::Plln::MUL18 => self * 1u32 / 18u32,
            crate::pac::rcc::vals::Plln::MUL19 => self * 1u32 / 19u32,
            crate::pac::rcc::vals::Plln::MUL20 => self * 1u32 / 20u32,
            crate::pac::rcc::vals::Plln::MUL21 => self * 1u32 / 21u32,
            crate::pac::rcc::vals::Plln::MUL22 => self * 1u32 / 22u32,
            crate::pac::rcc::vals::Plln::MUL23 => self * 1u32 / 23u32,
            crate::pac::rcc::vals::Plln::MUL24 => self * 1u32 / 24u32,
            crate::pac::rcc::vals::Plln::MUL25 => self * 1u32 / 25u32,
            crate::pac::rcc::vals::Plln::MUL26 => self * 1u32 / 26u32,
            crate::pac::rcc::vals::Plln::MUL27 => self * 1u32 / 27u32,
            crate::pac::rcc::vals::Plln::MUL28 => self * 1u32 / 28u32,
            crate::pac::rcc::vals::Plln::MUL29 => self * 1u32 / 29u32,
            crate::pac::rcc::vals::Plln::MUL30 => self * 1u32 / 30u32,
            crate::pac::rcc::vals::Plln::MUL31 => self * 1u32 / 31u32,
            crate::pac::rcc::vals::Plln::MUL32 => self * 1u32 / 32u32,
            crate::pac::rcc::vals::Plln::MUL33 => self * 1u32 / 33u32,
            crate::pac::rcc::vals::Plln::MUL34 => self * 1u32 / 34u32,
            crate::pac::rcc::vals::Plln::MUL35 => self * 1u32 / 35u32,
            crate::pac::rcc::vals::Plln::MUL36 => self * 1u32 / 36u32,
            crate::pac::rcc::vals::Plln::MUL37 => self * 1u32 / 37u32,
            crate::pac::rcc::vals::Plln::MUL38 => self * 1u32 / 38u32,
            crate::pac::rcc::vals::Plln::MUL39 => self * 1u32 / 39u32,
            crate::pac::rcc::vals::Plln::MUL40 => self * 1u32 / 40u32,
            crate::pac::rcc::vals::Plln::MUL41 => self * 1u32 / 41u32,
            crate::pac::rcc::vals::Plln::MUL42 => self * 1u32 / 42u32,
            crate::pac::rcc::vals::Plln::MUL43 => self * 1u32 / 43u32,
            crate::pac::rcc::vals::Plln::MUL44 => self * 1u32 / 44u32,
            crate::pac::rcc::vals::Plln::MUL45 => self * 1u32 / 45u32,
            crate::pac::rcc::vals::Plln::MUL46 => self * 1u32 / 46u32,
            crate::pac::rcc::vals::Plln::MUL47 => self * 1u32 / 47u32,
            crate::pac::rcc::vals::Plln::MUL48 => self * 1u32 / 48u32,
            crate::pac::rcc::vals::Plln::MUL49 => self * 1u32 / 49u32,
            crate::pac::rcc::vals::Plln::MUL50 => self * 1u32 / 50u32,
            crate::pac::rcc::vals::Plln::MUL51 => self * 1u32 / 51u32,
            crate::pac::rcc::vals::Plln::MUL52 => self * 1u32 / 52u32,
            crate::pac::rcc::vals::Plln::MUL53 => self * 1u32 / 53u32,
            crate::pac::rcc::vals::Plln::MUL54 => self * 1u32 / 54u32,
            crate::pac::rcc::vals::Plln::MUL55 => self * 1u32 / 55u32,
            crate::pac::rcc::vals::Plln::MUL56 => self * 1u32 / 56u32,
            crate::pac::rcc::vals::Plln::MUL57 => self * 1u32 / 57u32,
            crate::pac::rcc::vals::Plln::MUL58 => self * 1u32 / 58u32,
            crate::pac::rcc::vals::Plln::MUL59 => self * 1u32 / 59u32,
            crate::pac::rcc::vals::Plln::MUL60 => self * 1u32 / 60u32,
            crate::pac::rcc::vals::Plln::MUL61 => self * 1u32 / 61u32,
            crate::pac::rcc::vals::Plln::MUL62 => self * 1u32 / 62u32,
            crate::pac::rcc::vals::Plln::MUL63 => self * 1u32 / 63u32,
            crate::pac::rcc::vals::Plln::MUL64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Plln::MUL65 => self * 1u32 / 65u32,
            crate::pac::rcc::vals::Plln::MUL66 => self * 1u32 / 66u32,
            crate::pac::rcc::vals::Plln::MUL67 => self * 1u32 / 67u32,
            crate::pac::rcc::vals::Plln::MUL68 => self * 1u32 / 68u32,
            crate::pac::rcc::vals::Plln::MUL69 => self * 1u32 / 69u32,
            crate::pac::rcc::vals::Plln::MUL70 => self * 1u32 / 70u32,
            crate::pac::rcc::vals::Plln::MUL71 => self * 1u32 / 71u32,
            crate::pac::rcc::vals::Plln::MUL72 => self * 1u32 / 72u32,
            crate::pac::rcc::vals::Plln::MUL73 => self * 1u32 / 73u32,
            crate::pac::rcc::vals::Plln::MUL74 => self * 1u32 / 74u32,
            crate::pac::rcc::vals::Plln::MUL75 => self * 1u32 / 75u32,
            crate::pac::rcc::vals::Plln::MUL76 => self * 1u32 / 76u32,
            crate::pac::rcc::vals::Plln::MUL77 => self * 1u32 / 77u32,
            crate::pac::rcc::vals::Plln::MUL78 => self * 1u32 / 78u32,
            crate::pac::rcc::vals::Plln::MUL79 => self * 1u32 / 79u32,
            crate::pac::rcc::vals::Plln::MUL80 => self * 1u32 / 80u32,
            crate::pac::rcc::vals::Plln::MUL81 => self * 1u32 / 81u32,
            crate::pac::rcc::vals::Plln::MUL82 => self * 1u32 / 82u32,
            crate::pac::rcc::vals::Plln::MUL83 => self * 1u32 / 83u32,
            crate::pac::rcc::vals::Plln::MUL84 => self * 1u32 / 84u32,
            crate::pac::rcc::vals::Plln::MUL85 => self * 1u32 / 85u32,
            crate::pac::rcc::vals::Plln::MUL86 => self * 1u32 / 86u32,
            crate::pac::rcc::vals::Plln::MUL87 => self * 1u32 / 87u32,
            crate::pac::rcc::vals::Plln::MUL88 => self * 1u32 / 88u32,
            crate::pac::rcc::vals::Plln::MUL89 => self * 1u32 / 89u32,
            crate::pac::rcc::vals::Plln::MUL90 => self * 1u32 / 90u32,
            crate::pac::rcc::vals::Plln::MUL91 => self * 1u32 / 91u32,
            crate::pac::rcc::vals::Plln::MUL92 => self * 1u32 / 92u32,
            crate::pac::rcc::vals::Plln::MUL93 => self * 1u32 / 93u32,
            crate::pac::rcc::vals::Plln::MUL94 => self * 1u32 / 94u32,
            crate::pac::rcc::vals::Plln::MUL95 => self * 1u32 / 95u32,
            crate::pac::rcc::vals::Plln::MUL96 => self * 1u32 / 96u32,
            crate::pac::rcc::vals::Plln::MUL97 => self * 1u32 / 97u32,
            crate::pac::rcc::vals::Plln::MUL98 => self * 1u32 / 98u32,
            crate::pac::rcc::vals::Plln::MUL99 => self * 1u32 / 99u32,
            crate::pac::rcc::vals::Plln::MUL100 => self * 1u32 / 100u32,
            crate::pac::rcc::vals::Plln::MUL101 => self * 1u32 / 101u32,
            crate::pac::rcc::vals::Plln::MUL102 => self * 1u32 / 102u32,
            crate::pac::rcc::vals::Plln::MUL103 => self * 1u32 / 103u32,
            crate::pac::rcc::vals::Plln::MUL104 => self * 1u32 / 104u32,
            crate::pac::rcc::vals::Plln::MUL105 => self * 1u32 / 105u32,
            crate::pac::rcc::vals::Plln::MUL106 => self * 1u32 / 106u32,
            crate::pac::rcc::vals::Plln::MUL107 => self * 1u32 / 107u32,
            crate::pac::rcc::vals::Plln::MUL108 => self * 1u32 / 108u32,
            crate::pac::rcc::vals::Plln::MUL109 => self * 1u32 / 109u32,
            crate::pac::rcc::vals::Plln::MUL110 => self * 1u32 / 110u32,
            crate::pac::rcc::vals::Plln::MUL111 => self * 1u32 / 111u32,
            crate::pac::rcc::vals::Plln::MUL112 => self * 1u32 / 112u32,
            crate::pac::rcc::vals::Plln::MUL113 => self * 1u32 / 113u32,
            crate::pac::rcc::vals::Plln::MUL114 => self * 1u32 / 114u32,
            crate::pac::rcc::vals::Plln::MUL115 => self * 1u32 / 115u32,
            crate::pac::rcc::vals::Plln::MUL116 => self * 1u32 / 116u32,
            crate::pac::rcc::vals::Plln::MUL117 => self * 1u32 / 117u32,
            crate::pac::rcc::vals::Plln::MUL118 => self * 1u32 / 118u32,
            crate::pac::rcc::vals::Plln::MUL119 => self * 1u32 / 119u32,
            crate::pac::rcc::vals::Plln::MUL120 => self * 1u32 / 120u32,
            crate::pac::rcc::vals::Plln::MUL121 => self * 1u32 / 121u32,
            crate::pac::rcc::vals::Plln::MUL122 => self * 1u32 / 122u32,
            crate::pac::rcc::vals::Plln::MUL123 => self * 1u32 / 123u32,
            crate::pac::rcc::vals::Plln::MUL124 => self * 1u32 / 124u32,
            crate::pac::rcc::vals::Plln::MUL125 => self * 1u32 / 125u32,
            crate::pac::rcc::vals::Plln::MUL126 => self * 1u32 / 126u32,
            crate::pac::rcc::vals::Plln::MUL127 => self * 1u32 / 127u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Plln> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Plln) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Plln::MUL8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL17 => self * 17u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL18 => self * 18u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL19 => self * 19u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL20 => self * 20u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL21 => self * 21u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL22 => self * 22u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL23 => self * 23u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL24 => self * 24u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL25 => self * 25u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL26 => self * 26u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL27 => self * 27u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL28 => self * 28u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL29 => self * 29u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL30 => self * 30u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL31 => self * 31u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL32 => self * 32u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL33 => self * 33u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL34 => self * 34u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL35 => self * 35u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL36 => self * 36u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL37 => self * 37u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL38 => self * 38u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL39 => self * 39u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL40 => self * 40u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL41 => self * 41u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL42 => self * 42u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL43 => self * 43u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL44 => self * 44u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL45 => self * 45u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL46 => self * 46u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL47 => self * 47u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL48 => self * 48u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL49 => self * 49u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL50 => self * 50u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL51 => self * 51u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL52 => self * 52u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL53 => self * 53u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL54 => self * 54u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL55 => self * 55u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL56 => self * 56u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL57 => self * 57u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL58 => self * 58u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL59 => self * 59u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL60 => self * 60u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL61 => self * 61u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL62 => self * 62u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL63 => self * 63u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL65 => self * 65u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL66 => self * 66u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL67 => self * 67u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL68 => self * 68u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL69 => self * 69u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL70 => self * 70u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL71 => self * 71u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL72 => self * 72u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL73 => self * 73u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL74 => self * 74u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL75 => self * 75u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL76 => self * 76u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL77 => self * 77u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL78 => self * 78u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL79 => self * 79u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL80 => self * 80u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL81 => self * 81u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL82 => self * 82u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL83 => self * 83u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL84 => self * 84u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL85 => self * 85u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL86 => self * 86u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL87 => self * 87u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL88 => self * 88u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL89 => self * 89u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL90 => self * 90u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL91 => self * 91u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL92 => self * 92u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL93 => self * 93u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL94 => self * 94u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL95 => self * 95u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL96 => self * 96u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL97 => self * 97u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL98 => self * 98u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL99 => self * 99u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL100 => self * 100u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL101 => self * 101u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL102 => self * 102u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL103 => self * 103u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL104 => self * 104u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL105 => self * 105u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL106 => self * 106u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL107 => self * 107u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL108 => self * 108u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL109 => self * 109u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL110 => self * 110u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL111 => self * 111u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL112 => self * 112u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL113 => self * 113u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL114 => self * 114u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL115 => self * 115u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL116 => self * 116u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL117 => self * 117u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL118 => self * 118u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL119 => self * 119u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL120 => self * 120u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL121 => self * 121u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL122 => self * 122u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL123 => self * 123u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL124 => self * 124u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL125 => self * 125u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL126 => self * 126u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL127 => self * 127u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllp> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllp) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllp::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllp::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllp::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllp::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllp::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllp::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllp::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Pllp::DIV9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Pllp::DIV10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Pllp::DIV11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Pllp::DIV12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Pllp::DIV13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Pllp::DIV14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Pllp::DIV15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Pllp::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Pllp::DIV17 => self * 1u32 / 17u32,
            crate::pac::rcc::vals::Pllp::DIV18 => self * 1u32 / 18u32,
            crate::pac::rcc::vals::Pllp::DIV19 => self * 1u32 / 19u32,
            crate::pac::rcc::vals::Pllp::DIV20 => self * 1u32 / 20u32,
            crate::pac::rcc::vals::Pllp::DIV21 => self * 1u32 / 21u32,
            crate::pac::rcc::vals::Pllp::DIV22 => self * 1u32 / 22u32,
            crate::pac::rcc::vals::Pllp::DIV23 => self * 1u32 / 23u32,
            crate::pac::rcc::vals::Pllp::DIV24 => self * 1u32 / 24u32,
            crate::pac::rcc::vals::Pllp::DIV25 => self * 1u32 / 25u32,
            crate::pac::rcc::vals::Pllp::DIV26 => self * 1u32 / 26u32,
            crate::pac::rcc::vals::Pllp::DIV27 => self * 1u32 / 27u32,
            crate::pac::rcc::vals::Pllp::DIV28 => self * 1u32 / 28u32,
            crate::pac::rcc::vals::Pllp::DIV29 => self * 1u32 / 29u32,
            crate::pac::rcc::vals::Pllp::DIV30 => self * 1u32 / 30u32,
            crate::pac::rcc::vals::Pllp::DIV31 => self * 1u32 / 31u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllp> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllp) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllp::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV17 => self * 17u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV18 => self * 18u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV19 => self * 19u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV20 => self * 20u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV21 => self * 21u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV22 => self * 22u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV23 => self * 23u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV24 => self * 24u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV25 => self * 25u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV26 => self * 26u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV27 => self * 27u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV28 => self * 28u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV29 => self * 29u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV30 => self * 30u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV31 => self * 31u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllq> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllq) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllq::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllq::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllq::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllq::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllq> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllq) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllq::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllr> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllr) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllr::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllr::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllr::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllr::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllr> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllr) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllr::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_2;
    }
    pub mod ADC123_COMMON {}
    pub mod ADC2 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_2;
    }
    pub mod ADC3 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC3;
    }
    pub mod CAN1 {
        pub type RX0 = crate::interrupt::typelevel::CAN1_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN1_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN1_SCE;
        pub type TX = crate::interrupt::typelevel::CAN1_TX;
    }
    pub mod COMP1 {
        pub type WKUP = crate::interrupt::typelevel::COMP;
    }
    pub mod COMP2 {
        pub type WKUP = crate::interrupt::typelevel::COMP;
    }
    pub mod CRC {}
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod DBGMCU {}
    pub mod DFSDM1 {
        pub type FLT0 = crate::interrupt::typelevel::DFSDM1_FLT0;
        pub type FLT1 = crate::interrupt::typelevel::DFSDM1_FLT1;
        pub type FLT2 = crate::interrupt::typelevel::DFSDM1_FLT2;
        pub type FLT3 = crate::interrupt::typelevel::DFSDM1_FLT3;
    }
    pub mod DMA1 {
        pub type CH1 = crate::interrupt::typelevel::DMA1_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_CHANNEL2;
        pub type CH3 = crate::interrupt::typelevel::DMA1_CHANNEL3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_CHANNEL4;
        pub type CH5 = crate::interrupt::typelevel::DMA1_CHANNEL5;
        pub type CH6 = crate::interrupt::typelevel::DMA1_CHANNEL6;
        pub type CH7 = crate::interrupt::typelevel::DMA1_CHANNEL7;
    }
    pub mod DMA2 {
        pub type CH1 = crate::interrupt::typelevel::DMA2_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA2_CHANNEL2;
        pub type CH3 = crate::interrupt::typelevel::DMA2_CHANNEL3;
        pub type CH4 = crate::interrupt::typelevel::DMA2_CHANNEL4;
        pub type CH5 = crate::interrupt::typelevel::DMA2_CHANNEL5;
        pub type CH6 = crate::interrupt::typelevel::DMA2_CHANNEL6;
        pub type CH7 = crate::interrupt::typelevel::DMA2_CHANNEL7;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI9_5;
    }
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod GPIOF {}
    pub mod GPIOG {}
    pub mod GPIOH {}
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1_ER;
        pub type EV = crate::interrupt::typelevel::I2C1_EV;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_ER;
        pub type EV = crate::interrupt::typelevel::I2C2_EV;
    }
    pub mod I2C3 {
        pub type ER = crate::interrupt::typelevel::I2C3_ER;
        pub type EV = crate::interrupt::typelevel::I2C3_EV;
    }
    pub mod IWDG {}
    pub mod LCD {
        pub type GLOBAL = crate::interrupt::typelevel::LCD;
    }
    pub mod LPTIM1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM1;
    }
    pub mod LPTIM2 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM2;
    }
    pub mod LPUART1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPUART1;
    }
    pub mod OPAMP1 {}
    pub mod OPAMP2 {}
    pub mod PWR {}
    pub mod QUADSPI {
        pub type GLOBAL = crate::interrupt::typelevel::QUADSPI;
    }
    pub mod RCC {
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RNG {
        pub type GLOBAL = crate::interrupt::typelevel::RNG;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC_ALARM;
        pub type STAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type TAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type WKUP = crate::interrupt::typelevel::RTC_WKUP;
    }
    pub mod SAI1 {
        pub type A = crate::interrupt::typelevel::SAI1;
        pub type B = crate::interrupt::typelevel::SAI1;
    }
    pub mod SAI2 {
        pub type A = crate::interrupt::typelevel::SAI2;
        pub type B = crate::interrupt::typelevel::SAI2;
    }
    pub mod SDMMC1 {
        pub type GLOBAL = crate::interrupt::typelevel::SDMMC1;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod SPI3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI3;
    }
    pub mod SWPMI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SWPMI1;
    }
    pub mod SYSCFG {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM16;
    }
    pub mod TIM15 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type CC = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type COM = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type TRG = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type UP = crate::interrupt::typelevel::TIM1_BRK_TIM15;
    }
    pub mod TIM16 {
        pub type BRK = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type CC = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type COM = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type TRG = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM16;
    }
    pub mod TIM17 {
        pub type BRK = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type CC = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type UP = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM4;
        pub type CC = crate::interrupt::typelevel::TIM4;
        pub type COM = crate::interrupt::typelevel::TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM4;
        pub type UP = crate::interrupt::typelevel::TIM4;
    }
    pub mod TIM5 {
        pub type BRK = crate::interrupt::typelevel::TIM5;
        pub type CC = crate::interrupt::typelevel::TIM5;
        pub type COM = crate::interrupt::typelevel::TIM5;
        pub type TRG = crate::interrupt::typelevel::TIM5;
        pub type UP = crate::interrupt::typelevel::TIM5;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod TIM7 {
        pub type BRK = crate::interrupt::typelevel::TIM7;
        pub type CC = crate::interrupt::typelevel::TIM7;
        pub type COM = crate::interrupt::typelevel::TIM7;
        pub type TRG = crate::interrupt::typelevel::TIM7;
        pub type UP = crate::interrupt::typelevel::TIM7;
    }
    pub mod TIM8 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK;
        pub type CC = crate::interrupt::typelevel::TIM8_CC;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM;
        pub type UP = crate::interrupt::typelevel::TIM8_UP;
    }
    pub mod TSC {
        pub type GLOBAL = crate::interrupt::typelevel::TSC;
    }
    pub mod UART4 {
        pub type GLOBAL = crate::interrupt::typelevel::UART4;
    }
    pub mod UART5 {
        pub type GLOBAL = crate::interrupt::typelevel::UART5;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3;
    }
    pub mod USB_OTG_FS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_FS;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_FS;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_FS;
        pub type WKUP = crate::interrupt::typelevel::OTG_FS;
    }
    pub mod VREFBUF {}
    pub mod VREFINTCAL {}
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH1, 0u8, Stop1);
dma_channel_impl!(DMA1_CH2, 1u8, Stop1);
dma_channel_impl!(DMA1_CH3, 2u8, Stop1);
dma_channel_impl!(DMA1_CH4, 3u8, Stop1);
dma_channel_impl!(DMA1_CH5, 4u8, Stop1);
dma_channel_impl!(DMA1_CH6, 5u8, Stop1);
dma_channel_impl!(DMA1_CH7, 6u8, Stop1);
dma_channel_impl!(DMA2_CH1, 7u8, Stop1);
dma_channel_impl!(DMA2_CH2, 8u8, Stop1);
dma_channel_impl!(DMA2_CH3, 9u8, Stop1);
dma_channel_impl!(DMA2_CH4, 10u8, Stop1);
dma_channel_impl!(DMA2_CH5, 11u8, Stop1);
dma_channel_impl!(DMA2_CH6, 12u8, Stop1);
dma_channel_impl!(DMA2_CH7, 13u8, Stop1);
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL1() {
    <crate::peripherals::DMA1_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL2() {
    <crate::peripherals::DMA1_CH2 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL3() {
    <crate::peripherals::DMA1_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL4() {
    <crate::peripherals::DMA1_CH4 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL5() {
    <crate::peripherals::DMA1_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL6() {
    <crate::peripherals::DMA1_CH6 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL7() {
    <crate::peripherals::DMA1_CH7 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL1() {
    <crate::peripherals::DMA2_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL2() {
    <crate::peripherals::DMA2_CH2 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL3() {
    <crate::peripherals::DMA2_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL4() {
    <crate::peripherals::DMA2_CH4 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL5() {
    <crate::peripherals::DMA2_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL6() {
    <crate::peripherals::DMA2_CH6 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_CHANNEL7() {
    <crate::peripherals::DMA2_CH7 as crate::dma::ChannelInterrupt>::on_irq();
}
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 0usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 1usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 2usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 3usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 4usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 5usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 6usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 0usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 1usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 2usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 3usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 4usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 5usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 6usize,
    },
];
pub fn gpio_block(n: usize) -> crate::pac::gpio::Gpio {
    unsafe { crate::pac::gpio::Gpio::from_ptr((1207959552usize + 1024usize * n) as _) }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 1048576usize;
pub const WRITE_SIZE: usize = 8usize;
