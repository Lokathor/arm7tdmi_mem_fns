
extern "C" {
  pub fn libc_memcpy(d: *mut u8, s: *const u8, bytes: usize) -> *mut u8;
  pub fn aeabi_memcpy(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memcpy4(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memcpy8(d: *mut u8, s: *const u8, bytes: usize);
  pub fn gba_memcpy_sram(d: *mut u8, s: *const u8, bytes: usize);

  pub fn libc_memmove(d: *mut u8, s: *const u8, bytes: usize) -> *mut u8;
  pub fn aeabi_memmove(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memmove4(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memmove8(d: *mut u8, s: *const u8, bytes: usize);

  pub fn libc_memset(d: *mut u8, val: i32, bytes: usize) -> *mut u8;
  pub fn aeabi_memset(d: *mut u8, bytes: usize, val: i32);
  pub fn aeabi_memset4(d: *mut u8, bytes: usize, val: i32);
  pub fn aeabi_memset8(d: *mut u8, bytes: usize, val: i32);

  pub fn aeabi_memclr(d: *mut u8, bytes: usize);
  pub fn aeabi_memclr4(d: *mut u8, bytes: usize);
  pub fn aeabi_memclr8(d: *mut u8, bytes: usize);
  
  pub fn aeabi_uread4(address: *const u32) -> u32;
  pub fn aeabi_uread8(address: *const u64) -> u64;
  pub fn aeabi_uwrite4(value: u32, address: *mut u32) -> u32;
  pub fn aeabi_uwrite8(value: u64, address: *mut u64) -> u64;

  pub fn aeabi_idiv(n: i32, d: i32) -> i32;
  pub fn aeabi_uidiv(n: u32, d: u32) -> u32;
  pub fn aeabi_idivmod(n: i32, d: i32) -> u64;
  pub fn aeabi_uidivmod(n: u32, d: u32) -> u64;
}
