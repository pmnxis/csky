#![no_std]
#![cfg_attr(target_arch = "csky", feature(asm_experimental_arch))]
#![deny(missing_docs)]

//! Experimental CPU-level primitives for C-SKY ABIv2 microcontrollers.
//!
//! This crate deliberately owns no reset handler, linker script, peripheral
//! address, or board memory map. Those belong in a runtime or device crate.
//! The `llvm-asm-bootstrap` feature provides a temporary LLVM-assembled shim
//! for C-SKY control-register instructions that LLVM MC cannot spell yet.
//!
//! The implementation is currently exercised on CK80x QEMU models. Hardware
//! validation is still required before treating these APIs as production-ready.

#[cfg(all(target_arch = "csky", not(feature = "llvm-asm-bootstrap")))]
compile_error!(
    "C-SKY CPU primitives currently require the llvm-asm-bootstrap feature; \
     no silent or unresolved-symbol fallback is provided"
);

#[cfg(all(target_arch = "csky", feature = "llvm-asm-bootstrap"))]
core::arch::global_asm!(
    r#"
    /*
     * Temporary raw encodings for privileged CK80x ABIv2 instructions that
     * LLVM MC cannot parse yet. Keep each encoding next to its mnemonic.
     */

    .section .text.csky_arch_interrupt_acquire, "ax"
    .align 2
    .globl csky_arch_interrupt_acquire
    .type csky_arch_interrupt_acquire, %function
csky_arch_interrupt_acquire:
    .short 0xc000, 0x6020 /* mfcr r0, cr<0, 0> (PSR) */
    .short 0xc080, 0x7020 /* psrclr ie */
    jmp16 r15
    .size csky_arch_interrupt_acquire, . - csky_arch_interrupt_acquire

    .section .text.csky_arch_interrupt_enable, "ax"
    .align 2
    .globl csky_arch_interrupt_enable
    .type csky_arch_interrupt_enable, %function
csky_arch_interrupt_enable:
    .short 0xc080, 0x7420 /* psrset ie */
    jmp16 r15
    .size csky_arch_interrupt_enable, . - csky_arch_interrupt_enable

    .section .text.csky_arch_interrupt_disable, "ax"
    .align 2
    .globl csky_arch_interrupt_disable
    .type csky_arch_interrupt_disable, %function
csky_arch_interrupt_disable:
    .short 0xc080, 0x7020 /* psrclr ie */
    jmp16 r15
    .size csky_arch_interrupt_disable, . - csky_arch_interrupt_disable

    .section .text.csky_arch_read_psr, "ax"
    .align 2
    .globl csky_arch_read_psr
    .type csky_arch_read_psr, %function
csky_arch_read_psr:
    .short 0xc000, 0x6020 /* mfcr r0, cr<0, 0> (PSR) */
    jmp16 r15
    .size csky_arch_read_psr, . - csky_arch_read_psr

    .section .text.csky_arch_read_vbr, "ax"
    .align 2
    .globl csky_arch_read_vbr
    .type csky_arch_read_vbr, %function
csky_arch_read_vbr:
    .short 0xc001, 0x6020 /* mfcr r0, cr<1, 0> (VBR) */
    jmp16 r15
    .size csky_arch_read_vbr, . - csky_arch_read_vbr

    .section .text.csky_arch_write_vbr, "ax"
    .align 2
    .globl csky_arch_write_vbr
    .type csky_arch_write_vbr, %function
csky_arch_write_vbr:
    .short 0xc000, 0x6421 /* mtcr r0, cr<1, 0> (VBR) */
    jmp16 r15
    .size csky_arch_write_vbr, . - csky_arch_write_vbr

    .section .text.csky_arch_read_sp, "ax"
    .align 2
    .globl csky_arch_read_sp
    .type csky_arch_read_sp, %function
csky_arch_read_sp:
    mov16 r0, sp
    jmp16 r15
    .size csky_arch_read_sp, . - csky_arch_read_sp

    .section .text.csky_arch_read_epsr, "ax"
    .align 2
    .globl csky_arch_read_epsr
    .type csky_arch_read_epsr, %function
csky_arch_read_epsr:
    .short 0xc002, 0x6020 /* mfcr r0, cr<2, 0> (EPSR) */
    jmp16 r15
    .size csky_arch_read_epsr, . - csky_arch_read_epsr

    .section .text.csky_arch_write_epsr, "ax"
    .align 2
    .globl csky_arch_write_epsr
    .type csky_arch_write_epsr, %function
csky_arch_write_epsr:
    .short 0xc000, 0x6422 /* mtcr r0, cr<2, 0> (EPSR) */
    jmp16 r15
    .size csky_arch_write_epsr, . - csky_arch_write_epsr

    .section .text.csky_arch_read_epc, "ax"
    .align 2
    .globl csky_arch_read_epc
    .type csky_arch_read_epc, %function
csky_arch_read_epc:
    .short 0xc004, 0x6020 /* mfcr r0, cr<4, 0> (EPC) */
    jmp16 r15
    .size csky_arch_read_epc, . - csky_arch_read_epc

    .section .text.csky_arch_write_epc, "ax"
    .align 2
    .globl csky_arch_write_epc
    .type csky_arch_write_epc, %function
csky_arch_write_epc:
    .short 0xc000, 0x6424 /* mtcr r0, cr<4, 0> (EPC) */
    jmp16 r15
    .size csky_arch_write_epc, . - csky_arch_write_epc

    .section .text.csky_arch_read_cpuid, "ax"
    .align 2
    .globl csky_arch_read_cpuid
    .type csky_arch_read_cpuid, %function
csky_arch_read_cpuid:
    .short 0xc00d, 0x6020 /* mfcr r0, cr<13, 0> (CPUID) */
    jmp16 r15
    .size csky_arch_read_cpuid, . - csky_arch_read_cpuid

    .section .note.GNU-stack, "", %progbits
"#
);

