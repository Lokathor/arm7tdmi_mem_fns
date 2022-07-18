
.global copy_u16_overlapping
.global copy_u16_forward
.global copy_u16_reverse

copy_u16_overlapping:
    cmp     r0, r1
    bgt     copy_u16_reverse
    @ fallthough to copy_u16_forward

copy_u16_forward:
    subs    r2, r2, #2
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    bgt     copy_u16_forward
    bxeq    lr
    adds    r2, r2, #1
    ldrbeq  r3, [r1]
    strbeq  r3, [r0]
    bx      lr

copy_u16_reverse:
    tst     r2, #1
    bne     2f
  1:
    subs    r2, r2, #2
    ldrhcs  r3, [r1, r2]
    strhcs  r3, [r0, r2]
    bgt     1b
    bxeq    lr
  2:
    sub     r2, r2, #1
    ldrb    r3, [r1, r2]
    strb    r3, [r0, r2]
    b       1b
