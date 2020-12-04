use srt_sys::SRT_SOCKOPT::*;
use srt_sys::*;

use os_socketaddr::OsSocketAddr;

use std::ffi::c_void;
use std::ffi::CStr;
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

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");
    let mut os_their_addr = OsSocketAddr::new();

    let os_addr: OsSocketAddr = addr.into();

    let yes = 1u32;

    unsafe {
        srt_startup();
    }

    let ss = unsafe { srt_create_socket() };
    assert_ne!(ss, SRT_ERROR, "create_socket {}", error_to_str());

    unsafe {
        srt_setsockflag(
            ss,
            SRTO_RCVSYN,
            &yes as *const u32 as *const c_void,
            size_of_val(&yes) as i32,
        );
    }

    let st = unsafe {
        srt_bind(
            ss,
            os_addr.as_ptr() as *const sockaddr,
            os_addr.len() as i32,
        )
    };
    assert_ne!(st, SRT_ERROR);

    let st = unsafe { srt_listen(ss, 2) };
    assert_ne!(st, SRT_ERROR, "listen {}", error_to_str());
    let mut len = os_their_addr.capacity() as i32;
    let their_fd = unsafe { srt_accept(ss, os_their_addr.as_mut_ptr() as *mut sockaddr, &mut len) };
    assert_ne!(their_fd, SRT_ERROR, "accept {}", error_to_str());

    for _ in 0..100 {
        let mut msg = [0u8; 2048];
        let st = unsafe { srt_recvmsg(their_fd, msg.as_mut_ptr() as *mut i8, msg.len() as i32) };
        assert_ne!(st, SRT_ERROR, "recvmsg {}", error_to_str());

        let s = CStr::from_bytes_with_nul(&msg[..st as usize]).expect("Malformed message");

        println!("Got msg of len {} << {:?}", st, s);
    }

    let st = unsafe { srt_close(ss) };
    assert_ne!(st, SRT_ERROR, "close {}", error_to_str());

    unsafe {
        srt_cleanup();
    }
}
