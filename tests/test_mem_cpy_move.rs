
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
