#include <stdio.h>
#include "krabcake.h"

int main() {
    printf("Hello world!\n");

    char val = (char)1;
    char *x = KC_BORROW_MUT(char, val); // x = &mut val;
    char *y = KC_BORROW_MUT(char, *x);

    printf("before *y = 5, val: %d\n", val);
    *y = 5;
    printf("after *y = 5, val: %d\n", val);

    printf("before *x = 3, val: %d\n", val);
    // Write through a pointer aliasing `y`
    *x = 3;
    printf("after *x = 3, val: %d\n", val);

    char end = *y;

    printf("Goodbye world, end: %d!\n", end);
}
