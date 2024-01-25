RISCV_PREFIX = riscv64-none-elf-
OBJCOPY = $(RISCV_PREFIX)objcopy
OBJDUMP = $(RISCV_PREFIX)objdump
RUST_TARGET := output/rust-riscv-benches
RUST_TARGET2 := output/bench
RUST_TARGET3 := output/inner_product
RUST_SOURCES := $(shell find src -type f) Cargo.lock Cargo.toml linker.ld riscv64-custom.json

.PHONY: all clean

all: output/loader.dump output/loader_binary.o output/loader_binary.dump output/bench.hex output/bench.dump output/inner_product.hex output/inner_product.dump

output/loader_binary.o: output/loader.bin
	$(OBJCOPY) -I binary -O elf64-littleriscv --set-section-alignment .data=8 $< $@

output/loader_binary.bin: $(RUST_TARGET)
	$(OBJCOPY) -O binary $< $@

output/loader_binary.dump: output/loader_binary.o
	$(OBJDUMP) $< -D > $@

output/loader: $(RUST_TARGET)
	mv $< $@

%.dump: %
	# $(OBJDUMP) $< -d -j .text -j .rodata -j .data -j .bss -C -M no-aliases -M numeric > $@
	$(OBJDUMP) $< -d -j .text -j .rodata -j .data -j .bss -C > $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.hex: %.bin
	od -An -t x1 $< -w1 -v | tr -d " " > $@

$(RUST_TARGET): $(RUST_SOURCES)
	cargo build --release -Z unstable-options --out-dir output

$(RUST_TARGET2): $(RUST_SOURCES)
	cargo build --release -Z unstable-options --out-dir output

run: $(RUST_TARGET)
	qemu-system-riscv64 -machine sifive_u -cpu rv64 -m 256M -bios none -display none -serial stdio -kernel $<

clean:
	cargo clean
	rm -f *.dump *.o *.bin
	rm -f output/*
