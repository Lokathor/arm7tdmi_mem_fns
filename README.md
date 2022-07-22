# arm7tdmi_aeabi

Implements runtime support functions according to ARM's [AEABI][aeabi].
All functions are specialized to the ARM7TDMI CPU.
They should work with any later CPU as well,
but because of instruciton pipeline differences they might have less than optimal performance.

[aeabi]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst

## Use

Unfortunately, crates can't specify what link section they want a *dependency's* code to use.
Since my main use for this assembly is to have it in a special section on the GBA,
I can't just publish it to crates.io and then use it as a dependency and have it go where I want.
Or, I could declare the stuff with the special sections I want,
but then *no one else* would be able to use the crate.
So, right now, this is just not published on crates.io at all.

If you want to use the code, just copy the assembly files directly into your own project
and use them that way. Hopefully this situation can be improved somehow. (eventually?)

If you've got ideas on how to make things work better please open up an issue.

## Testing

Testing of this crate is generally easiest using [cross](https://github.com/cross-rs/cross).

```
cross test --target arm-unknown-linux-gnueabi
```

Or, if you're running on an ARM device (eg: rpi with the 32-bit OS) then you can test natively I guess.

## License

All the code here is released under CCO.

OR (if you really want to use the standard Rust project licenses) Apache-2.0 OR MIT can also be used.
