//! Definitions found commonly among almost all Unix derivatives
//!
//! More functions and definitions can be found in the more specific modules
//! according to the platform in question.

use crate::c_void;
// The c_* primitive types come from the shared crate::primitives module,
// selected by the Zephyr branch in lib.rs like every other target. Zephyr's
// C compilers (Zephyr SDK) use the per-arch ABI defaults with no
// -f[unsigned]char override, which is exactly what crate::primitives models
// (e.g. unsigned char on arm/aarch64/riscv32/riscv64/xtensa, signed on x86).
pub use crate::primitives::*;

pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

extern "C" {
    pub fn strlen(cs: *const c_char) -> size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void,
                  n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void,
                   n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}
