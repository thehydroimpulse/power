build:
	rustc /vagrant/src/lib.rs --crate-name power --crate-type staticlib -g -C metadata=f718bb6745ff3366 -C extra-filename=-f718bb6745ff3366 --out-dir /vagrant/target/arm --dep-info /vagrant/target/arm/.fingerprint/power-f718bb6745ff3366/dep-lib-power --target arm -L /vagrant/target/arm -L /vagrant/target/arm/deps --extern rlibc=/vagrant/target/arm/deps/librlibc-174bb87856bbb923.rlib
	arm-none-eabi-ld  -o target/power.elf target/arm/libpower-f718bb6745ff3366.a -T layout.ld
	arm-none-eabi-objcopy target/power.elf target/power.bin

push:
	cp target/power.bin /Volumes/MBED
	

.PHONY: build push
