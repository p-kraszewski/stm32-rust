#![allow(dead_code)]


use m4::gpio::GPIO;

pub use m4::gpio::MODE as GPIO_MODE;
pub use m4::gpio::PIN as GPIO_PIN;

iomap!{
    GPIO @ 0x40020000 -> GPIOA;
    GPIO @ 0x40020400 -> GPIOB;
    GPIO @ 0x40020800 -> GPIOC;
    GPIO @ 0x40020C00 -> GPIOD;
    GPIO @ 0x40021000 -> GPIOE
}
