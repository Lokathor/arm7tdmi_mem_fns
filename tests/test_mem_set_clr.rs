#![allow(dead_code)]

core::arch::global_asm!(include_str!("../src/mem_set_clr.s"), options(raw));
