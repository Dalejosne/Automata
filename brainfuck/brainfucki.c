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
    //Welcome msg
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
    //Initialisation
    char INTERACTIVE=1;
    char STOP=0;
    Buffer* buffer = NULL;
    FILE* out = stdout;
    FILE* in  = stdin;
    init_nb_state(NB_STATE);
    
    //Init buffer from file and decide if running interactivly or not
    if(argc>1){
        buffer = new_buffer(100000);//Test if the buffer has been correctly created come after
        printf("Not interactivly running\n\n");
        in  = fopen(argv[1], "r");//Test if the file has been correctly opened come after
        INTERACTIVE=0;
    }else
        buffer = new_buffer(10000);
    
    //Check if errors occured
    if(!buffer){
        printf("Cannot create buffer, maybe due to a lack of memory\n");
        exit (0);
    }
    if(!in){
        printf("Error opening the file %s", argv[1]);
        exit(0);
    }
    
    //Main loop
    while(!STOP){
        STree* treeAct = new_stree(0, in, buffer, INTERACTIVE, &STOP);
        stree_eval(treeAct, out);
        delete_stree(treeAct);
    }
    return 0;
}
