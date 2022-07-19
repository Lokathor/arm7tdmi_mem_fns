
core::arch::global_asm!(include_str!("mem_cpy_move.s"), options(raw));

extern "C" {
  pub fn zmemcpy(d:*mut u8, s:*const u8, count: usize)->*mut u8;
  pub fn z__aeabi_memcpy(d:*mut u8, s:*const u8, count: usize);
  pub fn z__aeabi_memcpy4(d:*mut u8, s:*const u8, count: usize);
  pub fn z__aeabi_memcpy8(d:*mut u8, s:*const u8, count: usize);

  pub fn z__aeabi_memcpy_vram(d:*mut u8, s:*const u8, count: usize);
  pub fn z__aeabi_memcpy_sram(d:*mut u8, s:*const u8, count: usize);

  pub fn zmemmove(d:*mut u8, s:*const u8, count: usize)->*mut u8;
  pub fn z__aeabi_memmove(d:*mut u8, s:*const u8, count: usize);
  pub fn z__aeabi_memmove4(d:*mut u8, s:*const u8, count: usize);
  pub fn z__aeabi_memmove8(d:*mut u8, s:*const u8, count: usize);
}
