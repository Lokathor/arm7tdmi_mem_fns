
use arm7tdmi_mem_fns::*;

fn rand_bytes(n: usize) -> Vec<u8> {
  let mut v = vec![0; n];
  getrandom::getrandom(&mut v).unwrap();
  v
}

fn rand_u32() -> u32 {
  let mut bytes = [0; 4];
  getrandom::getrandom(&mut bytes).unwrap();
  u32::from_ne_bytes(bytes)
}

struct BadLCG(u32);
impl BadLcg {
  fn next_u32(&mut self) -> u32 {
    self.0 = self.0.wrapping_mul(5).wrapping_add(1);
    self.0
  }
}

fn do_check(mut b: Vec<u8>, d: usize, s: usize, count: usize) {
  let mut clone = b.clone();
  clone.copy_within(s..(s+count), d);
  unsafe {
    let p = b.as_mut_ptr();
    zmemmove(p.add(d), p.add(s), count);
  }
  assert_eq!(clone, b, "failure: d:{d}, s:{s}, count:{count}");
}

#[test]
fn test_zmemcpy() {
  let mut lcg = BadLcg(rand_u32());
  
  for count in 0..138 {
    let b = rand_bytes(1024);
    let d = lcg.next_u32() % 512;
    let s = lcg.next_u32() % 512;
    do_check(b, d, s, count);
  }
}
