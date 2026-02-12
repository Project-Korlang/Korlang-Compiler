#ifndef KORLANG_UI_H
#define KORLANG_UI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef uint64_t KorlangViewId;

typedef struct KorlangView {
    KorlangViewId id;
    const char* type;
    void* state;
} KorlangView;

typedef struct KorlangEvent {
    const char* kind;
    const char* target_id;
} KorlangEvent;

// Lifecycle hooks for a View
typedef struct KorlangViewVTable {
    KorlangView (*init)(void* ctx);
    void (*render)(KorlangView* view, void* ctx);
    void (*update)(KorlangView* view, void* ctx);
    void (*dispose)(KorlangView* view, void* ctx);
} KorlangViewVTable;

#ifdef __cplusplus
}
#endif

#endif
