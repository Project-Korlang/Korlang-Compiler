// x86_64 context switch skeleton (callee-saved register preservation)
// void korlang_ctx_switch(void** from_sp, void* to_sp);

.global korlang_ctx_switch
.type korlang_ctx_switch, @function
korlang_ctx_switch:
  // Save callee-saved registers on current stack.
  push %rbx
  push %rbp
  push %r12
  push %r13
  push %r14
  push %r15

  // Store old stack pointer.
  mov %rsp, (%rdi)
  // Switch to new stack pointer.
  mov %rsi, %rsp

  // Restore callee-saved registers from target stack.
  pop %r15
  pop %r14
  pop %r13
  pop %r12
  pop %rbp
  pop %rbx
  ret
