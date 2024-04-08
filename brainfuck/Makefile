CC_DEBUG=gcc -Wall -Wextra -Wpedantic -g -fsanitize=address,undefined
CC=gcc -Wall -Wextra -Wpedantic -O3 -s
MINGW=x86_64-w64-mingw32-gcc -static-libgcc
BIN=$(CURDIR)/build

all: linux
	./build/brainfucki

linux:
	$(CC) brainfucki.c -o $(BIN)/brainfucki

debug:
	$(CC_DEBUG) brainfucki.c -o $(BIN)/brainfucki

test:debug
	$(BIN)/brainfucki $(CURDIR)/examples/HelloWorld.bf

windows:
	@export CPATH=/usr/x86_64-w64-mingw32/include #maybe you will need to change it according to the install path of mingw on your distro. It works on arch.
	$(MINGW) brainfucki.c -o $(BIN)/brainfucki.exe

clean:
	@echo "Removing everything in build..."
	rm -rf $(BIN)/*
