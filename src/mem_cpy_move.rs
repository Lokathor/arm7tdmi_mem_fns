
core::arch::global_asm!(include_str!("mem_cpy_move.s"), options(raw));

extern "C" {
  /// exclusive regions. returns the dest pointer.
  pub fn zmemcpy(d:*mut u8, s:*const u8, count: usize)->*mut u8;
  /// exclusive regions.
  pub fn z__aeabi_memcpy(d:*mut u8, s:*const u8, count: usize);
  /// exclusive regions. pointers must both be aligned to 4.
  pub fn z__aeabi_memcpy4(d:*mut u8, s:*const u8, count: usize);
  /// exclusive regions. pointers must both be aligned to 8.
  pub fn z__aeabi_memcpy8(d:*mut u8, s:*const u8, count: usize);

  /// exclusive regions, pointers must align to 2.
  /// * if count is even then only halfword accesses will be used.
  pub fn z__aeabi_memcpy_vram(d:*mut u8, s:*const u8, count: usize);
  /// exclusive regions.
  /// * only byte accesses will be used.
  pub fn z__aeabi_memcpy_sram(d:*mut u8, s:*const u8, count: usize);

  /// overlapping regions. returns the dest pointer.
  pub fn zmemmove(d:*mut u8, s:*const u8, count: usize)->*mut u8;
  /// overlapping regions.
  pub fn z__aeabi_memmove(d:*mut u8, s:*const u8, count: usize);
  /// overlapping regions. pointers must both be aligned to 4.
  pub fn z__aeabi_memmove4(d:*mut u8, s:*const u8, count: usize);
  /// overlapping regions. pointers must both be aligned to 8.
  pub fn z__aeabi_memmove8(d:*mut u8, s:*const u8, count: usize);
}
