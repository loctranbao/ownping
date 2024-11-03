mod loadlibrary;
mod icmp;
mod ipv4;
use std::{env, error::Error, process::exit};

// 
fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        exit(1);
    });

    use icmp::Request;
    let dest = ipv4::Addr::parse(arg)?;
    let data = "O Romeo.";

    println!();
    println!("Pinging {:?} with {} bytes of data:", dest, data.len());

    use std::{thread::sleep, time::Duration};

    for _ in 0..4 {
        match Request::new(dest).ttl(128).timeout(4000).data(data).send() {
            Ok(res) => println!(
                "Reply from {:?}: bytes={} time={:?} TTL={}",
                res.addr,
                res.data.len(),
                res.rtt,
                res.ttl,
            ),
            Err(_) => println!("Something went wrong"),
        }

        sleep(Duration::from_secs(1));
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_transmute() {
        let myvec: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let val_as_u32:&u32 = unsafe {std::mem::transmute(&myvec)};
        let val_as_u16:&u16 = unsafe {std::mem::transmute(&myvec)};
        let val_as_u8:&u8 = unsafe {std::mem::transmute(&myvec)};
        println!("{}", val_as_u32);
        println!("{}", val_as_u16);
        println!("{}", val_as_u8);
        // assert_eq!(val_as_u32, 16909060);
    }
}
