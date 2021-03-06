/*
* Syntax tree main code
* Copyright Damien Lejosne 2021, all right reserved
*/

#include <stree.h>

//Global variables used by the syntax tree evaluator
int NB_STATE;
int* state;

void init_nb_state(int nb_state){
    NB_STATE=nb_state;
    state=malloc(sizeof(int)*nb_state);
    for(int i = 0; i<nb_state; i++)
        state[i]=0;
}

STree* new_stree(char from_loop, FILE* in, Buffer* buffer, char INTERACTIVE, char* prgm_should_stop){
    STree* This = (STree*) malloc(sizeof(STree));
    This->next=NULL;
    This->first_child=NULL;
    This->last_child=NULL;
    char stop=0;
    while(!stop){
        stop=1;
        int c = read_char(in, buffer, INTERACTIVE, prgm_should_stop);
        if(*prgm_should_stop)//EOF
            return NULL;
        switch(c){
            case '[' :
                This->value=LOOP;
                STree* child = new_stree(1, in, buffer, INTERACTIVE, prgm_should_stop);
                while(child){
                    if(This->first_child==NULL)//It is the first child
                        This->first_child=child;
                    else//Make a linked list of children
                        This->last_child->next=child;
                    This->last_child=child;
                    child = new_stree(1, in, buffer, INTERACTIVE, prgm_should_stop);//Child are also syntax trees
                }
                if(*prgm_should_stop){
                    printf("Unexpected EOF while parsing\n");
                    goto ERROR;
                }
                return This;
            break;
            case ']' :
                if(!from_loop)
                    printf("Error : unexpected ] while parsing\n");
                goto ERROR;//Not really an error, but it is easier to delete the tree and return NULL
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
    delete_stree(This);
    return NULL;
}

void delete_stree(STree* s){
    if(!s)
        return;
    delete_stree(s->next);
    delete_stree(s->first_child);
    //RQ : Do not delete last, which is contained in nexts or which is equal to first_child
    free(s);
}

void stree_eval(STree* s, FILE* out){
    static int i = 0;
    STree* act = s;
    //Evaluate each member of the linked list (s->next->next->...)
    while(act){
        switch(act->value){
            case LOOP :
                while(state[i])
                    stree_eval(act->first_child, out);//Evaluate loop body
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

void stree_show(STree* This){
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
        stree_show(next->first_child);
        next = next->next;
    }
    level--;
}
