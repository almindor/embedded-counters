# Embedded Cuonters

Library that provided `embedded-hal` implementations for benchmarking transfer performance for drivers.

## *!WARNING!* Experimental

This crate is still very experimental. The API, in particular the serialization/display aspect will probably change a lot.

## Usage

Drivers such as [ST7789](https://github.com/almindor/st7789) can gauge performance on host platforms by counting transfers over SPI, I2C or other `embedded-hal` abstracted interfaces.

Use the provided structures in stead of real abstractions with your driver implementation to get byte, call and delay counts of your driver given a specific operation.

## Examples

* `st7789` - basic shapes drawing benchmark