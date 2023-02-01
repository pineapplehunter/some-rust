RISCV_PREFIX ?= riscv64-unknown-elf-
OBJCOPY ?= $(RISCV_PREFIX)objcopy
OBJDUMP ?= $(RISCV_PREFIX)objdump
RUST_TARGET := output/some-rust
RUST_SOURCES := $(shell find src -type f) Cargo.lock Cargo.toml

.PHONY: all clean

all: output/loader_binary.o output/loader_binary.dump output/loader.dump

output/loader_binary.o: output/loader_binary.bin
	$(OBJCOPY) -I binary -O elf64-littleriscv --set-section-alignment .data=8 $< $@

output/loader_binary.bin: $(RUST_TARGET)
	$(OBJCOPY) -O binary $< $@

output/loader_binary.dump: output/loader_binary.o
	$(OBJDUMP) $< -D > $@

output/loader.dump: $(RUST_TARGET)
	$(OBJDUMP) $< -d -j .text -j .rodata -j .data -j .bss -C > $@

$(RUST_TARGET): $(RUST_SOURCES)
	cargo build --release -Z unstable-options --out-dir output

clean:
	cargo clean
	rm -f *.dump *.o *.bin
	rm -f output/*