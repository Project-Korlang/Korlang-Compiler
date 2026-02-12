// x86_64 syscall stub (System V ABI)
// u64 korlang_syscall(u64 n, u64 a1, u64 a2, u64 a3, u64 a4, u64 a5, u64 a6)

.global korlang_syscall
.type korlang_syscall, @function
korlang_syscall:
  // TODO: wire to real syscall instruction
  xor %rax, %rax
  ret
