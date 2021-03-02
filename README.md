# Rust LCD Display Implementation

This repository contains a simple 16x2 LCD display module implementation in Rust. The code is implemented using the ```stm32f1xx-hal``` crate, which documentation can be found [here](https://docs.rs/stm32f1xx-hal/0.7.0/stm32f1xx_hal/).

The module can be easily ported to other MCUs provided you adapt it to its proper library.

The module code can be found in the ```src/lcd.rs``` file, and the ```src/main.rs``` file is a simple **Hello World!** code.