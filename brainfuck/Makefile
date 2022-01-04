CC=gcc -Wall -O3
MINGW=x86_64-w64-mingw32-gcc -static-libgcc -static-libstdc++
INCLUDE=-I$(CURDIR)/include
SRC=$(CURDIR)/src
BIN=$(CURDIR)/build

all:windows linux
	make clean

linux:utils_o stree_o buffer_o brainfucki_o
	$(CC) *.o -o $(BIN)/brainfucki
	make clean
windows:utils_o_win stree_o_win buffer_o_win brainfucki_o_win
	$(MINGW) *.o -o $(BIN)/brainfucki.exe
	make clean

utils_o:
	$(CC) -c $(SRC)/utils.c $(INCLUDE)
stree_o:
	$(CC) -c $(SRC)/stree.c $(INCLUDE)
buffer_o:
	$(CC) -c $(SRC)/buffer.c $(INCLUDE)
brainfucki_o:
	$(CC) -c $(SRC)/brainfucki.c $(INCLUDE)

utils_o_win:
	$(MINGW) -c $(SRC)/utils.c $(INCLUDE)
stree_o_win:
	$(MINGW) -c $(SRC)/stree.c $(INCLUDE)
buffer_o_win:
	$(MINGW) -c $(SRC)/buffer.c $(INCLUDE)
brainfucki_o_win:
	$(MINGW) -c $(SRC)/brainfucki.c $(INCLUDE)

doc:
	make -C docs html

clean:
	make -C docs clean
	rm -f *.o
clean_all:clean
	rm -f $(BIN)/brainfucki $(BIN)/*.exe
