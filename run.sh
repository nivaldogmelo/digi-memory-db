#!/bin/bash

# Create flash image if it doesn't exist
if [ ! -f flash_image.bin ]; then
    dd if=/dev/zero of=flash_image.bin bs=1M count=1
fi

# Build the Rust binary
cargo build --release --target thumbv7m-none-eabi

# Run QEMU with the specified flash image
qemu-system-arm -M lm3s6965evb -kernel target/thumbv7m-none-eabi/release/digi-memory-db -nographic -serial mon:stdio -drive if=sd,format=raw,file=$(pwd)/flash_image.bin
