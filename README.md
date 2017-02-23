# Programming STM32 microcontrollers in Rust

Strongly influenced by [this article](http://japaric.github.io/copper/intro.html) by @japaric ([Jorge Aparicio](https://github.com/japaric)). Thank you!

## Prerequisites
Everything is based on `Ubuntu 16.10` platform. Not tested on other. 
  
What you will need:
* **Rust-nigtly**, preferably installed via  [rustup](https://www.rustup.rs/).
* **rust-src**, installed via `rustup component add rust-src`  
* **xargo**, installed via `cargo install xargo`, also by @japaric.

## Plans 
 
As of now - this is strongly **work in progress**. I plan to make it as complete as possible for the following processors (with roughly that order of importance):
* [STM32F407VG](http://www.st.com/en/microcontrollers/stm32f407vg.html) as found in [Discovery F4](http://www.st.com/en/evaluation-tools/stm32f4discovery.html)
* [STM32L062K8](http://www.st.com/content/st_com/en/products/microcontrollers/stm32-32-bit-arm-cortex-mcus/stm32l0-series/stm32l0x2/stm32l062k8.html) (for my pet-project)
* [STM32F439VG](http://www.st.com/en/microcontrollers/stm32f439vg.html) (my cpu-swapped DiscoveryF4 variant, with on-board crypto)
  
If time gives, I'll try the [STM32F746NG](http://www.st.com/en/microcontrollers/stm32f746ng.html) behemoth, as found in  [32F746GDISCOVERY](http://www.st.com/en/evaluation-tools/32f746gdiscovery.html).

## Purpose
To have maximally elastic macro- and object-based system to program 32-bit ARM processors, with some light std:: library focused on memory-constrained systems.
 
Each peripheral device will be implemented as a class with respective configuration and query methods (and of course a direct registry access). 

### Device memory map

```rust
// ...
iomap!{
    GPIO @ 0x40020000 -> GPIOA;
    GPIO @ 0x40020400 -> GPIOB;
    GPIO @ 0x40020800 -> GPIOC;
    GPIO @ 0x40020C00 -> GPIOD;
    GPIO @ 0x40021000 -> GPIOE
}
// ...
```


### Device usage

with *class-like* syntax:
```rust
// ...
let n = io!{GPIOE.SPEED};
io!{GPIOE.SPEED = n+5};
// ...
io!{GPIOE.setmode(GPIO_PIN::PIN03, GPIO_MODE::Output)};
io!{GPIOE.bset(0b0000000000001000)};
// ...
```
or with descriptive syntax:
```rust
// ...
let n = io!{get SPEED from GPIOE};
io!{set SPEED in GPIOE to n+5};
// ...
io!{with GPIOE do setmode(GPIO_PIN::PIN03, GPIO_MODE::Output)};
io!{with GPIOE do bset(0b0000000000001000)};
// ...
```
