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
	*
    * Equiv to :
	*
    *   This->first_child------------->Another tree...
	*
    *   |       |
	*
    *   |     next------------->Another tree...
	*
    *   |       |
	*
    *   |---->next (last_child)
    */
    typedef struct Tree{
        struct Tree* next;
        struct Tree* first_child;
        struct Tree* last_child;
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
    * @param from_loop 0 if call from the beginning, 1 if call recursively from a loop
    * @param in Input file on which you want to read the program
    * @param buffer Buffer where you want to store the read program
    * @param INTERACTIVE Are you in interactive mode, or not ?
    * @param prgm_should_stop Value of this pointer is updated by a call to this function. If set to 1,
    *                       you reached EOF and you should stop evaluating the file you are reading.
    * @return The newly created STree, or NULL
    */
    STree* new_stree(char from_loop, FILE* in, Buffer* buffer, char INTERACTIVE, char* prgm_should_stop);

    /**
    * Evaluate the syntax tree
    * @param s Syntax tree to be evaluated
    * @param out Where the program output have to be printed
    */
    void stree_eval(STree* s, FILE* out);

    //Debug
    void stree_show(STree* This);
#endif
