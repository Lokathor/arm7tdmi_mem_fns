
@ function names deliberately avoid the "actual" names to work with the test suite.
@ when using this code in a project, remove the leading `z` with search and replace.

.global zmemcpy
.global z__aeabi_memcpy
.global z__aeabi_memcpy4
.global z__aeabi_memcpy8

.global z__aeabi_memcpy_vram
.global z__aeabi_memcpy_sram

.global zmemmove
.global z__aeabi_memmove
.global z__aeabi_memmove4
.global z__aeabi_memmove8

@
@ MOVE
@

zmemmove:
    push   {r0, lr}
    bl     z__aeabi_memmove
    pop    {r0, lr}
    bx     lr

z__aeabi_memmove4:
z__aeabi_memmove8:
    cmp    r0, r1
    bgt    z__aeabi_memcpy4_reverse
    b      z__aeabi_memcpy4

@ Note: unaligned aeabi memmove is in the forward copy code

@
@ FORWARD COPY
@

zmemcpy:
    push    {r0, lr}
    bl      z__aeabi_memcpy
    pop     {r0, lr}
    bx      lr

z__aeabi_memmove:
    cmp     r0, r1
    bgt     z__aeabi_memcpy_reverse
    @ fallthrough
z__aeabi_memcpy:
    eor     r12, r0, r1
    lsls    r12, r12, #31
    @ max align 1
    bmi     z__aeabi_memcpy_sram
    @ max align 2
    bcs     .L_align2_forward_fixup
    @ max align 4
    tst     r0, #3
    bne     .L_align4_forward_fixup
    @ fallthrough
z__aeabi_memcpy8:
z__aeabi_memcpy4:
    cmp     r2, #32
    bge     .L_block_forward_entry
  .L_block_forward_after:
    @ check for 4 word copy
    subs    r2, r2, #16
    ldmcs   r1!, {r3, r12}
    stmcs   r0!, {r3, r12}
    ldmcs   r1!, {r3, r12}
    stmcs   r0!, {r3, r12}
    bxeq    lr @ early return
    @ carry/sign for 2 word and/or 1 word copy
    lsls    r3, r2, #29
    ldmcs   r1!, {r3, r12}
    stmcs   r0!, {r3, r12}
    ldrmi   r3, [r1], #4
    strmi   r3, {r0], #4
    @ carry/sign for halfword and/or byte copy
    lsls    r3, r2, #31
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    ldrbmi  r3, [r1], #1
    ldrbmi  r3, [r0], #1
    bx      lr
  .L_block_forward_entry:
    push    {r4-r9}
  1:
    subs    r2, r2, #32
    ldmcs   r1!, {r3-r9, r12}
    stmcs   r0!, {r3-r9, r12}
    bgt     1b
    pop     {r4-r9}
    bxeq    lr @ early return
    b       .L_block_forward_after
  .L_align4_forward_fixup:
    lsls    r12, r0, #31
    @ if current align 1, advance by 1
    submi   r2, r2, #1
    ldrbmi  r3, [r1], #1
    ldrbmi  r3, [r0], #1
    @ if current align 2, advance by 2
    subcs   r2, r2, #2
    ldrhcs  r3, [r1], #2
    ldrhcs  r3, [r0], #2
    b       z__aeabi_memcpy4

.L_align2_forward_fixup:
    tst     r0, #1
    ldrbne  r3, [r1], #1
    ldrbne  r3, [r0], #1
z__aeabi_memcpy_vram:
    subs    r2, r2, #2
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    bgt     z__aeabi_memcpy_vram
    bxeq    lr
    adds    r2, r2, #1
    ldrbeq  r3, [r1]
    strbeq  r3, [r0]
    bx      lr

z__aeabi_memcpy_sram:
    subs    r2, r2, #1
    ldrbcs  r3, [r1], #1
    strbcs  r3, [r0], #1
    bgt     z__aeabi_memcpy_sram
    bx      lr

@
@ COPY REVERSE
@

.L_aeabi_memcpy_reverse:
    eor    r12, r0, r1
    lsls   r12, r12, #31
    @ max align 1
    bmi    .L_memcpy_sram_reverse
    @ max align 2
    bcs    .L_align2_reverse_fixup
    @ max align 4
    /* If we have to reverse copy, and we're unaligned, rip */
    tst    r0, #3
    bne    .L_memcpy_sram_reverse
    @ fallthrough
.L__aeabi_memcpy4_reverse:
    tst     r2, #3
    bne     .L_align4r_handle_byte_and_halfword
  .L_align4r_count_is_now_a_multiple_of_four:
    add     r1, r1, r2
    add     r0, r0, r2
    tst     r2, #32
    bge     .L_block_copy_sub
  .L_align4r_done_with_block_copy:
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
  .L_align4r_block_copy_sub:
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
  .L_align4r_handle_byte_and_halfword:
    lsls    r3, r2, #31
    submi   r2, r2, #1
    ldrbmi  r3, [r1, r2]
    strbmi  r3, [r0, r2]
    subcs   r2, r2, #2
    ldrhcs  r3, [r1, r2]
    strhcs  r3, [r0, r2]
    b       .L_align4r_count_is_now_a_multiple_of_four

.L_align2_reverse_fixup:
    /* If we have to reverse copy and we're unaligned, rip */
    tst    r0, #1
    bne    .L_memcpy_sram_reverse
.L_memcpy_vram_reverse:
    tst    r2, #1
    bne    2f @ handle tail byte, if any
  1:
    subs   r2, r2, #2
    ldrhcs r3, [r1, r2]
    strhcs r3, [r0, r2]
    bgt    1b
    bx     lr
  2:
    sub    r2, r2, #1
    ldrb   r2, [r1, r2]
    strb   r2, [r0, r2]
    b      1b

.L_memcpy_sram_reverse:
    subs   r2, r2, #1
    ldrbcs r3, [r1, r2]
    strbcs r3, [r0, r2]
    bgt    .L_memcpy_sram_reverse
    bx     lr
