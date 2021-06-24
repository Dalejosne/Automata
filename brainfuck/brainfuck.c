/*
* Code which interpret a brainfuck code
* Copyright Damien Lejosne 2021, all right reserved
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NB_STATE 100000
typedef struct Buffer{
    char value[100000];
    int ind, size;
}Buffer;
//Types of possible actions to do
enum {LOOP, INC, DEC, PRINT, READ, NEXT, PREC};
/**
* Syntax tree structure
* Equiv to :
* This->firstChild------------->Another tree...
*   |       |
*   |     next------------->Another tree...
*   |       |
*   |---->next (lastChild)
*/
typedef struct Tree{
    struct Tree* next;
    struct Tree* firstChild;
    struct Tree* lastChild;
    int value;
}STree;

void delTree(STree* s){
    if(!s)
        return;
    delTree(s->next);
    delTree(s->firstChild);
    //RQ : Do not delete last, which is contained in nexts or which is equal to firstChild
    free(s);
}

//Global
FILE* in;
FILE* out;
char STOP=0;//Set to 1 if EOF is reached
char INTERACTIVE=1;
Buffer buffer;

//Utils
/**
* Read caracters at input
*/
int readChar(){
    static char fileRead=0;
    if(!fileRead && !INTERACTIVE){
        fileRead=1;
        buffer.ind=0;
        int c = 0;
        while(c != EOF){
            c = getc(in);
            if(c=='>' || c=='<' || c=='+' || c=='-' || c=='[' || c==']' || c==',' || c=='.')
                buffer.value[buffer.size++]=c;
        }
    }
    if((buffer.value[buffer.ind]=='\0' || buffer.ind==0) && INTERACTIVE){
        printf("> ");
        buffer.ind=0;
        fgets(buffer.value, 10000, in);
        if(!strchr(buffer.value, '\n')){
            int c = 0;
            while (c != '\n' && c != EOF)
                c = getc(in);
            if(c==EOF){
                STOP=1;
                return -1;
            }
        }
    }else if(buffer.size==buffer.ind && !INTERACTIVE){
        STOP=1;
        return -1;
    }
    int c = buffer.value[buffer.ind];
    buffer.ind++;
    return c;
}
//Analyse
/**
* Create the syntax tree.
* You can also uses it yourself if you define your own "readChar" function.
* @param fromLoop 0 if call from the beginning, 1 if call recursively from a loop
* @return The newly created STree, or NULL
*/
STree* createSTree(char fromLoop){
    STree* This = (STree*) malloc(sizeof(STree));
    This->next=NULL;
    This->firstChild=NULL;
    This->lastChild=NULL;
    char stop=0;
    while(!stop){
        stop=1;
        int c = readChar();
        if(STOP)//EOF
            return NULL;
        switch(c){
            case '[' :
                This->value=LOOP;
                STree* child = createSTree(1);
                while(child){
                    if(This->firstChild==NULL)//It is the first child
                        This->firstChild=child;
                    else//Make a linked list of children
                        This->lastChild->next=child;
                    This->lastChild=child;
                    child = createSTree(1);//Child are also syntax trees
                }
                if(STOP){
                    printf("Unexpected EOF while parsing\n");
                    return NULL;
                }
                return This;
            break;
            case ']' :
                if(!fromLoop)
                    printf("Error : unexpected ] while parsing\n");
                goto ERROR;
            break;
            case '>' : This->value=NEXT; return This;
            case '<' : This->value=PREC; return This;
            case '.' : This->value=PRINT; return This;
            case ',' : This->value=READ; return This;
            case '+' : This->value=INC; return This;
            case '-' : This->value=DEC; return This;
            case 'q' : exit(0);
            default : stop = 0;
        }
    }
ERROR:
    free(This);
    return NULL;
}
/**
* Evaluate the syntax tree
* @param s Syntax tree to be evaluated
*/
void evalSTree(STree* s){
    static int i = 0;
    static int state[NB_STATE];
    STree* act = s;
    //Evaluate each member of the linked list (s->next->next->...)
    while(act){
        switch(act->value){
            case LOOP : 
                int j = 0;
                while(state[i] && j++<10)
                    evalSTree(act->firstChild);//Evaluate loop body
            break;
            case NEXT :
                i++;
                i%=NB_STATE;
            break;
            case PREC :
                i--;
                if(i<0)
                    i+=NB_STATE;
            break;
            case PRINT : putc(state[i], out); break;
            case READ : state[i]=getc(stdin); break;
            case INC : state[i]++; break;
            case DEC : state[i]--; break;
        }
        act=act->next;
    }
}
//Debug
void showTree(STree* This){
    static int level=0;
    if(This==NULL){
        printf("-1\n");
        return;
    }
    level++;
    STree* next = This;
    while(next){
        for(int i = 1; i<level; i++)
            printf("  ");
        printf("v=%i\n", next->value);
        showTree(next->firstChild);
        next = next->next;
    }
    level--;
}

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
    out = stdout;
    buffer.ind=0;
    if(argc>1){
        printf("Not interactivly running\n\n");
        in  = fopen(argv[1], "r");
        if(!in){
            printf("Error opening the file %s", argv[1]);
            exit(0);
        }
        INTERACTIVE=0;
    }else
        in  = stdin;
    while(!STOP){
        STree* treeAct = createSTree(0);
        evalSTree(treeAct);
        delTree(treeAct);
    }
    return 0;
}
