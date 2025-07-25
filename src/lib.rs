//! # KTX2 Rust Wrapper
//!
//! A high-level, memory-safe Rust wrapper for the KTX2 texture format with full
//! Basis Universal compression support.
//!
//! ## Features
//!
//! - **Full KTX2 Support**: Read, write, and manipulate KTX2 texture files
//! - **Basis Universal Integration**: Compress textures with ETC1S and UASTC
//! - **Universal Transcoding**: Convert to GPU-specific formats
//! - **Memory Safety**: Safe Rust API with proper error handling
//! - **Cross-Platform**: Support for Windows, macOS, and Linux
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ktx2_rw::{Ktx2Texture, BasisCompressionParams};
//! # fn main() -> ktx2_rw::Result<()> {
//!
//! // Create a new texture
//! let mut texture = Ktx2Texture::create(512, 512, 1, 1, 1, 1, 37)?; // RGBA8
//!
//! // Load from file
//! let mut texture = Ktx2Texture::from_file("texture.ktx2")?;
//!
//! // Compress with Basis Universal
//! let params = BasisCompressionParams::builder()
//!     .quality_level(128)
//!     .thread_count(4)
//!     .build();
//! texture.compress_basis(&params)?;
//! # Ok(())
//! # }
//! ```

// Internal modules
mod bindings;
mod compression;
mod error;
mod format;
mod texture;

#[cfg(test)]
mod tests;

// Public API exports
pub use compression::{BasisCompressionParams, BasisCompressionParamsBuilder};
pub use error::{Error, Result};
pub use format::TranscodeFormat;
pub use texture::Ktx2Texture;
