use raylib::prelude::*;

mod main_menu;

const WIDTH : i32 = 160;
const HEIGTH : i32 = 144;
const BACKGROUND_COLOR : i32 = 0x2d3436;

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

    //MAIN LOOP
    while !rl.window_should_close()
    {
        //WINDOW
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::get_color(BACKGROUND_COLOR));

        if start_game
        {
            //EMULATION starts here

        }
        else
        {   
            //MAIN MENU
            main_menu::draw_main_menu(&mut d, SCREEN_WIDTH, &mut menu_counter, &mut start_game, &mut quit, &mut filename);
        }
        
        if quit{break;}
    } 
}