#![allow(dead_code)]

core::arch::global_asm!(include_str!("../src/mem_set_clr.s"), options(raw));

extern "C" {
  fn libc_memset(d: *mut u8, byte: i32, count: usize) -> *mut u8;
  // and 4 and 8
  fn aeabi_memset(d: *mut u8, count: usize, byte: i32);
  // and 4 and 8
  fn aeabi_memclr(d: *mut u8, count: usize);
}

#[test]
fn test_libc_memset() {
  for count in 0 .. 99 {
    for d in 0..8 {
      if d >= count {
        continue;
      }
      let byte = count as i32;
      let mut v = vec![0_u8; count];
      let out = unsafe { libc_memset(v.as_mut_ptr().add(d), byte, count-d) };
      assert_eq!(unsafe { v.as_ptr().add(d) }, out);
      assert!(v[d..].iter().all(|&b| (b as i32) == byte),
        "\n=dest: {d:?},\n=count: {count},\n=byte: {byte},\nv: {v:?}\n",
        d = v.as_ptr(),
      );
    }
  }
}
