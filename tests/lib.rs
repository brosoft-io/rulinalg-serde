#[macro_use]
extern crate rulinalg_serde;

pub mod mat;

#[cfg(feature = "io")]
pub mod io {
    mod csv;
}
