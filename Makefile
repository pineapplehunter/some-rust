RISCV_PREFIX = riscv64-none-elf-
OBJCOPY = $(RISCV_PREFIX)objcopy
OBJDUMP = $(RISCV_PREFIX)objdump
# OBJDUMP = llvm-objdump
RUST_TARGET := output/rust-riscv-benches
RUST_TARGET_OTHERS := output/bench \
						output/inner_product \
						output/element_wise_mul \
						output/element_wise_mul_halving \
						output/element_wise_mul_mt \
						output/element_wise_mul_mt_same \
						output/mat_mul_mt_same \
						output/mt_many_tasks \
						output/median
RUST_SOURCES := $(shell find src -type f) Cargo.lock Cargo.toml linker.ld riscv64-custom.json

OUTPUT_HEX = $(addsuffix .hex,$(RUST_TARGET_OTHERS))
OUTPUT_DUMP = $(addsuffix .dump,$(RUST_TARGET_OTHERS))

.PHONY: all clean

all: output/loader.dump output/loader_binary.o output/loader_binary.dump $(OUTPUT_HEX) $(OUTPUT_DUMP)

output/loader_binary.o: output/loader.bin
	$(OBJCOPY) -I binary -O elf64-littleriscv --set-section-alignment .data=8 $< $@

output/loader_binary.bin: $(RUST_TARGET)
	$(OBJCOPY) -O binary $< $@

output/loader_binary.dump: output/loader_binary.o
	$(OBJDUMP) $< -D > $@

output/loader: $(RUST_TARGET)
	cp $< $@

%.dump: %
	$(OBJDUMP) $< -d -j .text -j .rodata -j .data -j .bss -C -S -M numeric > $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.hex: %.bin
	od -An -t x1 $< -w1 -v | tr -d " " > $@

$(RUST_TARGET_OTHERS): $(RUST_TARGET)

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
