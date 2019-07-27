use std::sync::mpsc::Sender;

use super::comms;

pub fn parse(line: String, sender: &Sender<usize>) {
	println!("{:?}", line);
	if line == "4" {
		sender.send(comms::ARD_STARTED_UP).expect("Failed send");
	}
}