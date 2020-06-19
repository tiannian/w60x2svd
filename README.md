# w60x2svd

A tool for generating SVD files for the `w60x`.

This tool is required because official SVD files are not available for these devices at this time.
The generated SVD files are used for generating the `w60x`peripheral access crates using [svd2rust].

Register definitions from w60x official documents:
- [w600 registers manual](https://download.w600.fun/document/W600_寄存器手册.pdf)

## Status

- [X] Generate `Device` from json.
- [X] Generate `Peripheral` from json.
- [X] Generate `Register` from json.
- [X] Generate `Field` from json.
- [ ] Extract definitions from official registers description document.
  - [X] CLK
  - [X] DMA
  - [ ] GPIO
  - [X] RSA
  - [X] Crypto
  - [ ] HSPI
  - [ ] SDIO
  - [ ] Wrapper
  - [ ] SPI
  - [ ] I2C
  - [ ] UART
  - [ ] 7816
  - [ ] Timer
  - [ ] PMU
  - [ ] RTC
  - [ ] WDG
  - [ ] PWM
  - [ ] QFLASH

## Building

```bash
$ git clone https://github.com/tiannian/w60x2svd
$ cd w60x2svd/ && cargo build
```

### For w600

```bash
$ cargo run w600
```

This will create the file `esp32.svd` in the base project directory.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[svd2rust]: https://github.com/rust-embedded/svd2rust
