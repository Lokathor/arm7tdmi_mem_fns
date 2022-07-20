
.global aeabi_memmove1
.global aeabi_memcpy1
.global aeabi_memmove2
.global aeabi_memcpy2
.global aeabi_memmove4
.global aeabi_memcpy4
.global aeabi_memmove8
.global aeabi_memcpy8

@ the pointers are max aligned to 1

aeabi_memmove1:
    cmp    r0, r1
    bgt    .L_r_copy_u8
aeabi_memcpy1:
  .L_f_copy_u8:
    subs    r2, r2, #1
    ldrbcs  r3, [r1], #1
    strbcs  r3, [r0], #1
    bgt     .L_f_copy_u8
    bx      lr
  .L_r_copy_u8:
    subs    r2, r2, #1
    ldrbcs  r3, [r1, r2]
    strbcs  r3, [r0, r2]
    bgt     .L_r_copy_u8
    bx      lr

@ the pointers are max aligned to 2

aeabi_memmove2: @ this is mostly exposed for testing purposes
    cmp     r0, r1
    bgt     .L_r_copy_u16
aeabi_memcpy2:
  .L_f_copy_u16:
    subs    r2, r2, #2
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    bgt     .L_f_copy_u16
    bxeq    lr
    tst     r2, #1
    ldrbne  r3, [r1]
    strbne  r3, [r0]
    bx      lr
  .L_r_copy_u16:
    tst     r2, #1
    bne     2f
  1:
    subs    r2, r2, #2
    ldrhcs  r3, [r1, r2]
    strhcs  r3, [r0, r2]
    bgt     1b
    bx      lr
  2:
    sub     r2, r2, #1
    ldrb    r3, [r1, r2]
    strb    r3, [r0, r2]
    b       1b

@ the pointers are max aligned to 4

aeabi_memmove8:
aeabi_memmove4:
    cmp    r0, r1
    bgt    .L_r_copy_u32
aeabi_memcpy8:
aeabi_memcpy4:
    cmp    r2, #32
    bge    .L_f_copy_u32_block_work
  .L_f_copy_u32:
    subs   r2, r2, #4
    ldrcs  r3, [r1], #4
    strcs  r3, [r0], #4
    bgt    .L_f_copy_u32
    bxeq   lr
    lsls   r3, r2, #31
    ldrhcs r3, [r1], #2
    strhcs r3, [r0], #2
    ldrbmi r3, [r1], #1
    strbmi r3, [r0], #1
    bx     lr
  .L_f_copy_u32_block_work:
    push   {r4-r9}
  1:
    subs   r2, r2, #32
    ldmcs  r1!, {r3-r9, r12}
    stmcs  r0!, {r3-r9, r12}
    bgt    1b
    pop    {r4-r9}
    bxeq   lr
    add    r2, r2, #32
    b      .L_f_copy_u32
  .L_r_copy_u32:
    tst    r2, #3
    bne    2f
  1:
    subs   r2, r2, #4
    ldrcs  r3, [r1, r2]
    strcs  r3, [r0, r2]
    bgt    1b
    bx     lr
  2:
    lsls   r3, r2, #31
    submi  r2, r2, #1
    ldrbmi r3, [r1, r2]
    strbmi r3, [r0, r2]
    subcs  r2, r2, #2
    ldrhcs r3, [r1, r2]
    strhcs r3, [r0, r2]
    b      1b

