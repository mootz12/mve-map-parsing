#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(test)]
pub mod test;

pub mod contract;
pub use contract::*;

