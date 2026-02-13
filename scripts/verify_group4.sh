#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pass() { printf '[group4] PASS %s\n' "$1"; }
fail() { printf '[group4] FAIL %s\n' "$1" >&2; exit 1; }
req_file() { [[ -f "$1" ]] || fail "$2 (missing: $1)"; }
req_rg() { rg -n "$1" "$2" >/dev/null || fail "$3"; }

# O.4.1 .. O.4.9 runtime checks
cargo test --manifest-path "$ROOT/src/runtime/Cargo.toml" o4_ -- --nocapture
pass "O.4.1"
pass "O.4.2"
pass "O.4.3"
pass "O.4.4"
pass "O.4.5"
pass "O.4.6"
pass "O.4.7"
pass "O.4.8"
pass "O.4.9"

# O.4.1 WGPU instance without external headers
req_file "$ROOT/src/runtime/src/ui/render/mod.rs" "O.4.1"
req_rg 'struct WgpuInstance' "$ROOT/src/runtime/src/ui/render/mod.rs" "O.4.1 missing WgpuInstance"
if rg -n '#include.*wgpu|bindgen|clang' "$ROOT/src/runtime/src/ui/render/mod.rs" >/dev/null; then
  fail "O.4.1 found external header/bindgen usage"
fi
pass "O.4.1-header-check"

# O.4.2 Surface config (swapchain)
req_rg 'struct SurfaceConfig' "$ROOT/src/runtime/src/ui/render/mod.rs" "O.4.2 missing SurfaceConfig"
req_rg 'frame_count' "$ROOT/src/runtime/src/ui/render/mod.rs" "O.4.2 missing frame_count"
pass "O.4.2-structure"

# O.4.3 Render pipeline create/bind
req_rg 'struct RenderPipeline' "$ROOT/src/runtime/src/ui/render/mod.rs" "O.4.3 missing RenderPipeline"
req_rg 'fn bind\(&mut self\)' "$ROOT/src/runtime/src/ui/render/mod.rs" "O.4.3 missing bind"
pass "O.4.3-structure"

# O.4.4 Shader JIT path to SPIR-V / Metal
req_file "$ROOT/src/runtime/korlang/graphics/shader_jit.kor" "O.4.4"
req_rg 'fun jit_compile' "$ROOT/src/runtime/korlang/graphics/shader_jit.kor" "O.4.4 missing jit_compile"
req_rg 'emit_spirv' "$ROOT/src/runtime/korlang/graphics/shader_jit.kor" "O.4.4 missing SPIR-V emission"
req_rg 'emit_metal' "$ROOT/src/runtime/korlang/graphics/shader_jit.kor" "O.4.4 missing Metal emission"
pass "O.4.4-structure"

# O.4.5 Glyph atlas generation
req_file "$ROOT/src/runtime/src/ui/glyph.rs" "O.4.5"
req_rg 'struct GlyphAtlas' "$ROOT/src/runtime/src/ui/glyph.rs" "O.4.5 missing GlyphAtlas"
req_rg 'fn insert\(' "$ROOT/src/runtime/src/ui/glyph.rs" "O.4.5 missing atlas insert"
pass "O.4.5-structure"

# O.4.6 Scenegraph diff + patch
req_rg 'pub fn diff\(' "$ROOT/src/runtime/src/ui/mod.rs" "O.4.6 missing diff"
req_rg 'pub fn apply_diff\(' "$ROOT/src/runtime/src/ui/mod.rs" "O.4.6 missing apply_diff"
pass "O.4.6-structure"

# O.4.7 Event loop latency
req_file "$ROOT/src/runtime/src/ui/event_loop.rs" "O.4.7"
req_rg 'struct UiEventLoop' "$ROOT/src/runtime/src/ui/event_loop.rs" "O.4.7 missing UiEventLoop"
req_rg 'Duration::from_millis\(16\)' "$ROOT/src/runtime/src/ui/event_loop.rs" "O.4.7 missing 16ms target"
pass "O.4.7-structure"

# O.4.8 Flex/Grid layout
req_file "$ROOT/src/runtime/src/ui/layout.rs" "O.4.8"
req_rg 'compute_flex_row' "$ROOT/src/runtime/src/ui/layout.rs" "O.4.8 missing flex layout"
req_rg 'compute_grid' "$ROOT/src/runtime/src/ui/layout.rs" "O.4.8 missing grid layout"
pass "O.4.8-structure"

# O.4.9 PNG/JPG decode in Korlang
req_file "$ROOT/src/runtime/korlang/graphics/image_decode.kor" "O.4.9"
req_rg 'fun decode_png_header' "$ROOT/src/runtime/korlang/graphics/image_decode.kor" "O.4.9 missing PNG decode"
req_rg 'fun decode_jpg_header' "$ROOT/src/runtime/korlang/graphics/image_decode.kor" "O.4.9 missing JPG decode"
pass "O.4.9-korlang"

# O.4.10 Native window creation across platforms
req_file "$ROOT/src/runtime/korlang/graphics/windowing.kor" "O.4.10"
req_rg 'fun x11_or_wayland_create' "$ROOT/src/runtime/korlang/graphics/windowing.kor" "O.4.10 missing linux window path"
req_rg 'fun appkit_create' "$ROOT/src/runtime/korlang/graphics/windowing.kor" "O.4.10 missing macOS window path"
req_rg 'fun win32_create' "$ROOT/src/runtime/korlang/graphics/windowing.kor" "O.4.10 missing Windows window path"
pass "O.4.10"

printf '\n[group4] all Group 4 checks passed\n'
