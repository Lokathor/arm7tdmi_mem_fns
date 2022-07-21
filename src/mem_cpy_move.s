.global libc_memmove

libc_memmove:
    push   {r0, lr}
    bl     aeabi_memmove
    pop    {r0, lr}
    bx     lr

aeabi_memmove:
    cmp    r0, r1 @ if d < s, reverse copy
    bgt    .L_r_copy
  .L_f_copy:
  1:
    subs   r2, r2, #1
    ldrbge r3, [r1], #1
    strbge r3, [r0], #1
    bgt    1b
    bx     lr
  .L_r_copy:
    add r0, r0, r2
    add r1, r1, r2
  .L_r_copy_post_ptr_fix:
  1:
    subs   r2, r2, #1
    ldrbge r3, [r1, #-1]!
    strbge r3, [r0, #-1]!
    bgt    1b
    bx     lr
