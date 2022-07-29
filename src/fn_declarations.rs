extern "C" {
  /// Copies `n` bytes to `dest` from `src`.
  ///
  /// **Returns:** The original `dest` value.
  ///
  /// This implements the libc [memcpy][man-memcpy] function.
  ///
  /// [man-memcpy]: https://man7.org/linux/man-pages/man3/memcpy.3.html
  ///
  /// ## Safety
  /// * `dest` and `src` must point to exclusive regions.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn libc_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;

  /// Copies `n` bytes to `dest` from `src`.
  ///
  /// This implements the AEABI [aeabi_memcpy][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` and `src` must point to exclusive regions.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn aeabi_memcpy(dest: *mut u8, src: *const u8, n: usize);

  /// Copies `n` bytes to `dest` from `src`.
  ///
  /// This implements the AEABI [aeabi_memcpy4][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` and `src` must point to exclusive regions.
  /// * `dest` and `src` must be aligned to 4.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn aeabi_memcpy4(dest: *mut u8, src: *const u8, n: usize);

  /// Copies `n` bytes to `dest` from `src`.
  ///
  /// This implements the AEABI [aeabi_memcpy8][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` and `src` must point to exclusive regions.
  /// * `dest` and `src` must be aligned to 8.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn aeabi_memcpy8(dest: *mut u8, src: *const u8, n: usize);

  /// Copies `n` bytes to `dest` from `src`.
  ///
  /// This function will *only* copy using `ldrb` and `strb`. That makes it
  /// suitable for use with the GBA's SRAM region. Note that for this function
  /// to be usable with SRAM you must **also** place the functions provided by
  /// this crate into IWRAM, not into ROM.
  ///
  /// ## Safety
  /// * `dest` and `src` must point to exclusive regions.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn gba_memcpy_sram(dest: *mut u8, src: *const u8, n: usize);

  /// Copies `n` bytes to `dest` from `src` (overlap allowed).
  ///
  /// **Returns:** The original `dest` value.
  ///
  /// This implements the libc [memmove][man-memmove] function.
  ///
  /// [man-memmove]: https://man7.org/linux/man-pages/man3/memmove.3.html
  ///
  /// ## Safety
  /// * `dest` and `src` **can** point to overlapping regions.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn libc_memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;

  /// Copies `n` bytes to `dest` from `src` (overlap allowed).
  ///
  /// This implements the AEABI [aeabi_memmove][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` and `src` **can** point to overlapping regions.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn aeabi_memmove(dest: *mut u8, src: *const u8, n: usize);

  /// Copies `n` bytes to `dest` from `src` (overlap allowed).
  ///
  /// This implements the AEABI [aeabi_memmove4][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` and `src` **can** point to overlapping regions.
  /// * `dest` and `src` must be aligned to 4.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn aeabi_memmove4(dest: *mut u8, src: *const u8, n: usize);

  /// Copies `n` bytes to `dest` from `src` (overlap allowed).
  ///
  /// This implements the AEABI [aeabi_memmove8][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` and `src` **can** point to overlapping regions.
  /// * `dest` and `src` must be aligned to 8.
  /// * `dest` must be writable for the region given.
  /// * `src` must be readable for the region given.
  /// * This performs an "untyped" copy, so `src` **can** contain uninitialized
  ///   bytes. Those bytes will be uninitialized in `dest` after the copy.
  pub fn aeabi_memmove8(dest: *mut u8, src: *const u8, n: usize);

  /// Sets `n` bytes in `dest` to `val as u8`.
  ///
  /// **Returns:** The original `dest` value.
  ///
  /// This implements the libc [memset][man-memset] function.
  ///
  /// [man-memset]: https://man7.org/linux/man-pages/man3/memset.3.html
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  pub fn libc_memset(dest: *mut u8, val: i32, n: usize) -> *mut u8;

  /// Sets `n` bytes in `dest` to `val as u8`.
  ///
  /// This implements the AEABI [aeabi_memset][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  pub fn aeabi_memset(dest: *mut u8, n: usize, val: i32);

  /// Sets `n` bytes in `dest` to `val as u8`.
  ///
  /// This implements the AEABI [aeabi_memset4][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  /// * `dest` must be aligned to 4.
  pub fn aeabi_memset4(d: *mut u8, bytes: usize, val: i32);

  /// Sets `n` bytes in `dest` to `val as u8`.
  ///
  /// This implements the AEABI [aeabi_memset8][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  /// * `dest` must be aligned to 8.
  pub fn aeabi_memset8(d: *mut u8, bytes: usize, val: i32);

  /// Sets `n` bytes in `dest` to 0.
  ///
  /// This implements the AEABI [aeabi_memclr][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  pub fn aeabi_memclr(d: *mut u8, bytes: usize);

  /// Sets `n` bytes in `dest` to 0.
  ///
  /// This implements the AEABI [aeabi_memset][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  /// * `dest` must be aligned to 4.
  pub fn aeabi_memclr4(d: *mut u8, bytes: usize);

  /// Sets `n` bytes in `dest` to 0.
  ///
  /// This implements the AEABI [aeabi_memset][aeabi-mem] function.
  ///
  /// [aeabi-mem]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * `dest` must be writable for the number of bytes given.
  /// * `dest` must be aligned to 8.
  pub fn aeabi_memclr8(d: *mut u8, bytes: usize);

  /// Safely performs an unaligned read of the `u32` address given.
  ///
  /// The 4 bytes pointed to are read one byte at a time, and then the value is
  /// combined and returned.
  ///
  /// This implements the AEABI [aeabi_uread4][aeabi-unaligned] function.
  ///
  /// [aeabi-unaligned]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  ///
  /// ## Safety
  /// * `address` must be valid for a read (other than its alignment).
  pub fn aeabi_uread4(address: *const u32) -> u32;

  /// Safely performs an unaligned read of the `u64` address given.
  ///
  /// The 8 bytes pointed to are read one byte at a time, and then the value is
  /// combined and returned.
  ///
  /// This implements the AEABI [aeabi_uread8][aeabi-unaligned] function.
  ///
  /// [aeabi-unaligned]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  ///
  /// ## Safety
  /// * `address` must be valid for a read (other than its alignment).
  pub fn aeabi_uread8(address: *const u64) -> u64;

  /// Safely performs an unaligned write of the `u32` address given.
  ///
  /// The 4 bytes pointed to are written one byte at a time.
  ///
  /// This implements the AEABI [aeabi_uwrite4][aeabi-unaligned] function.
  ///
  /// [aeabi-unaligned]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  ///
  /// ## Safety
  /// * `address` must be valid for a write (other than its alignment).
  pub fn aeabi_uwrite4(value: u32, address: *mut u32) -> u32;

  /// Safely performs an unaligned write of the `u64` address given.
  ///
  /// The 8 bytes pointed to are written one byte at a time.
  ///
  /// This implements the AEABI [aeabi_uwrite8][aeabi-unaligned] function.
  ///
  /// [aeabi-unaligned]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  ///
  /// ## Safety
  /// * `address` must be valid for a write (other than its alignment).
  pub fn aeabi_uwrite8(value: u64, address: *mut u64) -> u64;

  /// Performs `i32 / i32` division.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is 0.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_idiv(numerator: i32, denominator: i32) -> i32;

  /// Performs `u32 / u32` division.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is 0.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_uidiv(numerator: u32, denominator: u32) -> u32;

  /// Performs `[i32 / i32, i32 % i32]`, returning the data packed in a `u64`.
  ///
  /// The return value is stored in the `r0` and `r1` registers. For ABI
  /// reasons, the only way that we can get an Rust `extern "C"` function
  /// declaration to grab both of those registers as the return value is to
  /// declare the return type as a 64-bit integer. The quotent will be the lower
  /// 32 bits, and the remainder will be the upper 32 bits.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]:
  ///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is `[0, numerator]`.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_idivmod(numerator: i32, denominator: i32) -> u64;

  /// Performs `[u32 / u32, u32 % u32]`, returning the data packed in a `u64`.
  ///
  /// The return value is stored in the `r0` and `r1` registers. For ABI
  /// reasons, the only way that we can get an Rust `extern "C"` function
  /// declaration to grab both of those registers as the return value is to
  /// declare the return type as a 64-bit integer. The quotent will be the lower
  /// 32 bits, and the remainder will be the upper 32 bits.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]:
  ///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is `[0, numerator]`.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_uidivmod(numerator: u32, denominator: u32) -> u64;
}
