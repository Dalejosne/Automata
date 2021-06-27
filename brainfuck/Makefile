CC=gcc -Wall -O3
MINGW=x86_64-w64-mingw32-gcc -static-libgcc -static-libstdc++
INCLUDE=-I$(CURDIR)

all:windows linux

linux:utils_o stree_o buffer_o brainfucki_o
	$(CC) *.o -o brainfucki
windows:utils_o_win stree_o_win buffer_o_win brainfucki_o_win
	$(MINGW) *.o -o brainfucki.exe

utils_o:
	$(CC) -c utils.c $(INCLUDE)
stree_o:
	$(CC) -c stree.c $(INCLUDE)
buffer_o:
	$(CC) -c buffer.c $(INCLUDE)
brainfucki_o:
	$(CC) -c brainfucki.c $(INCLUDE)

utils_o_win:
	$(MINGW) -c utils.c $(INCLUDE)
stree_o_win:
	$(MINGW) -c stree.c $(INCLUDE)
buffer_o_win:
	$(MINGW) -c buffer.c $(INCLUDE)
brainfucki_o_win:
	$(MINGW) -c brainfucki.c $(INCLUDE)

clean:
	rm -f *.o
clean_all:clean
	rm -f brainfucki *.exe