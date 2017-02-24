use m4::gpio::GPIO as dev_GPIO;

pub use m4::gpio::MODE as GPIO_MODE;
pub use m4::gpio::PIN as GPIO_PIN;
use m4::rcc::RCC as dev_RCC;

iomap!{
  GPIOA is dev_GPIO @ 0x4002_0000;
  GPIOB is dev_GPIO @ 0x4002_0400;
  GPIOC is dev_GPIO @ 0x4002_0800;
  GPIOD is dev_GPIO @ 0x4002_0C00;
  GPIOE is dev_GPIO @ 0x4002_1000;
  GPIOF is dev_GPIO @ 0x4002_1400;
  GPIOG is dev_GPIO @ 0x4002_1800;
  GPIOH is dev_GPIO @ 0x4002_1C00;
  GPIOI is dev_GPIO @ 0x4002_2000;
  GPIOJ is dev_GPIO @ 0x4002_2400;
  GPIOK is dev_GPIO @ 0x4002_2800;

  RCC   is dev_RCC  @ 0x4002_3800
}
