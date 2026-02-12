# Smart-GC Object Header Design

This document defines the object header layout used by the GC runtime.

## Header Layout (64-bit)
```
struct GcHeader {
  u32 flags;      // bit flags: mark, pinned, finalizer, etc.
  u32 type_id;    // runtime type table index
  u64 size;       // payload size in bytes
  u64 rc_or_aux;  // refcount or aux field (tier 3 / ARC)
};
```

## Flags
- `0x1` mark bit
- `0x2` pinned (FFI)
- `0x4` has_finalizer
- `0x8` old_gen

## Notes
- Header precedes object payload in memory.
- `type_id` resolves to metadata for tracing (field offsets).
- `rc_or_aux` used for off-heap buffers and ARC handles.

