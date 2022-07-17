
core::arch::global_asm!(include_str!("copy_u16.s"), options(raw));

extern "C" {
  /// copies from low to high address.
  /// 
  /// if the regions overlap, the dest addr must be less than the src addr. 
  pub fn copy_u16_forward(d: *mut u16, s: *const u16, count: usize);

  /// copies from high to low address
  ///
  /// if the regions overlap, the dest addres must be greater than the src addr.
  pub fn copy_u16_reverse(d: *mut u16, s: *const u16, count: usize);

  /// copies the memory, automatically determines the correct copy direction.
  pub fn copy_u16_overlapping(d: *mut u16, s: *const u16, count: usize);
}

#[test]
fn test_copy_u16() {
  const SIZE: usize = core::mem::size_of::<u16>();
  let mut buffer = vec![0_u16; 8];
  buffer[0] = 1;
  buffer[1] = 2;
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(0..2, 2);
    let b: *mut u16 = buffer.as_mut_ptr();
    copy_u16_forward(b.add(2), b.add(0), 2 * SIZE);
    assert_eq!(c, buffer);
  }
  // TODO: spare byte test forward
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(0..4, 4);
    let b: *mut u16 = buffer.as_mut_ptr();
    copy_u16_reverse(b.add(4), b.add(0), 4 * SIZE);
    assert_eq!(c, buffer);
  }
  // TODO: spare byte test reverse
  buffer[3] = 255;
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(4..(4+3), 3); // dest lower than src
    let b = buffer.as_mut_ptr();
    copy_u16_overlapping(b.add(3), b.add(4), 3 * SIZE);
    assert_eq!(c, buffer);
  }
  buffer[2] = 127;
  unsafe {
    let mut c = buffer.clone();
    c.copy_within(1..(1+5), 2); // src lower than dest
    let b = buffer.as_mut_ptr();
    copy_u16_overlapping(b.add(2), b.add(1), 5 * SIZE);
    assert_eq!(c, buffer);
  }
}
