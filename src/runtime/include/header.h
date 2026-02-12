#ifndef KORLANG_GC_HEADER_H
#define KORLANG_GC_HEADER_H

#include <stdint.h>

typedef struct KorlangGcHeader {
    uint32_t flags;   // mark, pinned, has_finalizer, etc.
    uint16_t age;     // generational age
    uint16_t gen;     // 0 = young, 1 = old
    uint64_t size;    // payload size in bytes
    uint64_t type_id; // runtime type table index
    uint64_t aux;     // ARC/off-heap handle or aux data
} KorlangGcHeader;

enum {
    KORLANG_GC_MARKED = 1 << 0,
    KORLANG_GC_PINNED = 1 << 1,
    KORLANG_GC_FINALIZER = 1 << 2,
};

#endif
