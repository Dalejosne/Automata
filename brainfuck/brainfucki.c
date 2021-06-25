/*
* Code which interpret a brainfuck code
* Copyright Damien Lejosne 2021, all right reserved
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stree.h>
#include <buffer.h>
#define NB_STATE 100000

//Main
int main(int argc, char** argv){
    printf("Welcome to this nice brainfuck interpretor !\n\
RQ : Maximum line size is limited to 10000 caracters.\n\
You can run it directly to use it interactivly, or specify a fileName to interpret the corresponding programm.\n\
You can do the followings actions :\n\
> equiv to i++ in C\n\
< equiv to i-- in C\n\
+ equiv to state[i]+=1 in C\n\
- equiv to state[i]-=1 in C\n\
[ equiv to while(state[i]){ in C\n\
] equiv to } in C\n\
; equiv to state[i]=getc(stdin) in C\n\
. equiv to putc(state[i], stdin) in C.\n\
And type q to quit\n\
Have fun ! ;-)\n");
    char INTERACTIVE=1;
    char STOP=0;
    Buffer* buffer;
    FILE* out = stdout;
    FILE* in  = stdin;
    init_nb_state(NB_STATE);
    if(argc>1){
        buffer = new_buffer(100000);
        printf("Not interactivly running\n\n");
        in  = fopen(argv[1], "r");
        if(!in){
            printf("Error opening the file %s", argv[1]);
            exit(0);
        }
        INTERACTIVE=0;
    }else
        buffer = new_buffer(10000);
    while(!STOP){
        STree* treeAct = new_stree(0, in, buffer, INTERACTIVE, &STOP);
        stree_eval(treeAct, out);
        delete_stree(treeAct);
    }
    return 0;
}
