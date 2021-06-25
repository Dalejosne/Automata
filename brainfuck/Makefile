CC=gcc -Wall -O3
INCLUDE=-I$(CURDIR)

all:utils_o stree_o buffer_o brainfucki_o
	$(CC) *.o -o brainfucki
utils_o:
	$(CC) -c utils.c $(INCLUDE)
stree_o:
	$(CC) -c stree.c $(INCLUDE)

buffer_o:
	$(CC) -c buffer.c $(INCLUDE)

brainfucki_o:
	$(CC) -c brainfucki.c $(INCLUDE)

clean:
	rm -f *.o brainfucki bfucki