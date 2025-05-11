#![allow(unused_imports)]
use crate::windows::*;
use std::ffi::CString;
use std::ptr::null_mut;

pub static PIPE_NAME: &[u8; 42] = b"\\\\.\\pipe\\RDLL_PIPE_NAME_NO_CHANGE_PLS\0\0\0\0\0";

#[cfg(not(debug_assertions))]
pub(crate) fn write_output(data: &str) {
    let pipe_name = String::from_utf8_lossy(&*PIPE_NAME);
    let message = data.as_bytes();
    let mut bytes_written: u32 = 0;

    let msg = pipe_name.as_ptr() as *const u8;
    unsafe {
        MessageBoxA(null_mut(), msg as LPVOID, msg as LPVOID, 0);
    }

    let h_pipe = unsafe {
        CreateNamedPipeA(
            pipe_name.as_ptr() as *const u8,
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_BYTE,
            1,
            4096,
            4096,
            0,
            std::ptr::null_mut(),
        )
    };

    if h_pipe == INVALID_HANDLE_VALUE {
        let err = unsafe { GetLastError() };
        dbg!("CreateNamedPipe failed: {}", err);
        return;
    }

    dbg!(
        "[*] Waiting for Beacon connection on {}",
        pipe_name.to_string()
    );

    let connected = unsafe { ConnectNamedPipe(h_pipe, std::ptr::null_mut()) };
    if connected == 0 {
        let err = unsafe { GetLastError() };
        if err != ERROR_PIPE_CONNECTED {
            dbg!("ConnectNamedPipe failed: {}", err);
            unsafe { CloseHandle(h_pipe) };
            return;
        }
    }

    dbg!("[+] Beacon connected! Sending message...");

    let success = unsafe {
        WriteFile(
            h_pipe,
            message.as_ptr(),
            message.len() as u32,
            &mut bytes_written,
            std::ptr::null_mut(),
        )
    };

    if success == 0 {
        let err = unsafe { GetLastError() };
        dbg!("WriteFile failed: {}", err);
        unsafe { CloseHandle(h_pipe) };
        return;
    }

    unsafe {
        FlushFileBuffers(h_pipe);
        CloseHandle(h_pipe);
    }
}

#[cfg(debug_assertions)]
pub(crate) fn write_output(data: &str) {
    println!("{}", data);
}
