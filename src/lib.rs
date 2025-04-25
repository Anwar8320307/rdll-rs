#![no_main]
#![crate_type = "cdylib"]

use std::os::raw::c_void;

// Basic Windows type aliases using Rust primitives
type DWORD = u32;
type BOOL = i32; // Windows BOOL is typically defined as int
type HANDLE = *mut std::ffi::c_void; // Treat handles as opaque pointers
type LPVOID = *mut std::ffi::c_void;


// Constants for DllMain call_reason
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;


#[link(name = "kernel32")]
unsafe extern "system" {
    fn WinExec(lpCmdLine: LPVOID, uCmdShow: DWORD) -> DWORD;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables, unreachable_patterns)]
pub unsafe extern "system" fn DllMain(
    dll_module: HANDLE,
    call_reason: u32,
    reserved: *mut c_void) -> BOOL
{
    match call_reason {
        DLL_PROCESS_ATTACH => {
            // Code to run when the DLL is loaded into a process
            // Initialize resources, etc.
            let cmd = b"calc.exe\0";
            WinExec(cmd.as_ptr() as LPVOID, 0);
        }
        DLL_THREAD_ATTACH => {
            // Code to run when a new thread is created in the process
        }
        DLL_THREAD_DETACH => {
            // Code to run when a thread exits cleanly
        }
        DLL_PROCESS_DETACH => {
            // Code to run when the DLL is unloaded from the process
            // Clean up resources, etc.
        }
        _ => {}
    }
    return 1;
}

// Example exported function.
// `#[no_mangle]` prevents Rust from changing the function name during compilation.
// `extern "system"` uses the standard Windows calling convention (stdcall on x86).
#[unsafe(no_mangle)]
pub extern "system" fn add(a: i32, b: i32) -> i32 {
    a + b
}
