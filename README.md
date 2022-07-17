# arm7tdmi_mem_fns

Memory operation functions for the ARM7TDMI.

For a step by step explanation of the assembly,
check out the [HackMD Article](https://hackmd.io/@Lokathor/HJXTfarj5).

## Use

Unfortunately, crates can't specify what link section they want a *dependency's* code to use.
Since my main use for this assembly is to have it in a special section on the GBA,
I can't just publish it to crates.io and then use it as a dependency.
Or, I could declare stuff with the `link_section` attributes I want,
but then *no one else* would be able to use the crate.
So, right now, the crate is just not published on crates.io at all.

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

OR (if you really want to use the standard Rust licenses) Apache-2.0 OR MIT can also be used.
