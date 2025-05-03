#![allow(unused_imports)]
use dll_rs::dll_main;

/// Executable wrapper for functionality in dll_rs
#[cfg(debug_assertions)]
fn main() {
    dll_main();
}

#[cfg(not(debug_assertions))]
fn main() {
    println!("[!] Executable not compiled in debug mode.");
}
