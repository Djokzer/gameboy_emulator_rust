use crate::cartridge;
use std::{fs::{self, metadata, File}, io::Read};

pub struct emulator
{
	pub cart : cartridge::cartridge,
	//TO DO
}

pub fn init_emulator() -> emulator
{
	emulator
	{
		cart : cartridge::init_cartridge(),
	}
}

impl emulator
{
	pub fn load_rom(&mut self, filename : &str)
	{
		//READ ROM FILE
		let mut f = File::open(filename).expect("no file found");
        let metadata = metadata(filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

		//LOAD THE ROM DATA
		self.cart.load_cartridge(filename, buffer);
	}
}


