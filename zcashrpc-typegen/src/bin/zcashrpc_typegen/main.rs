//! Main entry point for ZcashrpcTypegen

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use zcashrpc_typegen::application::APPLICATION;

/// Boot ZcashrpcTypegen
fn main() {
    abscissa_core::boot(&APPLICATION);
}
