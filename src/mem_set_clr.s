
.global libc_memset
.global aeabi_memset
.global aeabi_memset4
.global aeabi_memset8
.global aeabi_memclr
.global aeabi_memclr4
.global aeabi_memclr8

.section ".text.libc.memset"
libc_memset:
    mov    r3, r2
    mov    r2, r1
    mov    r1, r3
    push   {r0, lr}
    bl     aeabi_memset
    pop    {r0, lr}
    bx     lr
.previous

.section ".text.aeabi.memset"
aeabi_memclr8:
aeabi_memclr4:
    mov    r2, #0
    mov    r3, #0
    b      .L_check_for_block_work

aeabi_memclr:
    mov    r2, #0
aeabi_memset8:
aeabi_memset4:
aeabi_memset: @ r0(dest), r1(count), r2(byte)
    @ duplicate the byte across all of r2 and r3
    and    r2, r2, #0xFF
    orr    r2, r2, r2, lsl #8
    orr    r2, r2, r2, lsl #16
    mov    r3, r2
    @ for 'sets' too small to fixup we just byte loop
    cmp    r1, #3
    ble    .L_byte_loop
    @ carry/sign test on the address, then do fixup
    lsls   r12, r0, #31
    submi  r1, r1, #1
    strbmi r2, [r0], #1
    subcs  r1, r1, #2
    strhcs r2, [r0], #2
  .L_check_for_block_work:
    cmp    r1, #32
    bge    .L_block_work
  .L_post_block_work:
    @ set 4 words
    tst    r1, #0b10000
    stmne  r0!, {r2, r3}
    stmne  r0!, {r2, r3}
    @ set 2 and/or 1 words
    lsls   r12, r1, #29
    stmcs  r0!, {r2, r3}
    strmi  r2, [r0], #4
    @ set halfword and/or byte
    lsls   r12, r1, #31
    strhcs r2, [r0], #2
    strbmi r2, [r0], #1
    bx     lr

  .L_block_work:
    push   {r4-r9}
    mov    r4, r2
    mov    r5, r2
    mov    r6, r2
    mov    r7, r2
    mov    r8, r2
    mov    r9, r2
  1:
    subs   r1, r1, #32
    stmge  r0!, {r2-r9}
    bgt    1b
    pop    {r4-r9}
    bxeq   lr
    b      .L_post_block_work

  .L_byte_loop:
    subs   r1, r1, #1
    strbcs r2, [r0], #1
    bgt    .L_byte_loop
    bx     lr
.previous
