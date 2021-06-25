/*
* Buffer structure for brainfuck interpretor
* Header file
* Copyright Damien Lejosne 2021, all right reserved
*/
#ifndef __BUFFER_H__
    #include <stdlib.h>
    #define __BUFFER_H__
    typedef struct Buffer{
        char* value;
        int capacity;
        int ind, size;
    }Buffer;

    /**
    * @param capacity The capacity of the newly created buffer
    * @return The newly created buffer, NULL if something goes wrong
    */
    Buffer* new_buffer(int capacity);

    /**
    * @param This Buffer to be deleted
    */
    void delete_buffer(Buffer* This);

    /**
    * @param This Concerned buffer
    * @param c character to be push at the end of the buffer
    * @return 0 if everythings OK, 1 if buffer capacity is excedeed
    */
    int buffer_push(Buffer* This, int c);
#endif