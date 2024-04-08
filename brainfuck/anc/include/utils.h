/*
* Utils header file
* Copyright Damien Lejosne 2021, all right reserved
*/

#ifndef __UTILS_H__
    #define __UTILS_H__
    #include <stdio.h>
    #include <string.h>
    #include <buffer.h>
    /**
    * Read a character
    * @param in Input file on which you want to read the program
    * @param buffer Buffer where you want to store the read program
    * @param INTERACTIVE Are you in interactive mode, or not ?
    * @param prgm_should_stop Value of this pointer is updated by a call to this function. If set to 1,
    *                       you reached EOF and you should stop evaluating the file you are reading.
    * @return The read caracter, or -1 if EOF or maximal buffer size is reached
    */
    int read_char(FILE* in, Buffer* buffer, char INTERACTIVE, char* prgm_should_stop);
#endif
