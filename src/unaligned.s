
.global aeabi_uread4
.global aeabi_uwrite4
.global aeabi_uread8
.global aeabi_uwrite8

.section ".text.aeabi.uread4"
aeabi_uread4:
    ldrb r1, [r0], #1
    ldrb r2, [r0], #1
    ldrb r3, [r0], #1
    ldrb r12, [r0], #1
    orr  r0, r1, r2, lsl #8
    orr  r0, r0, r3, lsl #16
    orr  r0, r0, r12, lsl #24
    bx   lr
.previous
