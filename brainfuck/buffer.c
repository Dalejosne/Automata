/*
* Buffer structure for brainfuck interpretor
* Main file
* Copyright Damien Lejosne 2021, all right reserved
*/

#include <buffer.h>

Buffer* new_buffer(int capacity){
    Buffer* This = (Buffer*) malloc(sizeof(Buffer));
    if(!This)
        return NULL;
    This->value = (char*) malloc(sizeof(char)*(capacity+1));//Allocate capacity+1 bytes to put the last '\0'
    if(!This->value){
        free(This);
        return NULL;
    }
    This->capacity = capacity;
    This->ind = 0;
    This->size = 0;
    return This;
}

void delete_buffer(Buffer* This){
    free(This->value);
    free(This);
}

int buffer_push(Buffer* This, int c){
    //Do not make buffer overflow
    if(This->size==This->capacity)
        return 1;
    This->value[This->size++]=c;
    This->value[This->size]='\0';
    return 0;
}