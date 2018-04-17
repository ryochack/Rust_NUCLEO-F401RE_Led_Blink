# STMicro NUCLEO-F401RE board LED blink sample by Rust

## Setups
```sh
$ rustup update
$ rustup default nightly
```

Add Cortex-M target by rustup.
```sh
// NUCLEO-F401RE(STM32F401 MCU) is Cortex-M4 with FPU (ARMv7E-M)
$ rustup target add thumbv7em-none-eabihf
```

## Build
```sh
$ cargo build --target thumbv7em-none-eabihf

=> firmware file: target/thumbv7em-none-eabihf/debug/nucleo-f401re_led_blink is out.
```

## Write Firmware to NUCLEO-F401RE Board
```sh
$ openocd -f board/st_nucleo_f4.cfg
```

Open another terminal, and execute telnet.
```sh
$ telnet localhost 4444

> reset halt
> flash write_image erase $(absolute_path_to_firmware)
> reset
```

Blink LED!
