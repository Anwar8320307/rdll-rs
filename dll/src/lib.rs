#![no_main]
#![allow(dead_code)]
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

#[allow(non_snake_case)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn WinExec(lpCmdLine: LPVOID, uCmdShow: DWORD) -> DWORD;
    fn GetModuleHandleA(lpModuleName: LPVOID) -> HANDLE;
    fn GetProcAddress(hmodule: HANDLE, lpProcName: LPVOID) -> LPVOID;
}

#[allow(non_snake_case)]
#[link(name = "user32")]
unsafe extern "system" {
    fn MessageBoxA(
        hWnd: HANDLE,
        lpText: LPVOID,
        lpCaption: LPVOID,
        uType: DWORD
    );
}


#[repr(C)]
struct ImageDosHeader {
    e_magic: u16,
    e_cblp: u16,
    e_cp: u16,
    e_crlc: u16,
    e_cparhdr: u16,
    e_minalloc: u16,
    e_maxalloc: u16,
    e_ss: u16,
    e_sp: u16,
    e_csum: u16,
    e_ip: u16,
    e_cs: u16,
    e_lfarlc: u16,
    e_ovno: u16,
    e_res: [u16; 4],
    e_oemid: u16,
    e_oeminfo: u16,
    e_res2: [u16; 10],
    e_lfanew: i32,
}

#[repr(C)]
struct ImageNtHeaders {
    signature: u32,
}

const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;     // 'MZ'
const IMAGE_NT_SIGNATURE: u32 = 0x00004550;  // 'PE\0\0'

#[cfg(target_arch = "x86_64")]
unsafe fn get_ip() -> usize {
    let rip: usize;
    unsafe { std::arch::asm!("lea {}, [rip]", out(reg) rip) };
    rip
}

#[cfg(target_arch = "x86")]
unsafe fn get_ip() -> usize {
    let eip: usize;
    unsafe{
        std::arch::asm!(
        "call 1f",
        "1: pop {}",
        out(reg) eip,
        );
    }

    eip
}

pub fn find_mz_pe_signature() -> Option<*const u8> {
    unsafe {
        let rip = get_ip();
        let mut ptr = rip as *const u8;

        loop {
            if ptr < 2 as *const u8 {
                break;
            }

            let dos_header = ptr.offset(-2) as *const ImageDosHeader;

            if std::ptr::read_unaligned(&(*dos_header).e_magic) == IMAGE_DOS_SIGNATURE {
                let e_lfanew = std::ptr::read_unaligned(&(*dos_header).e_lfanew) as isize;

                if e_lfanew >= std::mem::size_of::<ImageDosHeader>() as isize && e_lfanew < 1024 {
                    let nt_header_ptr = (dos_header as *const u8).offset(e_lfanew) as *const ImageNtHeaders;

                    if std::ptr::read_unaligned(&(*nt_header_ptr).signature) == IMAGE_NT_SIGNATURE {
                        return Some(dos_header as *const u8);
                    }
                }
            }

            ptr = ptr.offset(-1);
        }

        None
    }
}

/// ReflectiveLoader for compatability with legacy Reflective DLL loaders
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "system" fn ReflectiveLoader(){
    let module_base = find_mz_pe_signature();
    if module_base.is_some() {
        let module_base = module_base.unwrap();
        unsafe { std::arch::asm!("call {0}", in(reg) module_base) };
    }
}

/// For maximum compatability with this template, all functionality should be called from `dll_main`
#[unsafe(no_mangle)]
#[allow(named_asm_labels)]
#[allow(non_snake_case, unused_variables)]
pub fn dll_main() {
    let msg = b"Hello from Rust Reflective DLL!\0";
    unsafe { MessageBoxA(std::ptr::null_mut(), msg.as_ptr() as LPVOID, msg.as_ptr() as LPVOID, 0 ); }
}


#[unsafe(no_mangle)]
#[allow(named_asm_labels)]
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
            dll_main();
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
