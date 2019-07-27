use std::sync::mpsc::Receiver;

use super::comms;

pub struct Writer {
    port: std::boxed::Box<(dyn serialport::SerialPort + 'static)>
}

impl Writer {
	pub fn new(port: std::boxed::Box<(dyn serialport::SerialPort + 'static)>) -> Writer {
		Writer { port: port }
	}

    pub fn write(&mut self, data: String) -> Result<(),&str> {
        match self.port.write((data + "\n").as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to write to port")
        }
    }
}

pub fn write_loop(writer: &mut Writer, receiver: Receiver<usize>) {
	
	if receiver.recv().expect("Arduino failed to start up") == comms::ARD_STARTED_UP {
		writer.write("3".to_string()).expect("Failed to write to port");
    	println!("Written startup");
		loop {
			match receiver.try_recv() {
				Ok(n) => {
					if n == comms::EXIT {
						println!("Exiting write thread");
						return;
					}
				},
				Err(_) => ()
			}
		}
	}
}