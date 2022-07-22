use raylib::prelude::*;

mod main_menu;
mod cartridge;
mod emulator;

const WIDTH : i32 = 160;
const HEIGTH : i32 = 144;
const BACKGROUND_COLOR : Color = Color::new(45, 52, 54, 255);
fn main() 
{
    //WINDOW
    const MULTIPLIER : i32 = 4;
    const SCREEN_WIDTH : i32 = WIDTH * MULTIPLIER; 
    const SCREEN_HEIGTH : i32 = HEIGTH * MULTIPLIER ; 

    let (mut rl, thread) = raylib::init()
    .size(SCREEN_WIDTH, SCREEN_HEIGTH)
    .title("GAMEBOY EMULATOR")
    .build();

    //MAIN MENU
    let mut filename : String = String::new();
    let mut menu_counter : u32 = 1;
    let mut start_game : bool = false;
    let mut quit : bool = false;

    //EMULATION
    let mut emulator = emulator::init_emulator();

    //MAIN LOOP
    while !rl.window_should_close()
    {
        //WINDOW
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        if start_game
        {
            //EMULATION starts here

            //TO DO

            quit = true;
        }
        else
        {   
            //MAIN MENU
            main_menu::draw_main_menu(&mut d, SCREEN_WIDTH, &mut menu_counter, &mut start_game, &mut quit, &mut filename);

            //IF GAME CHOSEN LOAD THE ROM
            if start_game
            {   
                emulator.load_rom(&filename);
            }
        }
        
        if quit{break;}
    } 
}