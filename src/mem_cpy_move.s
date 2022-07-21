
.global aeabi_memmove1
.global aeabi_memcpy1
.global aeabi_memmove2
.global aeabi_memcpy2
.global aeabi_memmove4
.global aeabi_memcpy4
.global aeabi_memmove8
.global aeabi_memcpy8
.global aeabi_memmove
.global aeabi_memcpy
.global libc_memmove
.global libc_memcpy

@ the pointers are co-aligned to 1

aeabi_memmove1:
    cmp    r0, r1
    bgt    .L_r_copy_u8
aeabi_memcpy1:
  .L_f_copy_u8:
  1:
    subs    r2, r2, #1
    ldrbcs  r3, [r1], #1
    strbcs  r3, [r0], #1
    bgt     1b
    bx      lr
  .L_r_copy_u8:
    add     r0, r0, r2
    add     r1, r1, r2
  .L_r_copy_u8_post_add:
  1:
    subs    r2, r2, #1
    ldrbcs  r3, [r1, #-1]!
    strbcs  r3, [r0, #-1]!
    bgt     1b
    bx      lr

@ the pointers are co-aligned to 2

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
    add     r0, r0, r2
    add     r1, r1, r2
  .L_r_copy_u16_post_add:
    tst     r0, #1
    bne     2f
  1:
    subs    r2, r2, #2
    ldrhcs  r3, [r1, #-2]!
    strhcs  r3, [r0, #-2]!
    bgt     1b
    bxeq    lr
    tst     r2, #1
    ldrbne  r3, [r1, #-1]!
    strbne  r3, [r0, #-1]!
    bx      lr
  2:
    sub     r2, r2, #1 @ count is odd to get here, so this won't go below 0
    ldrb    r3, [r1, #-1]!
    strb    r3, [r0, #-1]!
    b       1b

@ the pointers are co-aligned to 4

aeabi_memmove8:
aeabi_memmove4:
    cmp    r0, r1
    bgt    .L_r_copy_u32
aeabi_memcpy8:
aeabi_memcpy4:
    cmp    r2, #32
    bge    .L_f_copy_u32_block_work
  .L_f_copy_u32:
    @ copy 4 words
    tst    r2, #0b10000
    ldmne  r1!, {r3, r12}
    stmne  r0!, {r3, r12}
    ldmne  r1!, {r3, r12}
    stmne  r0!, {r3, r12}
    bics   r2, r2, #0b10000
    bxeq   lr
    @ copy 2 and/or 1 words
    lsls   r3, r2, #29
    ldmcs  r1!, {r3, r12}
    stmcs  r0!, {r3, r12}
    ldrmi  r3, [r1], #4
    strmi  r3, [r0], #4
    bics   r2, r2, #0b1100
    bxeq   lr
    @ copy halfword and/or byte
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
    b      .L_f_copy_u32
  .L_r_copy_u32:
    tst    r2, #3
    bne    .L_r_copy_u32_byte_and_halfword_copy
  .L_r_copy_u32_post_byte_and_halfword_copy:
    add    r0, r0, r2
    add    r1, r1, r2
    cmp    r2, #32
    bge    .L_r_copy_u32_block_work
  .L_r_copy_u32_lt32_bytes:
    @ copy 4 words
    tst    r2, #0b10000
    ldmdbne r1!, {r3, r12}
    stmdbne r0!, {r3, r12}
    ldmdbne r1!, {r3, r12}
    stmdbne r0!, {r3, r12}
    bics   r2, r2, #0b10000
    bxeq   lr
    @ copy 2 and/or 1 words
    lsls   r3, r2, #29
    ldmdbcs r1!, {r3, r12}
    stmdbcs r0!, {r3, r12}
    ldrmi  r3, [r1, #-4]!
    strmi  r3, [r0, #-4]!
    bx     lr
  .L_r_copy_u32_block_work:
    push   {r4-r9}
  1:
    subs   r2, r2, #32
    ldmdbcs r1!, {r3-r9, r12}
    stmdbcs r0!, {r3-r9, r12}
    bgt    1b
    pop    {r4-r9}
    bxeq   lr
    add    r2, r2, #32
    b      .L_r_copy_u32_lt32_bytes
  .L_r_copy_u32_byte_and_halfword_copy:
    lsls   r3, r2, #31
    submi  r2, r2, #1
    ldrbmi r3, [r1, r2]
    strbmi r3, [r0, r2]
    subcs  r2, r2, #2
    ldrhcs r3, [r1, r2]
    strhcs r3, [r0, r2]
    b      .L_r_copy_u32_post_byte_and_halfword_copy

@ the co-alignment is unknown, and must be detected for.

libc_memmove:
    push   {r0, lr}
    bl     aeabi_memmove
    pop    {r0, lr}
    bx     lr

libc_memcpy:
    push   {r0, lr}
    bl     aeabi_memcpy
    pop    {r0, lr}
    bx     lr

aeabi_memmove:
    cmp    r0, r1
    bgt    .L_r_copy_gain_alignment
aeabi_memcpy:
  .L_f_copy_gain_alignment:
    eor    r12, r0, r1
    lsls   r3, r12, #31
    @ max align 1
    bmi    .L_f_copy_u8
    @ max align 2
    bcs    1f
    @ max align 4
    tst    r0, #3
    beq    .L_f_copy_u32
    cmp    r2, #8 @ if bytes < 8, go ahead and byte copy
    blt    .L_f_copy_u8
    lsls   r3, r0, #31
    submi  r2, r2, #1
    ldrbmi r3, [r1], #1
    strbmi r3, [r0], #1
    subcs  r2, r2, #2
    ldrhcs r3, [r1], #2
    strhcs r3, [r0], #2
    b      .L_f_copy_u32
  1:
    tst    r0, #1
    beq    .L_f_copy_u16
    subs   r2, r2, #1
    ldrb   r3, [r1], #1
    strb   r3, [r0], #1
    b      .L_f_copy_u16
  .L_r_copy_gain_alignment:
    @ if the alignment is bad we can't fix it on a reverse copy,
    @ but we can still check, and we might be aligned already
    eor    r12, r0, r1
    lsls   r3, r12, #31
    @ max align 1
    bmi    .L_r_copy_u8
    @ max align 2
    bmi    1f
    @ max align 4
    tst    r0, #3
    beq    .L_r_copy_u32
    @ no possible fix
    b      .L_r_copy_u8
  1:
    tst    r0, #1
    beq    .L_r_copy_u16
    @ no possible fix
    b      .L_r_copy_u8
