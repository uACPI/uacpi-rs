//#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

pub mod kernel_api;
pub mod status;
pub mod types;
pub mod tables;

