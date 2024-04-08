/*
* Utils main code
* Copyright Damien Lejosne 2021, all right reserved
*/

#include <utils.h>

int read_char(FILE* in, Buffer* buffer, char INTERACTIVE, char* prgm_should_stop){
    static char file_read=0;
    if(!file_read && !INTERACTIVE){
        file_read=1;
        buffer->ind=0;
        int c = 0;
        while(c != EOF){
            c = getc(in);
            if(c=='>' || c=='<' || c=='+' || c=='-' || c=='[' || c==']' || c==',' || c=='.')
                if(buffer_push(buffer, c) ){
                    printf("Warning : Max file size excedeed !\n");
                    //Stop reading the file if capacity excedeed
                    goto GET_ENTRY;
                }
        }
    }else if(buffer->size==buffer->ind && !INTERACTIVE){
        *prgm_should_stop=1;
        return -1;
    }else if((buffer->value[buffer->ind]=='\0' || buffer->ind==0) && INTERACTIVE){
        printf("> ");
        buffer->ind=0;
        fgets(buffer->value, 10000, in);
        if(!strchr(buffer->value, '\n')){
            int c = 0;
            //Free input buffer.
            while (c != '\n' && c != EOF)
                c = getc(in);
        }
    }
GET_ENTRY:
    int c = buffer->value[buffer->ind];
    buffer->ind++;
    return c;
}
