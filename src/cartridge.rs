//Imports


// Constants
const NINTENDO_LOGO : [u8;48] = 
[
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 
    0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 
    0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 
    0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

const CART_TYPE : [&str; 35] = 
[
    "ROM ONLY", "MBC1", "MBC1+RAM", "MBC1+RAM+BATTERY", "0x04 ???",
    "MBC2", "MBC2+BATTERY", "0x07 ???", "ROM+RAM 1", "ROM+RAM+BATTERY 1", 
    "0x0A ???", "MMM01", "MMM01+RAM", "MMM01+RAM+BATTERY", "0x0E ???",
    "MBC3+TIMER+BATTERY", "MBC3+TIMER+RAM+BATTERY 2", "MBC3", "MBC3+RAM 2", 
    "MBC3+RAM+BATTERY 2", "0x14 ???", "0x15 ???", "0x16 ???", "0x17 ???",
    "0x18 ???", "MBC5", "MBC5+RAM", "MBC5+RAM+BATTERY", "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM", "MBC5+RUMBLE+RAM+BATTERY", "0x1F ???", "MBC6", "0x21 ???",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
];

// Cartridge Header struct
pub struct cartridge_header
{
    pub logo : [u8;48],                 //size = 0x30
    pub title : [u8; 16],               //16 ASCII Characters
    pub manufacturer_code : [u8; 4],    //4 ASCII Characters,     
    pub cgb_flag : u8,                  //GBC Mode or not
    pub sgb_flag : u8,                  //SGB Mode or not
    pub cartridge_type : u8,            //Cartridge Type (ROM_ONLY, MBC1,..)
    pub rom_size : u8,                  //Calculated as 32KB << n
    pub ram_size : u8,                  //(0, 8, 32, 128, 64)KB
    pub mask_rom_version : u8,          //Version Number of the Game
    pub header_checksum : u8,           //Checksum of the header
    pub global_checksum : u16,          //Checksum of the entire cartridge (Not need to be verified)
}

pub fn init_header() -> cartridge_header
{
    cartridge_header
    {
        logo : [0;48],
        title : [0;16],
        manufacturer_code : [0;4],
        cgb_flag : 0,
        sgb_flag : 0,
        cartridge_type : 0,
        rom_size : 0,
        ram_size : 0,
        mask_rom_version : 0,
        header_checksum : 0,
        global_checksum : 0,
    }
}

//Cartridge struct
pub struct cartridge
{
    pub filename : String,
    pub size : u32,
    pub data : Vec<u8>,
    pub header : cartridge_header,
}

pub fn init_cartridge() -> cartridge
{
    cartridge
    {
        filename : String::new(),
        size : 0,
        data : Vec::new(),
        header : init_header(),
    }
}

//Cartridge methods
impl cartridge
{
    pub fn load_cartridge(&mut self, filename : &str, data : Vec<u8>)
    {
        self.filename = filename.to_string();
        
        self.data = data;
        self.get_header();
    }
    
    pub fn get_header(&mut self)
    {
        self.header.logo.copy_from_slice(&self.data[0x104..0x134]);
        self.header.title.copy_from_slice(&self.data[0x134..0x144]);
        println!("Title: {}", String::from_utf8(self.header.title.to_vec()).unwrap());
    }
}