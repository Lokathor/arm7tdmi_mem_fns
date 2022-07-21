#![allow(dead_code)]

core::arch::global_asm!(include_str!("../src/mem_cpy_move.s"), options(raw));

extern "C" {
  /// Copies to `d` from `s`.
  /// * The two regions *may* overlap.
  /// * The maximum co-alignment is assumed to be 1.
  fn aeabi_memmove1(d: *mut u8, s: *const u8, bytes: usize);

  /// Copies to `d` from `s`.
  /// * The two regions *may* overlap.
  /// * The two pointers **must** be aligned to 2.
  /// * The maximum co-alignment is assumed to be 2.
  fn aeabi_memmove2(d: *mut u16, s: *const u16, bytes: usize);

  /// Copies to `d` from `s`.
  /// * The two regions *may* overlap.
  /// * The two pointers **must** be aligned to 4.
  fn aeabi_memmove4(d: *mut u32, s: *const u32, bytes: usize);

  /// Copies to `d` from `s`.
  /// * The two regions *may* overlap.
  /// * The pointers can be of any alignment.
  ///   This function will check the alignment of both pointers,
  ///   apply a fixup if possible,
  ///   and then call over to `aeabi_memmoveN` for however much
  ///   alignment is available.
  /// **Returns:** the `d` pointer you passed as input.
  fn libc_memmove(d: *mut u8, s: *const u8, bytes: usize) -> *mut u8;
}

fn rand_bytes(n: usize) -> Vec<u8> {
  let mut v = vec![0; n];
  getrandom::getrandom(&mut v).unwrap();
  v
}

fn rand_halfwords(n: usize) -> Vec<u16> {
  let mut v = vec![0_u16; n];
  getrandom::getrandom(bytemuck::cast_slice_mut(&mut v)).unwrap();
  v
}

fn rand_words(n: usize) -> Vec<u32> {
  let mut v = vec![0_u32; n];
  getrandom::getrandom(bytemuck::cast_slice_mut(&mut v)).unwrap();
  v
}

fn rand_u32() -> u32 {
  let mut bytes = [0; 4];
  getrandom::getrandom(&mut bytes).unwrap();
  u32::from_ne_bytes(bytes)
}

struct Lcg(u32);
impl Lcg {
  fn new() -> Self {
    Self(rand_u32())
  }
  fn next_u32(&mut self) -> u32 {
    self.0 = self.0.wrapping_mul(747796405).wrapping_add(1);
    self.0
  }
}

//#[test]
fn test_aeabi_memmove1(){
  let mut lcg = Lcg::new();
  for _ in 0..10 {
    for bytes in 0..99 {
      let mut buffer = rand_bytes(256);
      let mut clone = buffer.clone();
      let d = (lcg.next_u32() % 128) as usize;
      let s = (lcg.next_u32() % 128) as usize;
      unsafe {
        let p = buffer.as_mut_ptr();
        aeabi_memmove1(p.add(d), p.add(s), bytes);
      }
      clone.copy_within(s..(s+bytes), d);
      assert_eq!(clone, buffer, "\nd: {d},\ns: {s},\nbytes: {bytes}");
    }
  }
}

//#[test]
fn test_aeabi_memmove2() {
  let mut lcg = Lcg::new();
  for _ in 0 .. 10 {
    for bytes in 0..99 {
      let mut buffer = rand_halfwords(256);
      let mut clone = buffer.clone();
      let d = (lcg.next_u32() % 128) as usize;
      let s = (lcg.next_u32() % 128) as usize;
      unsafe {
        let p: *mut u16 = buffer.as_mut_ptr();
        aeabi_memmove2(p.add(d), p.add(s), bytes);
      }
      bytemuck::cast_slice_mut::<u16, u8>(&mut clone).copy_within((s*2)..(s*2+bytes), d*2);
      assert_eq!(clone, buffer, "\nd: {d},\ns: {s},\nbytes: {bytes}");
    }
  }
}

//#[test]
fn test_aeabi_memmove4() {
  {
    let mut buffer = rand_words(10);
    let mut clone = buffer.clone();
    let d = 1;
    let s = 0;
    let bytes = 32;
    unsafe {
      let p: *mut u32 = buffer.as_mut_ptr();
      aeabi_memmove4(p.add(d), p.add(s), bytes);
    }
    bytemuck::cast_slice_mut::<u32, u8>(&mut clone).copy_within((s*4)..(s*4+bytes),d*4);
    assert_eq!(clone, buffer, "\nd: {d},\ns: {s},\nbytes: {bytes}");
  }
  let mut lcg = Lcg::new();
  for _ in 0 .. 10 {
    for bytes in 0 .. 128 {
      let mut buffer = rand_words(256);
      let mut clone = buffer.clone();
      let d = (lcg.next_u32() % 128) as usize;
      let s = (lcg.next_u32() % 128) as usize;
      unsafe {
        let p: *mut u32 = buffer.as_mut_ptr();
        aeabi_memmove4(p.add(d), p.add(s), bytes);
      }
      bytemuck::cast_slice_mut::<u32, u8>(&mut clone).copy_within((s*4)..(s*4+bytes),d*4);
      assert_eq!(clone, buffer, "\nd: {d},\ns: {s},\nbytes: {bytes}");
    }
  }
}

#[test]
fn test_libc_memmove() {
  let mut lcg = Lcg::new();
  for _ in 0 .. 1000 {
    for bytes in 0 .. 128 {
      let mut buffer = rand_bytes(256);
      let mut clone = buffer.clone();
      let d = (lcg.next_u32() % 128) as usize;
      let s = (lcg.next_u32() % 128) as usize;
      unsafe {
        let p: *mut u8 = buffer.as_mut_ptr();
        let out = libc_memmove(p.add(d), p.add(s), bytes);
        assert_eq!(p.add(d), out);
      }
      clone.copy_within(s..(s+bytes),d);
      assert_eq!(clone, buffer, "\nd: {d},\ns: {s},\nbytes: {bytes}");
    }
  }
}
