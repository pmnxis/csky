# `csky`

Low-level, `no_std` CPU primitives for experimental Rust support on C-SKY
ABIv2 microcontroller profiles.

This architecture crate provides typed PSR access, VBR/SP/EPSR/EPC/CPUID
access, instruction and compiler barriers, interrupt masking, and an optional
single-core critical-section implementation. It deliberately does not provide
a reset handler, vector table, linker script, peripheral definitions, or a
board memory map. Those belong in a runtime crate such as `csky-rt` and in
device or board support crates.

## Status

This crate is experimental. It currently depends on a custom C-SKY Rust target
and a patched LLVM/LLD toolchain. The assembly bootstrap contains reviewed raw
encodings for control-register instructions that LLVM's C-SKY assembler does
not yet accept by mnemonic. Passing host-side `cargo test` does not validate
those instructions; firmware builds must also be linked and exercised on a
C-SKY emulator or device.

The currently tested project profiles are CK801, CK802, CK803, CK804, CK805,
and CK807. This is not a claim that every chip using those cores is supported:
privilege rules, CPU revisions, interrupt controllers, memory maps, startup,
and peripherals remain device-specific. In particular, CK802 E2 code
generation still has an unresolved execution discrepancy in the project's
current QEMU/toolchain combination.

## Features

- `llvm-asm-bootstrap`: includes the temporary LLVM-integrated assembly
  implementation. Current firmware users must enable this feature. `csky-rt`
  enables and forwards it by default so applications normally do not select it
  twice.
- `critical-section-single-core`: installs a global `critical-section`
  implementation by retaining the previous interrupt-enable boolean and
  masking interrupts.

`critical-section-single-core` is only suitable for a privileged, single-core
bare-metal environment. It is not a multicore lock, and it may be inappropriate
when an scheduler owns interrupt masking or when some interrupts must remain enabled.

```toml
[dependencies]
csky = { version = "0.0.1", features = [
    "llvm-asm-bootstrap",
    "critical-section-single-core",
] }
```

## Ordering primitives

- `asm::barrier()` and `asm::compiler_barrier()` prevent compiler reordering
  but emit no C-SKY hardware synchronization instruction.
- `asm::sync()` emits `sync32` and also prevents compiler reordering.
- `asm::wait()` emits `wait32`; configuring a valid wake source remains the
  responsibility of the device runtime or HAL.

The crate intentionally does not expose ARM-named DMB, DSB, or ISB guarantees
until equivalent semantics are established for the relevant C-SKY profiles.

## Safety

Enabling interrupts is unsafe because vector state, handlers, stack state, and
peripheral interrupt sources must already be valid. Writing VBR, EPSR, or EPC
is unsafe because invalid values can break vector dispatch or exception return.
Register and interrupt operations may require privileged execution. Interrupt
masking does not by itself provide inter-core synchronization.

## Runtime boundary

The companion `csky-rt` crate provides reset initialization, linker sections,
and generic 32/48/64-entry vector-table layouts. Chip-specific memory maps,
external interrupt assignments, interrupt-controller drivers, and RAM-vector
policy belong in device, PAC, HAL, or board crates rather than this crate.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)); or
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
