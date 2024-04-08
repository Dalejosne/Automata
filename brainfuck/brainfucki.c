/*
* Code which interpret a brainfuck code
* Copyright Damien Lejosne 2021, all right reserved
*/
#define __BF_IMPLEMENTATION__
#include "brainfucki.h"
#include <stdbool.h>
#define BUFFER_SIZE 10000
#define STATES_COUNT 100000

int file_size(FILE* file) {
    int size = 0;
    while(fgetc(file) != EOF) size++;
    rewind(file);
    return size;
}

bool is_correct(char* code, int len) {
    int opening = 0;
    for(int i = 0; i < len; i++) {
        if(code[i] == '[') opening++;
        else if(code[i] == ']') opening--;
        if(opening < 0) return false;
    }
    return opening == 0;
}

char* retrieve_interactive(int* size) {
    char* res = malloc(sizeof(char) * (BUFFER_SIZE + 1));
    int capacity = BUFFER_SIZE;
    bool stop = false;
    printf(">>> ");
    while(!stop) {
        if(capacity < BUFFER_SIZE + *size ) {
            capacity += BUFFER_SIZE;
            res = realloc(res, sizeof(char) * capacity);
        }
        fgets(res + *size, BUFFER_SIZE, stdin);
        while(res[*size] != '\0') {
            if(res[*size] == 'q') {
                free(res);
                return NULL;
            }
            (*size) ++;
        }
        stop = is_correct(res, *size);
        if(!stop) {
            printf("\n... ");
        }
    }
    printf("\n");
    return res;
}

char* read_code(FILE* in, bool interactive, int* code_size) {
    char* code = NULL;
    if(interactive) {
        code = retrieve_interactive(code_size);
    } else {
        *code_size = file_size(in);
        code = malloc(sizeof(char) * (*code_size + 1));
        for(int i = 0; i < *code_size; i++) {
            code[i] = fgetc(in);
        }
        code[*code_size] = '\0';
    }
    return code;
}

//Main
int main(int argc, char** argv){
    //Welcome msg
    printf("Welcome to this nice brainfuck interpretor !\n\
RQ : Maximum line size is limited to 10_000 caracters, and it has 100_000 available states (of int size).\n\
You can run it directly to use it interactivly, or specify a fileName to interpret the corresponding programm.\n\
You can do the followings actions :\n\
> equiv to i++ in C\n\
< equiv to i-- in C\n\
+ equiv to state[i]++ in C\n\
- equiv to state[i]-- in C\n\
[ equiv to while(state[i]){ in C\n\
] equiv to } in C\n\
; equiv to state[i]=getc(stdin) in C\n\
. equiv to putc(state[i], stdout) in C.\n\
And type q to quit\n\
Have fun ! ;-)\n");
    //Initialisation
    bool INTERACTIVE=true;
    int* a_states = NULL;
    FILE* in  = stdin;
    
    //Init states from file and decide if running interactivly or not
    if(argc>1){
        a_states = malloc(sizeof(int) * STATES_COUNT);//Test if the states has been correctly created come after
        printf("Not interactively running\n\n");
        in  = fopen(argv[1], "r");//Test if the file has been correctly opened come after
        INTERACTIVE=false;
    }else
        a_states = malloc(sizeof(int) * STATES_COUNT);
    // Initialize states :
    for(int i = 0; i < STATES_COUNT; i++) {
        a_states[i] = 0;
    }
    
    //Check if errors occured
    if(!a_states){
        printf("Cannot create states array, maybe due to a lack of memory\n");
        exit (EXIT_FAILURE);
    }
    if(!in){
        printf("Error opening the file %s", argv[1]);
        exit(EXIT_FAILURE);
    }

    int pc = 0;
    //Main loop
    do {
        // read code
        int code_size = 0;
        char* code = read_code(in, INTERACTIVE, &code_size);
        if(!code) break;
        // execute it
        switch(eval(code, a_states, STATES_COUNT, &pc)) {
            case BF_NO_ERROR:break;
            case BF_LPARSE_ERROR:
                printf("Parse error : Unexpected ']' (without previous '[')\n");
                exit(EXIT_FAILURE);
            break;
            case BF_RPARSE_ERROR:
                if(!INTERACTIVE) {
                    printf("Parse error : Unended '['.\n");
                    exit(EXIT_FAILURE);
                }
            break;
        }
        free(code);
    } while(INTERACTIVE);
	free(a_states);
    return 0;
}
