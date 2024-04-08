#ifndef __BF_H__
#define __BF_H__

/**
 * Error enum.
 */
typedef enum BF_Error {
    /// Parse error : Unexpected ']' (without previous '[')
    BF_LPARSE_ERROR,
    /// Parse error : Unended '['.
    BF_RPARSE_ERROR,
    /// No error.
    BF_NO_ERROR,
} BF_Error;
#ifdef __BF_IMPLEMENTATION__
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int* compute_jmp_table(char* code, int code_size, BF_Error *error) {
    int opening = 0;
    int* pc_stack = (int*) malloc(sizeof(int) * code_size / 2); // worst case : buffer_size / 2 opening [
    int* jmp_table = (int*) malloc(sizeof(int) * code_size);
    for(int pc = 0; pc < code_size; pc++) {
        if(code[pc] == '[') pc_stack[opening++] = pc;
        else if(code[pc] == ']') {
            if(opening == 0) {
                free(pc_stack); free(jmp_table); *error = BF_LPARSE_ERROR;
                return NULL;
            }
            jmp_table[pc] = pc_stack[--opening]; // be aware of the --opening !
            jmp_table[pc_stack[opening]] = pc;
        }
    }
    free(pc_stack);
    if(opening != 0) {
        free(jmp_table); *error = BF_RPARSE_ERROR;
        return NULL;
    }
    return jmp_table;
}
BF_Error eval(char* code, int* a_states, int states_count, int* p_state) {
    int code_size = strlen(code);
    BF_Error error = BF_NO_ERROR;
    int *jmp_table = compute_jmp_table(code, code_size, &error);
    if(!jmp_table) return error;
    int pos = *p_state;
    for(int pc = 0; pc < code_size; pc++) {
        switch(code[pc]) {
            case '+': a_states[pos]++; break;
            case '-': a_states[pos]--; break;
            case '>':
                if(pos == states_count - 1) pos = 0;
                else pos++;
            break;
            case '<':
                if(pos == 0) pos = states_count - 1;
                else pos--;
            break;
            case '[':
                if(!a_states[pos]) {
                    pc = jmp_table[pc]; // No +1 because incremented, we jump **after** the next ]
                }
            break;
            case ']':
                pc = jmp_table[pc] - 1; // -1 because incremented at end of loop
            break;
            case '.' : fputc(a_states[pos], stdout); break;
            case ';' : a_states[pos]=fgetc(stdin); break;
        }
    }
    *p_state = pos;
    free(jmp_table);
    return error;
}
#else
/**
 *
 * \brief Evaluate a brainfuck source code.
 * \param code Brainfuck source code to be evaluate. It **must** be a C String.
 * \param a_states An array of states previously initialised with 0s, or containing the state of a previous execution you want to continue.
 *        It will be use as a circular array : If you do a < or a > which make you access out of the bound of the buffer, you will be
 *        redirect to the end (resp. the begining) of the array.
 * \param state_count the number of states the array.
 * \param p_state A pointer on an integer representing the current state position, it will be updated by eval.
 * \return An error if one occured.
 *
 */
BF_Error eval(char* code, int* a_states, int states_count, int* p_state);
#endif
#endif
