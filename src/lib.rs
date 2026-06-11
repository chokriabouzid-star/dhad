//! # Dhad (ضاد) — Arabic Text Canonicalization Library
//!
//! Dhad implements a deterministic 12-stage pipeline that converts
//! Arabic Unicode text into a canonical [`AtomStream`] of [`DhadAtom`]s,
//! then computes two independent hashes:
//!
//! - **[`hash::core_hash`]**: orthographic identity (base + marks + flags)
//! - **[`hash::phonetic_hash`]**: prosodic identity (CoreHash + prosody layer)
//!
//! ## Quick Start
//!
//! ```rust
//! use dhad::modes::process_mode_a;
//!
//! let result = process_mode_a("بِسْمِ".as_bytes()).unwrap();
//! println!("atoms:    {}", result.stream.len());
//! println!("core:     {}", hex::encode(result.core_hash));
//! println!("phonetic: {}", hex::encode(result.phonetic_hash));
//! ```
//!
//! ## Design Axioms
//!
//! | Axiom | Description |
//! |-------|-------------|
//! | A1 | Determinism — same input → identical output |
//! | A2 | No Silent Correction — invalid input → error |
//! | A3 | Separation of Concerns — CoreHash ⊥ PhoneticHash |
//! | A6 | Mark Order Independence — diacritic order is irrelevant |
//! | A7 | Source Digit Independence — ASCII/Arabic-Indic/Extended are identical |
//! | A8 | Semantic Integrity — contradictory prosody is rejected |
//!
//! ## Pipeline Stages
//!
//! ```text
//! Input bytes
//!   │ Pre-stage: MAX_INPUT_BYTES check (4 MiB)
//!   │ Stage 1:  UTF-8 decode
//!   │ Stage 2:  BOM removal
//!   │ Stage 3:  FAPS decomposition (presentation forms)
//!   │ Stage 4:  Noise filtering (tatweel, ZWJ, BiDi controls…)
//!   │ Stage 5:  Codepoint classification
//!   │ Stage 6:  Base atom construction + diacritic attachment
//!   │ Stage 7:  Flag resolution (hamza/madda)
//!   │ Stage 8:  Digit normalization
//!   │ Stage 9:  Prosody resolution (tanween, superscript alef)
//!   │ Stage 10: CRF validation (23 invariants)
//!   │ Stage 11: Serialization (n × 8 bytes, little-endian)
//!   └ Stage 12: CoreHash + PhoneticHash (SHA-256)
//! ```
//!
//! ## Specification
//!
//! This implementation conforms to **Dhad Implementation Specification v1.0**
//! with corrections CR-01 through CR-07 applied.

pub(crate) mod base_map;
pub mod constants;
pub(crate) mod faps;
pub mod hash;
pub mod invariants;
pub mod mode_b;
pub mod model;
pub mod modes;
pub(crate) mod noise;
pub(crate) mod pipeline;
pub mod registry;

pub use model::{AtomStream, DhadAtom, DhadResult, ErrorKind};
pub use modes::{process_mode_a, process_mode_b};
