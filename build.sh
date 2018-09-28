#!/bin/bash
set -e

cargo build --release && \
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/guided-rocket-rs target/thumbv6m-none-eabi/release/guided-rocket-rs.bin && \
~/.arduino15/packages/arduino/tools/bossac/1.7.0/bossac -i -d -U true -i -e -w -v target/thumbv6m-none-eabi/release/guided-rocket-rs.bin -R
