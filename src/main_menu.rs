use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use std::path::Path;

pub fn draw_main_menu(h : &mut RaylibDrawHandle, screen_width : i32, menu_counter : &mut u32, start : &mut bool, quit : &mut bool, filename : &mut String)
{   
    //CHOOSE COLOR FOR MENU
    let color_start : Color;
    let color_load : Color;
    let color_quit : Color;
    match *menu_counter
    {
        0 =>
        {
            color_start = Color::GREEN;
            color_load = Color::WHITE;
            color_quit = Color::WHITE;
        },
        1 =>
        {
            color_start = Color::WHITE;
            color_load = Color::GREEN;
            color_quit = Color::WHITE;
        },
        2 =>
        {
            color_start = Color::WHITE;
            color_load = Color::WHITE;
            color_quit = Color::GREEN;
        },
        _ =>
        {
            color_start = Color::WHITE;
            color_load = Color::WHITE;
            color_quit = Color::WHITE;
        }
    }


    //DRAW MAIN MENU
    let text_width : i32 = measure_text("MAIN MENU", 60);
    h.draw_text("MAIN MENU", screen_width/2 - (text_width/2), 20, 60, Color::WHITE);
    
    if filename.is_empty()
    {
        let text_width : i32 = measure_text("No file selected", 40);
        h.draw_text("No file selected", screen_width/2 - (text_width/2), 100, 40, Color::RED);

        let text_width : i32 = measure_text("START", 40);
        h.draw_text("START", screen_width/2 - (text_width/2), 250, 40, Color::GRAY);
    }
    else
    {
        let rom_name = Path::new(&filename).file_name().unwrap().to_os_string().into_string().unwrap(); //Get the rom name
        let text_width : i32 = measure_text(&rom_name, 20);
        h.draw_text(&rom_name, screen_width/2 - (text_width/2), 100, 20, Color::WHITE);

        let text_width : i32 = measure_text("START", 40);
        h.draw_text("START", screen_width/2 - (text_width/2), 250, 40, color_start);
    }

    let text_width : i32 = measure_text("LOAD GAME FILE", 40);
    h.draw_text("LOAD GAME FILE", screen_width/2 - (text_width/2), 300, 40, color_load);

    let text_width : i32 = measure_text("QUIT", 40);
    h.draw_text("QUIT", screen_width/2 - (text_width/2), 350, 40, color_quit);

    //CHOOSE MENU OPTION
    if h.is_key_pressed(KEY_ENTER)
    {
        match *menu_counter
        {
            0 => 
            {
                if !filename.is_empty()
                {
                    *start = true;
                } 
            },
            1 => 
            {
                let file = tinyfiledialogs::open_file_dialog(
                    "Open",
                    "",
                    Some((&["*.gb", "*.gbc"], ".gb or .gbc")));
                filename.clear();
                filename.push_str(file.unwrap().as_str());
            },
            2 => 
            {
                *quit = true;
            },
            _ => 
            {
                *menu_counter = 1;
            }
        }
    }
    else if h.is_key_pressed(KEY_DOWN)
    {
        if *menu_counter < 2{*menu_counter += 1;}
    }
    else if h.is_key_pressed(KEY_UP)
    {
        if *menu_counter > 0{*menu_counter -= 1;}
        if *menu_counter == 0 && filename.is_empty()
        {
            *menu_counter = 1;
        }
    }
}