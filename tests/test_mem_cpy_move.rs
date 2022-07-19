
use arm7tdmi_mem_fns::*;

fn rand_bytes(n: usize) -> Vec<u8> {
  let mut v = vec![0; n];
  getrandom::getrandom(&mut v).unwrap();
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

struct BadLCG(u32);
impl BadLCG {
  fn next_u32(&mut self) -> u32 {
    self.0 = self.0.wrapping_mul(5).wrapping_add(1);
    self.0
  }
}

fn do_move(mut b: Vec<u8>, d: usize, s: usize, count: usize) {
  let mut clone = b.clone();
  clone.copy_within(s..(s+count), d);
  unsafe {
    let p = b.as_mut_ptr();
    zmemmove(p.add(d), p.add(s), count);
  }
  assert_eq!(clone, b, "failure: d:{d}, s:{s}, count:{count}");
}

#[test]
fn test_zmemmove() {
  let mut lcg = BadLCG(rand_u32());
  
  for _ in 0..10 {
    for count in 0..138 {
      let b = rand_bytes(1024);
      let d = (lcg.next_u32() % 512) as usize;
      let s = (lcg.next_u32() % 512) as usize;
      do_move(b, d, s, count);
    }
  }
}

fn do_move4(mut b: Vec<u32>, d: usize, s: usize, count: usize) {
  let mut clone = b.clone();
  clone.copy_within(s..(s+count), d);
  unsafe {
    let p = b.as_mut_ptr();
    z__aeabi_memmove4(p.add(d).cast(), p.add(s).cast(), count);
  }
  assert_eq!(clone, b, "\nfailure:\nd:{d}\ns:{s}\ncount:{count}");
}

#[test]
fn test_z__aeabimemmove4() {
  let mut lcg = BadLCG(rand_u32());

  do_move4(rand_words(2), 0, 1, 1);
  do_move4(rand_words(2), 1, 0, 1);

  for _ in 0 .. 10 {
    for words in 0..9 {
      let b = rand_words(16);
      let d = (lcg.next_u32() % 4) as usize;
      let s = (lcg.next_u32() % 4) as usize;
      do_move4(b, d, s, words);
    }
  }
}
