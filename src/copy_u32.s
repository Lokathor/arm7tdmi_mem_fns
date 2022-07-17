
.global copy_u32_overlapping
.global copy_u32_forward
.global copy_u32_reverse

copy_u32_overlapping:
    cmp     r0, r1
    bgt     copy_u32_reverse
    @ fallthough to copy_u32_forward

copy_u32_forward:
    cmp     r2, #32
    bge     2f
  1: @ copy less than 8 words
    subs    r2, r2, #8
    ldmcs   r1!, {r3, r12}
    stmcs   r0!, {r3, r12}
    bgt     1b
    bxeq    lr
    adds    r2, r2, #4
    ldrpl   r3, [r1], #4
    strpl   r3, [r0], #4
    bxeq    lr
    lsls    r3, r2, #31
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    ldrbmi  r3, [r1], #1
    strbmi  r3, [r0], #1
    bx      lr
  2: @ copy 8 words at a time (loop)
    push    {r4-r9}
  3:
    subs    r2, r2, #32
    ldmcs   r1!, {r3-r9, r12}
    stmcs   r0!, {r3-r9, r12}
    bgt     3b
    pop     {r4-r9}
    bxeq    lr
    adds    r2, r2, #32
    b       1b

copy_u32_reverse:
    tst     r2, #3
    bne     .L_handle_byte_and_halfword
  .L_count_is_now_a_multiple_of_four:
    add     r1, r1, r2
    add     r0, r0, r2
    tst     r2, #32
    bge     .L_block_copy_sub
  .L_done_with_block_copy:
    tst     r2, #(1<<4)
    ldmdbne r1!, {r3, r12}
    stmdbne r0!, {r3, r12}
    ldmdbne r1!, {r3, r12}
    stmdbne r0!, {r3, r12}
    lsls    r3, r2, #29
    ldmdbcs r1!, {r3, r12}
    stmdbcs r0!, {r3, r12}
    ldrmi   r3, [r1, #-4]
    strmi   r3, [r0, #-4]
    bx      lr
  .L_block_copy_sub:
    push    {r4-r9}
  1:
    subs    r2, r2, #32
    ldmdbcs r1!, {r3-r9, r12}
    stmdbcs r0!, {r3-r9, r12}
    bgt     1b
    pop     {r4-r9}
    bxeq    lr
    adds    r2, r2, #32
    b       .L_done_with_block_copy
  .L_handle_byte_and_halfword:
    lsls    r3, r2, #31
    submi   r2, r2, #1
    ldrbmi  r3, [r1, r2]
    strbmi  r3, [r0, r2]
    subcs   r2, r2, #2
    ldrhcs  r3, [r1, r2]
    strhcs  r3, [r0, r2]
    b       .L_count_is_now_a_multiple_of_four
