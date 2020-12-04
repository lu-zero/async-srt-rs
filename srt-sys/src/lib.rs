#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg_attr(feature = "cargo-clippy", allow(const_static_lifetime))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

mod srt {
    include!(concat!(env!("OUT_DIR"), "/srt.rs"));
}

pub use srt::*;

#[cfg(test)]
mod test {
    use super::SRT_SOCKOPT::*;
    use super::SRT_TRANSTYPE::*;
    use super::*;
    use std::ffi::c_void;
    use std::mem::size_of_val;

    #[test]
    fn capi() {
        unsafe {
            let yes = 1u32;
            srt_startup();

            let ss = srt_create_socket();
            assert_ne!(ss, SRT_ERROR);

            let minversion = SRT_VERSION_FEAT_HSv5;
            srt_setsockflag(
                ss,
                SRTO_MINVERSION,
                &minversion as *const u32 as *const c_void,
                size_of_val(&minversion) as i32,
            );

            let file_mode = SRTT_FILE;
            srt_setsockflag(
                ss,
                SRTO_TRANSTYPE,
                &file_mode as *const u32 as *const c_void,
                size_of_val(&file_mode) as i32,
            );
            srt_setsockflag(
                ss,
                SRTO_MESSAGEAPI,
                &yes as *const u32 as *const c_void,
                size_of_val(&yes) as i32,
            );

            let st = srt_close(ss);
            assert_ne!(st, SRT_ERROR);

            srt_cleanup();
        }
    }
}
