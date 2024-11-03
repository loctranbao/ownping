use crate::ipv4;
use crate::loadlibrary::Library;
use core::ffi::c_void;
pub type Handle = *const c_void;



pub struct Functions {
    pub IcmpCreateFile: extern "stdcall" fn() -> Handle,
    pub IcmpSendEcho: extern "stdcall" fn(
        handle: Handle,
        dest: ipv4::Addr,
        request_data: *const u8,
        request_size: u16,
        request_options: Option<&IpOptionInformation>,
        reply_buffer: *mut u8,
        reply_size: u32,
        timeout: u32,
    ) -> u32,
    pub IcmpCloseHandle: extern "stdcall" fn(handle: Handle),
}


impl Functions {
    fn get() -> Self {
        let iphlp = Library::new("IPHLPAPI.dll").unwrap();
        Self {
            IcmpCreateFile: unsafe { iphlp.get_proc("IcmpCreateFile").unwrap() },
            IcmpSendEcho: unsafe { iphlp.get_proc("IcmpSendEcho").unwrap() },
            IcmpCloseHandle: unsafe { iphlp.get_proc("IcmpCloseHandle").unwrap() },
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IpOptionInformation  {
    pub ttl: u8,
    pub tos: u8,
    pub flags: u8,
    pub options_size: u8,
    // actually a 32-bit pointer, but, that's a Windows
    // oddity and I couldn't find a built-in Rust type for it.    
    pub options_data: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct IcmpEchoReply {
    pub address: ipv4::Addr,
    pub status: u32,
    pub rtt: u32,
    pub data_size: u16,
    pub reserved: u16,
    pub data: *const u8,
    pub options: IpOptionInformation,
}


use std::sync::LazyLock;
pub static FUNCTIONS: LazyLock<Functions> = LazyLock::new(|| {
    Functions::get()
});
