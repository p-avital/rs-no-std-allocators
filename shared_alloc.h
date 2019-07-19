#include <stdint.h>

typedef void *(*MallocFunction)(uintptr_t);

typedef void (*FreeFunction)(void*);

void rust_allocator_set_alloc_functions(MallocFunction malloc, FreeFunction free);

void rust_main(uintptr_t n);
