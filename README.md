# Rust LCD Display Implementation

This repository contains a simple 16x2 LCD display module implementation in Rust. The code is implemented using the [```stm32f1xx-hal```](https://github.com/stm32-rs/stm32f1xx-hal) crate. The module can be easily ported to other MCUs provided you adapt it to its proper library.

The module itself can be found in the ```src/lcd.rs``` file, and the ```src/main.rs``` file is a simple **Hello World!** code. The board used in the main example is the [NUCLEO-F103RB](https://www.st.com/en/evaluation-tools/nucleo-f103rb.html).

You can find a more *in-depth* description of the code on this [post](https://bacelarhenrique.me/2021/03/01/how-to-configure-lcd-rust.html).

# Building
```bash
cargo build
```

# Flashing
```bash
cargo flash --chip stm32f103rb
```

# References
- [How To Configure LCD in 4-Bit Mode on STM32 With Rust](https://bacelarhenrique.me/2021/03/01/how-to-configure-lcd-rust.html)
- [```stm32f1xx-hal```](https://github.com/stm32-rs/stm32f1xx-hal)

# Author
Henrique Bacelar
