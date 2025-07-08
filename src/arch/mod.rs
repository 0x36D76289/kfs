//! Architecture-specific code
//! 
//! This module contains all architecture-specific implementations.
//! Currently supports x86_64 only.

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Unsupported architecture");
