use std::{env, process, thread};
use std::sync::mpsc::{channel, Sender};
use crate::config::argument::Argument;
use crate::service::port_sniffer::scan_for_available_port;

mod config;
mod service;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let arguments = match Argument::new(&cli_args) {
        Ok(a) => a,
        Err(e) => {
            if e.contains("help") {
                println!("program usage is\r\n '-ip' desired ip address\r\n '-t' count of threads");
                process::exit(1)
            } else {
                eprintln!("problem with parsing arguments: '{}'", e);
                process::exit(-1)
            }
        }
    };

    let (sender, receiver) = channel();
    for i in 0..arguments.get_thread_count() {
        let thread_sender: Sender<u16> = sender.clone();
        let ip = arguments.get_ip_address();
        let number_thread = arguments.get_thread_count();
        thread::spawn(move || {
            scan_for_available_port(thread_sender, i, ip, number_thread)
        });
    }
    drop(sender);

    let mut output: Vec<u16> = vec![];

    for v in receiver {
        output.push(v)
    }
    println!("\nAvailable ports found:");
    for port in output {
        println!("{port}");
    }
}
