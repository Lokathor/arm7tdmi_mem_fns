# arm7tdmi_mem_fns

Memory operation functions for the ARM7TDMI.

For a step by step explanation,
check out the [HackMD](https://hackmd.io/@Lokathor/HJXTfarj5).

## Use

Crates can't really specify what link section they want a dep's code to go in,
so this crate isn't even published for direct use.

At the moment, just copy the assembly files directly into your own project.

## Testing

Testing of this crate is generally easiest using [cross](https://github.com/cross-rs/cross).

```
cross test --target arm-unknown-linux-gnueabi
```
