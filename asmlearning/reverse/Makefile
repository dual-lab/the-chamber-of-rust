all:
	nasm -g -f elf64 -I ../macro/ -o reverse.o reverse.asm
	ld -o reverse reverse.o

clean:
	rm reverse reverse.o
