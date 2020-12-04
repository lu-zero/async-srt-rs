use srt_sys::SRT_SOCKOPT::*;
use srt_sys::*;

use os_socketaddr::OsSocketAddr;

use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::mem::size_of_val;
use std::net::SocketAddr;

fn error_to_str() -> &'static str {
    unsafe {
        let s = srt_getlasterror_str();
        CStr::from_ptr(s).to_str().unwrap()
    }
}

fn main() {
    let mut args = std::env::args();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <remote host>:<remote port>",
            args.next().unwrap()
        );
    }

    let _bin = args.next().unwrap();

    let remote = args.next().unwrap();

    let message = CString::new("This message should be sent to the other side")
        .unwrap()
        .into_bytes_with_nul();

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    let os_addr: OsSocketAddr = addr.into();

    let yes = 1u32;

    unsafe {
        srt_startup();
    }

    let ss = unsafe { srt_create_socket() };
    assert_ne!(ss, SRT_ERROR, "create_socket: {}", error_to_str());

    unsafe {
        srt_setsockflag(
            ss,
            SRTO_SENDER,
            &yes as *const u32 as *const c_void,
            size_of_val(&yes) as i32,
        );
    }

    let st = unsafe {
        srt_connect(
            ss,
            os_addr.as_ptr() as *const sockaddr,
            os_addr.len() as i32,
        )
    };
    assert_ne!(st, SRT_ERROR, "connect {}", error_to_str());
    for i in 0..100 {
        println!("Sending message {}", i);
        let st = unsafe {
            srt_sendmsg2(
                ss,
                message.as_ptr() as *const i8,
                message.len() as i32,
                std::ptr::null_mut(),
            )
        };
        assert_ne!(st, SRT_ERROR, "sendmsg2 {}", error_to_str());

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let st = unsafe { srt_close(ss) };
    assert_ne!(st, SRT_ERROR, "close {}", error_to_str());

    unsafe {
        srt_cleanup();
    }
}
