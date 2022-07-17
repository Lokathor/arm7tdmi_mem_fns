
.global copy_u8_overlapping
.global copy_u8_forward
.global copy_u8_reverse

copy_u8_overlapping:
    cmp     r0, r1
    bgt     copy_u8_reverse
    @ fallthough to copy_u8_forward

copy_u8_forward:
    subs    r2, r2, #1
    ldrbcs  r3, [r1], #1
    strbcs  r3, [r0], #1
    bgt     copy_u8_forward
    bx      lr

copy_u8_reverse:
    subs    r2, r2, #1
    ldrbcs  r3, [r1, r2]
    strbcs  r3, [r0, r2]
    bgt     copy_u8_reverse
    bx      lr
