extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;
use std::{thread, mem};
use std::sync::mpsc::{channel, Sender, Receiver};

use serialport::prelude::*;

mod comms;
mod krpc;

use krpc::bindings::{krpc_services_KRPC,krpc_connect};


mod parse;
use parse::parse;

mod write;
use write::{Writer, write_loop};

fn main() {
    let port_name = "/dev/tty.usbmodem14201";
    let baud_rate = "115200";

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            run(port);
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

fn run(mut serialport: std::boxed::Box<dyn serialport::SerialPort>){
    // Clone the port
    let mut clone = serialport.try_clone().expect("Failed to clone");
    if let Some(name) = serialport.name(){
        println!("Connected to {}", name);
    }

    let (sender, reciever): (Sender<usize>, Receiver<usize>) = channel();

    // Send out 4 bytes every second
    let handle = thread::spawn(move || {
        write_loop(&mut Writer::new(clone), reciever);
    });

    // Read the four bytes back from the cloned port
    let mut byte_buffer: [u8; 1] = unsafe { mem::uninitialized() };
    let mut line = String::new();
    loop {
        match serialport.read(&mut byte_buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    //println!("Received: {:?}", byte_buffer);
                    let last_char = &String::from_utf8_lossy(&byte_buffer);
                    if last_char == "\n" {
                        line.pop();
                        parse(line, &sender);
                        line = String::new();
                    }else{
                        line.push_str(last_char);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                println!("Broken pipe, exiting");
                sender.send(comms::EXIT).expect("Failed to send exit signal");
                break;
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
    handle.join();
}

