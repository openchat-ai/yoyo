# yoyo-asm — ground truth (Project 4)

NASM ?= nasm
LD ?= ld

.PHONY: all clean test

all: yoyo-asm

yoyo-asm: yoyo-asm.o
	$(LD) -o $@ $<

yoyo-asm.o: yoyo-asm.s
	$(NASM) -f elf64 -o $@ $<

# Windows note: use `nasm -f win64` + link with GoLink/lld-link (Phase 4d).

clean:
	rm -f yoyo-asm yoyo-asm.o yoyo-asm.exe *.obj

test: yoyo-asm
	bash test.sh
