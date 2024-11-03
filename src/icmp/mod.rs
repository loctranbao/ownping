// note that we're not declaring this one "pub"
// it's just our own!
mod icmp_sys;
use icmp_sys::*;
use crate::ipv4;



pub struct Request {
    dest: ipv4::Addr,
    ttl: u8,
    timeout: u32,
    data: Option<Vec<u8>>
}

impl Request {
    pub fn new(dest: ipv4::Addr) -> Self {
        Self {
            dest,
            ttl: 128,
            timeout: 4000,
            data: None
        }
    }

    pub fn ttl (mut self, ttl: u8) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn timeout (mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn data<D>(mut self, data: D) -> Self 
    where
        D: Into<Vec<u8>>,
    {
        self.data = Some(data.into());
        self
    }        

    pub fn send(self) ->  Result<Reply, String> {
        let funcs = &icmp_sys::FUNCTIONS;

        let handle = (funcs.IcmpCreateFile)();
        let data = self.data.unwrap_or_default();
        let ip_opts = IpOptionInformation {
            ttl: self.ttl,
            tos: 0,
            flags: 0,
            options_data: 0,
            options_size: 0,
        };
    
        use std::mem;
        let reply_size = mem::size_of::<IcmpEchoReply>();
        let reply_buf_size = reply_size + 8 + data.len();
        let mut reply_buf = vec![0u8; reply_buf_size];    
    
        let ret = (funcs.IcmpSendEcho)(
            handle,
            self.dest, // destination
            data.as_ptr(),        // request data
            data.len() as u16,
            Some(&ip_opts),
            reply_buf.as_mut_ptr(), // reply buffer
            reply_buf_size as u32,
            self.timeout, // timeout (4 seconds)
        );

        (funcs.IcmpCloseHandle)(handle);
    
        

        if ret > 0 {
            let reply: &IcmpEchoReply = unsafe {transmute(&reply_buf[0])};
            let data: *const u8 = unsafe {transmute(&reply_buf[reply_size+8])};
            let reply_data = unsafe {std::slice::from_raw_parts(data, reply.data_size as usize)};
            Ok(Reply {
                addr: self.dest,
                data: reply_data.into(),
                rtt: Duration::from_millis(reply.rtt as u64),
                ttl : reply.options.ttl
            })
        } else {
            Err("IcmpSendEcho failed".to_string())
        }
    }
}

use std::{mem::transmute, time::Duration};
pub struct Reply {
    pub addr: ipv4::Addr,
    pub data: Vec<u8>,
    pub rtt: Duration,
    pub ttl: u8,
}