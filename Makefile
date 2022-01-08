c:
	cargo run
	riscv-gcc/bin/riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 asm.S -o out
vm:
	spike --isa=RV32G env/pk out
	echo $?

dump:
	rust-objdump --arch-name=riscv32 -x out

readelf:
	riscv-gcc/bin/riscv64-unknown-elf-readelf -a out

c-a:
	riscv-gcc/bin/riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 -S env/input.c 