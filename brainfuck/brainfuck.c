#include <stdio.h>
#include <stdlib.h>

//Struct
/**
* buffer for reading input correctly
*/
typedef struct Buffer{
    int value[1000];
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
Buffer buffer;
int state[100];
int i = 0;
//Util
/**
* Read caracters at input
*/
int readChar(){
    if(buffer.ind==buffer.size){
        printf("> ");
        buffer.ind=0;
        buffer.size=0;
        char c = ' ';
        for(int i = 0; i<1000 && !(c=='\n' || c=='\0' || c==EOF); i++){
            c=getc(in);
            buffer.value[buffer.size++] = c;
        }
    }
    return buffer.value[buffer.ind++];
}
//Analyse
/**
* Create the syntax tree
* Return -1 if get [ after a ]
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
            case ';' : This->value=READ; return This;
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
/**
* Evaluate the syntax tree
*/
void evalSTree(STree* s){
    STree* act = s;
    //Evaluate each member of the linked list (s->next->next->...)
    while(act){
        switch(act->value){
            case LOOP : 
                int j = 0;
                while(state[i] && j++<10)
                    evalSTree(act->firstChild);//Evaluate loop body
            break;
            case NEXT : i++; break;
            case PREC : i--; break;
            case PRINT : putc(state[i], out); break;
            case READ : printf("Enter a number :\n");state[i]=getc(in);getc(in); break;
            case INC : state[i]++; break;
            case DEC : state[i]--; break;
        }
        act=act->next;
    }
}

//Main
int main(){
    printf("Welcome to this nice brainfuck interpretor !\nYou can do the followings actions :\n\
RQ : Maximum line (or file) size is limited to 1000 caracters.\n\
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
    in  = stdin;
    out = stdout;
    buffer.ind=0;
    buffer.size=0;
    while(1){
        STree* treeAct = createSTree(0);
        evalSTree(treeAct);
        delTree(treeAct);
    }
    return 0;
}
