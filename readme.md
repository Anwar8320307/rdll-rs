# dll-rs

A Rust project that demonstrates Windows DLL functionality and WinAPI interaction.

## Overview

dll-rs is a Rust template that can be compiled as both a dynamic-link library (DLL) and a regular executable. It provides an example of how to create Windows DLLs using Rust, including proper exports and Windows API integration.

## Features

- Dual compilation modes (DLL and executable)
- Windows API integration through FFI
- Example exported functions
- DLL lifecycle management

## Project Structure

- `src/main.rs` - Executable entry point
- `src/lib.rs` - Library implementation with DLL exports
- Supporting Rust source files

## Building

To build the project, use Cargo:
```bash
cargo build
```
Or to build in release:
```bash
cargo build --release
```

## Usage

The project can be used in two ways:

1. As a DLL:
    - Build in release mode to generate the DLL
    - The DLL exports a `DllMain` function and example functionality

2. As an executable:
    - Run in debug mode to test DLL functionality without DLL debugging gymnastics
    - Running in release mode will display a warning message

## Technical Details

- Uses `cdylib` and `rlib` crate types
- Implements Windows API bindings
- Provides internal FFI declarations for Windows types
- Includes DLL entry point handling

## Requirements

- Rust 2024 edition
- Windows operating system
- Cargo build system

## Licensing

- MIT or Apache 2.0