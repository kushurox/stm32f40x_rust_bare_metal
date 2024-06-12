CMD = ~/Documents/arm-gnu-toolchain-13.2.Rel1-x86_64-arm-none-eabi/bin/arm-none-eabi-objcopy
TR  = target/thumbv7em-none-eabihf/release/unmute

run: src/main.rs src/link.x $(TR)
	$(CMD) -O binary $(TR) firmware/firmware.bin