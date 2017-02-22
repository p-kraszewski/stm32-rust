#![allow(dead_code)]
use ARM::STM32::M4::GPIO::*;

pub use ARM::STM32::M4::GPIO::MODE as GPIO_MODE;

pub const GPIOA: *mut GPIO = 0x40020000 as *mut GPIO;
pub const GPIOB: *mut GPIO = 0x40020400 as *mut GPIO;
pub const GPIOC: *mut GPIO = 0x40020800 as *mut GPIO;
pub const GPIOD: *mut GPIO = 0x40020C00 as *mut GPIO;
pub const GPIOE: *mut GPIO = 0x40021000 as *mut GPIO;