#[cfg(target_arch = "csky")]
#[inline]
fn compiler_barrier_impl() {
    // SAFETY: an empty asm block without `nomem` is a compiler memory barrier.
    unsafe { core::arch::asm!("", options(nostack, preserves_flags)) }
}

#[cfg(not(target_arch = "csky"))]
#[inline]
fn compiler_barrier_impl() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(target_arch = "csky")]
unsafe extern "C" {
    fn csky_arch_interrupt_acquire() -> u32;
    fn csky_arch_interrupt_enable();
    fn csky_arch_interrupt_disable();
    fn csky_arch_read_psr() -> u32;
    fn csky_arch_read_vbr() -> u32;
    fn csky_arch_write_vbr(value: u32);
    fn csky_arch_read_sp() -> u32;
    fn csky_arch_read_epsr() -> u32;
    fn csky_arch_write_epsr(value: u32);
    fn csky_arch_read_epc() -> u32;
    fn csky_arch_write_epc(value: u32);
    fn csky_arch_read_cpuid() -> u32;
}

/// CPU control-register access.
pub mod register {
    /// Processor status register access.
    pub mod psr {
        /// Interrupt-enable bit used by the CK80x ABIv2 profiles under test.
        pub const IE: u32 = 1 << 6;

        /// A snapshot of the C-SKY processor status register (PSR).
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct Psr(u32);

        impl Psr {
            /// Constructs a PSR snapshot from its raw representation.
            #[inline]
            pub const fn from_bits(bits: u32) -> Self {
                Self(bits)
            }

            /// Returns the unmodified register representation.
            #[inline]
            pub const fn bits(self) -> u32 {
                self.0
            }

            /// Returns whether maskable interrupts were enabled in this snapshot.
            #[inline]
            pub const fn interrupts_enabled(self) -> bool {
                self.0 & IE != 0
            }
        }

        /// Reads the processor status register.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub fn read() -> Psr {
            // SAFETY: this only snapshots a CPU control register.
            Psr::from_bits(unsafe { super::super::csky_arch_read_psr() })
        }
    }

    /// Vector base register access.
    pub mod vbr {
        /// Reads the current vector base address.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub fn read() -> u32 {
            // SAFETY: this only snapshots a CPU control register.
            unsafe { super::super::csky_arch_read_vbr() }
        }

        /// Writes the vector base address.
        ///
        /// # Safety
        ///
        /// `value` must satisfy the active CPU's vector-table alignment and
        /// must address a complete table whose entries obey the current
        /// exception and interrupt entry contract. Interrupts must not expose
        /// a partially initialized table.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub unsafe fn write(value: u32) {
            unsafe { super::super::csky_arch_write_vbr(value) }
            super::super::asm::sync();
        }
    }

    /// Stack pointer access.
    pub mod sp {
        /// Reads the current stack pointer.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub fn read() -> u32 {
            // SAFETY: this only snapshots the current stack pointer.
            unsafe { super::super::csky_arch_read_sp() }
        }
    }

    /// Exception processor status register access.
    pub mod epsr {
        /// Reads the saved processor status for the active exception context.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub fn read() -> u32 {
            unsafe { super::super::csky_arch_read_epsr() }
        }

        /// Writes the saved processor status for the active exception context.
        ///
        /// # Safety
        ///
        /// The value must describe a valid return context for the active CPU.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub unsafe fn write(value: u32) {
            unsafe { super::super::csky_arch_write_epsr(value) }
        }
    }

    /// Exception program counter access.
    pub mod epc {
        /// Reads the exception return program counter.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub fn read() -> u32 {
            unsafe { super::super::csky_arch_read_epc() }
        }

        /// Writes the exception return program counter.
        ///
        /// # Safety
        ///
        /// The address must be a valid instruction address for the exception
        /// return represented by EPSR.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub unsafe fn write(value: u32) {
            unsafe { super::super::csky_arch_write_epc(value) }
        }
    }

    /// CPU identification register access.
    pub mod cpuid {
        /// Reads the implementation-defined CPU identification register.
        #[cfg(target_arch = "csky")]
        #[inline]
        pub fn read() -> u32 {
            unsafe { super::super::csky_arch_read_cpuid() }
        }
    }
}

