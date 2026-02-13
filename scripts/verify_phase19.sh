#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_file() {
  [[ -f "$1" ]] || { echo "missing file: $1" >&2; exit 1; }
}

require_symbol() {
  rg -n "$1" "$2" >/dev/null || { echo "missing symbol '$1' in $2" >&2; exit 1; }
}

# 19.1
require_file "$ROOT/src/runtime/korlang/media/graph.kor"
require_file "$ROOT/src/runtime/korlang/media/audio_backend.kor"
require_file "$ROOT/src/runtime/korlang/media/video_pipeline.kor"
require_symbol "fun Graph_connect_sidechain" "$ROOT/src/runtime/korlang/media/graph.kor"
require_symbol "fun async_sample_rate_convert" "$ROOT/src/runtime/korlang/media/graph.kor"
require_symbol "fun linux_open" "$ROOT/src/runtime/korlang/media/audio_backend.kor"
require_symbol "fun coreaudio_open" "$ROOT/src/runtime/korlang/media/audio_backend.kor"
require_symbol "fun wasapi_open" "$ROOT/src/runtime/korlang/media/audio_backend.kor"
require_symbol "fun yuv_to_rgb_zero_copy" "$ROOT/src/runtime/korlang/media/video_pipeline.kor"
require_symbol "fun vaapi_open" "$ROOT/src/runtime/korlang/media/video_pipeline.kor"
require_symbol "fun videotoolbox_open" "$ROOT/src/runtime/korlang/media/video_pipeline.kor"

# 19.2
require_file "$ROOT/src/runtime/korlang/graphics/driver_bindings.kor"
require_file "$ROOT/src/runtime/korlang/graphics/shader_jit.kor"
require_file "$ROOT/src/runtime/korlang/graphics/windowing.kor"
require_symbol "fun open_driver" "$ROOT/src/runtime/korlang/graphics/driver_bindings.kor"
require_symbol "fun jit_compile" "$ROOT/src/runtime/korlang/graphics/shader_jit.kor"
require_symbol "fun window_create" "$ROOT/src/runtime/korlang/graphics/windowing.kor"

# 19.3
require_file "$ROOT/src/runtime/korlang/debug/symbols.kor"
require_file "$ROOT/src/runtime/src/stdio.rs"
require_file "$ROOT/src/tools/cli/src/main.rs"
require_symbol "korlang_io_println_i64" "$ROOT/src/runtime/src/stdio.rs"
require_symbol "fn split_run_args" "$ROOT/src/tools/cli/src/main.rs"
require_symbol "\"repl\"" "$ROOT/src/tools/cli/src/main.rs"
require_symbol "fn print_diags" "$ROOT/src/tools/cli/src/main.rs"
require_symbol "LineBuffer" "$ROOT/src/runtime/korlang/stdlib/io.kor"
require_symbol "toString" "$ROOT/src/stdlib/core/io.kor"

echo "phase19 verification: ok"
