
core::arch::global_asm!(include_str!("copy_u32.s"), options(raw));

extern "C" {
  /// copies from low to high address.
  /// 
  /// if the regions overlap, the dest addr must be less than the src addr. 
  pub fn copy_u32_forward(d: *mut u32, s: *const u32, count: usize);

  /// copies from high to low address
  ///
  /// if the regions overlap, the dest addres must be greater than the src addr.
  pub fn copy_u32_reverse(d: *mut u32, s: *const u32, count: usize);

  /// copies the memory, automatically determines the correct copy direction.
  pub fn copy_u32_overlapping(d: *mut u32, s: *const u32, count: usize);
}

#[test]
fn test_copy_u32() {
  const SIZE: usize = core::mem::size_of::<u32>();
  let mut buffer = vec![0_u32; 8];
  buffer[0] = 1;
  buffer[1] = 2;
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(0..2, 2);
    let b: *mut u32 = buffer.as_mut_ptr();
    copy_u32_forward(b.add(2), b.add(0), 2 * SIZE);
    assert_eq!(c, buffer);
  }
  // TODO: spare bytes test forward
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(0..4, 4);
    let b: *mut u32 = buffer.as_mut_ptr();
    copy_u32_reverse(b.add(4), b.add(0), 4 * SIZE);
    assert_eq!(c, buffer);
  }
  // TODO: spare bytes test reverse
  buffer[3] = 255;
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(4..(4+3), 3); // dest lower than src
    let b = buffer.as_mut_ptr();
    copy_u32_overlapping(b.add(3), b.add(4), 3 * SIZE);
    assert_eq!(c, buffer);
  }
  buffer[2] = 127;
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(1..(1+5), 2); // src lower than dest
    let b = buffer.as_mut_ptr();
    copy_u32_overlapping(b.add(2), b.add(1), 5 * SIZE);
    assert_eq!(c, buffer);
  }

  let mut seed: u32 = 0xDEAD;
  buffer = core::iter::from_fn(|| {
    // the joke here is that this is one of the worst LCG multipliers.
    seed = seed.wrapping_mul(5).wrapping_add(1);
    Some(seed)
  }).take(100).collect();
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(4..(4+32), 3); // dest lower than src
    let b = buffer.as_mut_ptr();
    copy_u32_overlapping(b.add(3), b.add(4), 32 * SIZE);
  }
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(5..(5+65), 4); // dest lower than src
    let b = buffer.as_mut_ptr();
    copy_u32_overlapping(b.add(4), b.add(5), 65 * SIZE);
  }
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(1..(1+97), 2); //src lower than dest
    let b = buffer.as_mut_ptr();
    copy_u32_overlapping(b.add(2), b.add(1), 97 * SIZE);
  }
}












