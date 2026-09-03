# Changelog

All notable changes to this project will be documented in this file.

## [0.0.2] - 2026-09-04

- Add `unsafe asm::enable_wait()` for executor idle paths that need an
  adjacent interrupt-enable and `wait32` sequence.
- Document the recurring wake-interrupt contract and the remaining one-shot
  wake race explicitly.

## [0.0.1] - 2026-08-24

- Add experimental C-SKY ABIv2 CPU register access primitives.
- Add interrupt masking and an optional single-core critical-section
  implementation.
- Add compiler and architectural ordering helpers.
- Add the temporary LLVM assembly bootstrap for control-register operations.

[0.0.1]: https://github.com/pmnxis/csky/releases/tag/v0.0.1
[0.0.2]: https://github.com/pmnxis/csky/compare/v0.0.1...v0.0.2
