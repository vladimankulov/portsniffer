use std::io::{stdout, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{Sender};

const MAX_PORT: u16 = 65535;

pub fn scan_for_available_port(tx: Sender<u16>, start_port: u16, ip_addr: IpAddr, thread_count: u16) {
    let mut port: u16 = start_port + 1;

    loop {
        match TcpStream::connect((ip_addr, port)) {
            Ok(_) => {
                print!(".");
                stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => ()
        }


        if (MAX_PORT - port) <= thread_count {
            break;
        }
        port += thread_count
    }
}