test:
	cargo run
	riscv-gcc/bin/riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 asm.S -o out
    spike --isa=RV32G env/pk out

dump:
	