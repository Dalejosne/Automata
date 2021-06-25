/*
* Syntax tree header file
* Copyright Damien Lejosne 2021, all right reserved
*/

#ifndef __STREE_H__
    #define __STREE_H__
    #include <buffer.h>
    #include <utils.h>
    #include <stdio.h>
    #include <stdlib.h>
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

    /**
    * First function to be called
    * Init the states of the interpretor with 0
    * @param nbStates set the len of the list used by the interpretor
    */
    void init_nb_state(int nbStates);

    void delete_stree(STree* s);

    /**
    * Create the syntax tree.
    * You can also uses it yourself if you define your own "readChar" function.
    * @param fromLoop 0 if call from the beginning, 1 if call recursively from a loop
    * @param in Input file on which you want to read the program
    * @param buffer Buffer where you want to store the read program
    * @param INTERACTIVE Are you in interactive mode, or not ?
    * @param prgmShouldStop Value of this pointer is updated by a call to this function. If set to 1,
    *                       you reached EOF and you should stop evaluating the file you are reading.
    * @return The newly created STree, or NULL
    */
    STree* new_stree(char fromLoop, FILE* in, Buffer* buffer, char INTERACTIVE, char* prgmShouldStop);

    /**
    * Evaluate the syntax tree
    * @param s Syntax tree to be evaluated
    * @param out Where the program output have to be printed
    */
    void stree_eval(STree* s, FILE* out);

    //Debug
    void stree_show(STree* This);
#endif