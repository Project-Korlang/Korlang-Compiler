// AArch64 context switch skeleton (callee-saved register preservation)
// void korlang_ctx_switch(void** from_sp, void* to_sp);

.global korlang_ctx_switch
.type korlang_ctx_switch, %function
korlang_ctx_switch:
  // Save callee-saved regs x19-x29 and lr on current stack.
  stp x29, x30, [sp, #-16]!
  stp x27, x28, [sp, #-16]!
  stp x25, x26, [sp, #-16]!
  stp x23, x24, [sp, #-16]!
  stp x21, x22, [sp, #-16]!
  stp x19, x20, [sp, #-16]!

  // Store old SP and switch.
  str sp, [x0]
  mov sp, x1

  // Restore from target stack.
  ldp x19, x20, [sp], #16
  ldp x21, x22, [sp], #16
  ldp x23, x24, [sp], #16
  ldp x25, x26, [sp], #16
  ldp x27, x28, [sp], #16
  ldp x29, x30, [sp], #16
  ret
