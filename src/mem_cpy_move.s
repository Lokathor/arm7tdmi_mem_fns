libc_memmove:
    push   {r0, lr}
    bl     aeabi_memmove
    pop    {r0, lr}
    bx     lr

aeabi_memmove:
    cmp    r0, r1 @ if d > s, reverse copy
    bgt    .L_r_copy_gain_align
aeabi_memcpy:
  .L_f_copy_gain_align:
    eor    r12, r0, r1
    lsls   r3, r12, #31
    bmi    .L_f_copy_max_coalign1
    bcs    .L_f_copy_max_coalign2
    @ else fallthrough

  .L_f_copy_max_coalign4:
    @ todo
  .L_f_copy_coalign4_assured:
    @ todo
  .L_f_copy_fixup4:
    @ todo

  .L_f_copy_max_coalign2:
    tst     r0, #1
    bne     .L_f_copy_fixup2
  .L_f_copy_coalign2_assured:
    @ todo
  .L_f_copy_fixup2:
    @ todo
    b       .L_f_copy_coalign2_assured

  .L_f_copy_max_coalign1:
  1:
    subs    r2, r2, #1
    ldrbge  r3, [r1], #1
    strbge  r3, [r0], #1
    bgt     1b
    bx      lr

  .L_r_copy_gain_align:
    add     r0, r0, r2
    add     r1, r1, r2
    eor     r12, r0, r1
    lsls    r3, r12, #31
    bmi     .L_r_copy_max_coalign1
    bcs     .L_r_copy_max_coalign2
    @ else fallthrough
  .L_r_copy_max_coalign4:
    @ todo
  .L_r_copy_max_coalign2:
    @ todo
  .L_r_copy_max_coalign1:
  1:
    subs    r2, r2, #1
    ldrbge  r3, [r1, #-1]!
    strbge  r3, [r0, #-1]!
    bgt     1b
    bx      lr
