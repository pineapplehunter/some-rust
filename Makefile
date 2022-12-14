OBJCOPY := riscv64-unknown-elf-objcopy

all: loader_binary.o

loader_binary.o: loader_binary.bin
	$(OBJCOPY) --input binary $< -O elf64-littleriscv $@

loader_binary.bin: /home/shogo/some-rust/target/riscv64i/release/some-rust
	$(OBJCOPY) $< -O binary $@

/home/shogo/some-rust/target/riscv64i/release/some-rust:
	cargo build --release