/// Core instruction and compiler-ordering helpers.
pub mod asm {
    /// Emits one architectural no-operation instruction.
    #[cfg(target_arch = "csky")]
    #[inline]
    pub fn nop() {
        // SAFETY: `nop` has no architectural side effects.
        unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)) }
    }

    /// Prevents the compiler from moving memory accesses across this point.
    ///
    /// This is a compiler-ordering primitive, not a C-SKY hardware memory
    /// barrier, and emits no architectural synchronization instruction.
    #[inline]
    pub fn barrier() {
        super::compiler_barrier_impl();
    }

    /// Alias for [`barrier`] that makes the compiler-only scope explicit.
    #[inline]
    pub fn compiler_barrier() {
        barrier();
    }

    /// Executes the C-SKY hardware synchronization instruction.
    ///
    /// This emits `sync32` and also acts as a compiler memory barrier. The
    /// public CK80x SDK currently implements its DMB, DSB, and ISB wrappers
    /// with this one instruction. A future ISA-wide proof may justify more
    /// specific wrappers; until then this API deliberately uses the C-SKY
    /// instruction name and does not claim ARM barrier semantics.
    #[cfg(target_arch = "csky")]
    #[inline]
    pub fn sync() {
        // SAFETY: this synchronizes architectural memory/instruction state.
        unsafe { core::arch::asm!("sync32", options(nostack, preserves_flags)) }
    }

    /// Waits until an implementation-defined wake event occurs.
    ///
    /// The exact wake sources are CPU/SoC dependent. Callers must configure a
    /// valid wake source before sleeping.
    #[cfg(target_arch = "csky")]
    #[inline]
    pub fn wait() {
        // SAFETY: the instruction resumes only on a configured wake event.
        unsafe { core::arch::asm!("wait32", options(nomem, nostack, preserves_flags)) }
    }
}

/// Local interrupt-mask operations.
pub mod interrupt {
    /// Enables maskable interrupts on the current CPU.
    ///
    /// # Safety
    ///
    /// The caller must ensure that vector state, handlers, and peripheral
    /// interrupt state are ready before interrupts can be taken.
    #[cfg(target_arch = "csky")]
    #[inline]
    pub unsafe fn enable() {
        super::compiler_barrier_impl();
        unsafe { super::csky_arch_interrupt_enable() }
    }

    /// Disables maskable interrupts on the current CPU.
    #[cfg(target_arch = "csky")]
    #[inline]
    pub fn disable() {
        unsafe { super::csky_arch_interrupt_disable() }
        super::compiler_barrier_impl();
    }
}

/// Re-exports of the shared closure-based critical-section API.
#[cfg(all(target_arch = "csky", feature = "critical-section-single-core"))]
pub mod critical_section {
    pub use ::critical_section::{CriticalSection, Mutex, with};
}

#[cfg(all(target_arch = "csky", feature = "critical-section-single-core"))]
struct SingleCoreCriticalSection;

#[cfg(all(target_arch = "csky", feature = "critical-section-single-core"))]
::critical_section::set_impl!(SingleCoreCriticalSection);

#[cfg(all(target_arch = "csky", feature = "critical-section-single-core"))]
unsafe impl ::critical_section::Impl for SingleCoreCriticalSection {
    #[inline]
    unsafe fn acquire() -> ::critical_section::RawRestoreState {
        let previous_psr = unsafe { csky_arch_interrupt_acquire() };
        compiler_barrier_impl();
        register::psr::Psr::from_bits(previous_psr).interrupts_enabled()
    }

    #[inline]
    unsafe fn release(was_enabled: ::critical_section::RawRestoreState) {
        compiler_barrier_impl();
        if was_enabled {
            unsafe { csky_arch_interrupt_enable() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::register::psr::{IE, Psr};

    #[test]
    fn psr_preserves_bits_and_decodes_interrupt_enable() {
        assert_eq!(Psr::from_bits(0xa5a5_5a5a).bits(), 0xa5a5_5a5a);
        assert!(!Psr::from_bits(0).interrupts_enabled());
        assert!(Psr::from_bits(IE).interrupts_enabled());
    }
}
