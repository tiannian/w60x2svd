# w60x2svd

A tool for generating SVD files for the `w60x`.

This tool is required because official SVD files are not available for these devices at this time.
The generated SVD files are used for generating the `w60x`peripheral access crates using [svd2rust].

Register definitions from w60x official documents:
- [w600 registers manual](https://download.w600.fun/document/W600_寄存器手册.pdf)

## Status

- [X] CLK
- [X] DMA
- [X] GPIO
- [X] RSA
- [X] Crypto
- [ ] HSPI
- [ ] SDIO
- [ ] Wrapper
- [X] SPI
- [X] I2C
- [X] UART
- [X] 7816
- [X] Timer
- [X] PMU
- [X] RTC
- [X] WDG
- [X] PWM
- [X] QFLASH

## Building

```bash
$ git clone https://github.com/tiannian/w60x2svd
$ cd w60x2svd/ && cargo build
```

### For w600

```bash
$ cargo run json
$ cargo run svd
$ xmllint -format w600.svd > w600.format.svd
```

This will create the file `w600.svd` in the base project directory.

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
