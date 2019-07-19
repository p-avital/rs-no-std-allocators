#include "stdlib.h"
#include "stdio.h"
#include "shared_alloc.h"

int main() {
    printf("Hello World\n");
    // rust_allocator_init_functions(&malloc, &free);
    rust_main();
    return 0;
}