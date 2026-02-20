#![windows_subsystem = "windows"]

use fastnoise_lite::{CellularDistanceFunction, CellularReturnType, FastNoiseLite};
use rand::Rng;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::image::*;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseState;
use sdl2::rect::{FRect, Rect};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use sdl2::pixels::{Color, PixelFormatEnum};



use crate::cell_update::update_cell;

pub mod pixels;
pub mod cell_update;
pub mod structs;
pub mod entity;
pub mod shift;


const CELL_AMOUNT:usize = 10;
const REGION_AMOUNT:usize = 5;
const ACTIVE_REGS: usize = 3;
const CELL_SIZE:usize = 128;
const PITCH_SIZE:usize= 4;
const PITCH:usize = CELL_SIZE*PITCH_SIZE;
const PIXEL_DATA_SIZE:usize = CELL_SIZE*PITCH;


const OFFSET_X:usize = 3;
const OFFSET_Y:usize = 4;

const CENT_ACT_REG32:i32 = (CELL_AMOUNT*CELL_AMOUNT*ACTIVE_REGS + CELL_AMOUNT) as i32;


use crate::entity::{Entity, check_collision, update_player};
use crate::shift::{make_big_cells, update_cells_x, update_cells_y, update_regions_x, update_regions_y};
use crate::structs::Region;
use crate::{structs::Cell, structs::DirtyRect};



fn place_line(cells: &mut Vec<Arc<Mutex<Cell>>>,mut x1:i32,x2:i32,mut y1:i32,y2:i32,id:u8, color:[u8;3], brush_size:i32){
    if x1 == x2 && y1 == y2{
        for i in -(brush_size/2-1)..(brush_size/2+1){
            for j in -(brush_size/2-1)..(brush_size/2+1){
                let cell = &mut cells[((x1+j).abs()/(CELL_SIZE as i32) +((y1+i).abs()/(CELL_SIZE as i32)*CELL_AMOUNT as i32)) as usize].lock().unwrap();
                if cell.grid[((x1+j).abs()%(CELL_SIZE as i32)) as usize + ((y1+i).abs()%(CELL_SIZE as i32) )as usize*CELL_SIZE].id == 0 || id ==0{
                    cell.place_pixel(((x1+j).abs()%(CELL_SIZE as i32)) as usize, ((y1+i).abs()%(CELL_SIZE as i32)) as usize, id, color);		   
                }
            }
        }
        return;
    }
    let dx = (x2-x1).abs();
    let dy = (y2-y1).abs();
    let mut sx = -1;
    let mut sy = -1;
    let mut err = dx -dy;
    if x1 < x2{
        sx = 1;
    }
    if y1 < y2{
        sy = 1;
    }
    while x1 != x2 || y1 != y2{
        for i in -(brush_size/2-1)..(brush_size/2+1){
            for j in -(brush_size/2-1)..(brush_size/2+1){
                let cell = &mut cells[((x1+j).abs()/(CELL_SIZE as i32) +((y1+i).abs()/(CELL_SIZE as i32)*CELL_AMOUNT as i32)) as usize].lock().unwrap();
                if cell.grid[((x1+j).abs()%(CELL_SIZE as i32)) as usize + ((y1+i).abs()%(CELL_SIZE as i32))as usize*CELL_SIZE].id == 0 || id ==0{
                    cell.place_pixel(((x1+j).abs()%(CELL_SIZE as i32)) as usize, ((y1+i).abs()%(CELL_SIZE as i32)) as usize, id, color);
                }
            }
        }
        let err2 = 2 *err;
        if err2 > -dy{
            err -= dy;
            x1 += sx;
        }
        if err2 < dx{
            err += dx;
            y1 += sy;
        }
    }
}


pub fn get_biome(biome_noise1:&FastNoiseLite,biome_noise2:&FastNoiseLite,x:f32, y:f32) -> u32{
    let biome_size = 200.;
    return (biome_noise1.get_noise_2d(x ,  y)*biome_size* biome_noise2.get_noise_2d(x, y)) as u32/8;

}

pub fn update_map(regions:&Vec<Region>, biome_noise1:&FastNoiseLite,biome_noise2:&FastNoiseLite) -> Vec<u8>{
    let mut map_data = vec![];
    let x = regions[REGION_AMOUNT*REGION_AMOUNT/2].position[0];
    let y = regions[REGION_AMOUNT*REGION_AMOUNT/2].position[1];
    for i in 0..CELL_SIZE{
        for j in 0..CELL_SIZE{
            let biome = get_biome(biome_noise1,  biome_noise2,(x + j as i32 - CELL_SIZE as i32/2) as f32, (y + i as i32 - CELL_SIZE as i32/2) as f32);
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            match biome{
                0=>{

                }
                1=>{
                    r = 255
                }
                2 =>{
                    g = 255
                }
                3 =>{
                    b = 255
                }
                4=>{
                    g = 255;
                    r = 255
                }
                5 =>{
                    r = 255;
                    g = 255
                }
                6 =>{
                    r = 255;
                    b = 255
                }
                _=>{
                    r = ((biome_noise1.get_noise_2d((i as i32+y) as f32, (j as i32+x) as f32) + 0.6)*16.) as u8/4 *80;
                    g = 255 -((biome_noise1.get_noise_2d((i as i32+y) as f32, (j as i32+x) as f32) + 0.6)*16.) as u8/4 *80;
                    b = 128 -((biome_noise1.get_noise_2d((i as i32+y) as f32, (j as i32+x) as f32) + 0.6)*16.) as u8/4 *100
                }
            }
            if j  >= CELL_SIZE/2 && j <= CELL_SIZE/2 && i >= CELL_SIZE/2 && i<= CELL_SIZE/2{
                map_data.push(255);
                map_data.push(255);
                map_data.push(255);
                map_data.push(255);
            } 
            else {
                map_data.push(b);
                map_data.push(g);
                map_data.push(r);
                map_data.push(255);
            }
        }
    }
    return map_data;
}


pub fn screenshot(big_cells: &Vec<Arc<Mutex<Cell>>>){
    let mut screenshot_data = vec![];
    for i in 0..CELL_AMOUNT*ACTIVE_REGS{
        for y in 0..CELL_SIZE{
            for j in 0..CELL_AMOUNT*ACTIVE_REGS{
                for x in 0..PITCH{
                    let cell = big_cells[i * CELL_AMOUNT * ACTIVE_REGS + j].lock().unwrap();
                    screenshot_data.push(cell.pixel_data[x + y*PITCH].clone());
                }
            }
        }
    }

    let surface = Surface::from_data(&mut screenshot_data, (CELL_AMOUNT*CELL_SIZE*ACTIVE_REGS) as u32, (CELL_AMOUNT*CELL_SIZE*ACTIVE_REGS) as u32, (PITCH*CELL_AMOUNT*ACTIVE_REGS) as u32, PixelFormatEnum::ARGB8888).unwrap();
    surface.convert_format(PixelFormatEnum::BGR888).unwrap();
    surface.save("Prikol.png").unwrap();
}

pub fn main(){
    let screen_scale: f32 = 4.0;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();


    let mut window = video_subsystem.window("PlanetPixel", 1280, 720)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.clone().into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    sdl_context.mouse().show_cursor(false);

    canvas.set_scale(screen_scale, screen_scale).unwrap();

    let texture_creator = canvas.texture_creator();

    let mut mouseinfo_prev = [0,0];
    let mut brush_size:i32 = 16;

    
    let font = ttf_context.load_font("src/Fonts/Sans.ttf", 24).unwrap();



    let mut debug_surface;



    let bg_texture = texture_creator.load_texture( "src/Sprites/bg.png").unwrap();




    let mut cell_texture = texture_creator.create_texture(PixelFormatEnum::ARGB8888, sdl2::render::TextureAccess::Static, (CELL_SIZE*CELL_AMOUNT) as u32, (CELL_SIZE*CELL_AMOUNT) as u32).unwrap();
    
    
    cell_texture.set_blend_mode(sdl2::render::BlendMode::Blend);


    let mut biome_noise1 = FastNoiseLite::new();
    let mut biome_noise2 = FastNoiseLite::new();
    biome_noise2.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin));

    biome_noise1.set_frequency(Some(0.1));
    biome_noise2.set_frequency(Some(0.1));

    biome_noise1.set_noise_type(Some(fastnoise_lite::NoiseType::Cellular));
    biome_noise1.set_cellular_distance_function(Some(CellularDistanceFunction::Euclidean));
    biome_noise1.set_cellular_return_type(Some(CellularReturnType::Distance2Sub));
    biome_noise1.set_cellular_jitter(Some(1.800));

    let texture_surface = Surface::from_file("src/Sprites/rock.png").unwrap().into_canvas().unwrap();
    let texture_data = texture_surface.read_pixels(None, PixelFormatEnum::ARGB8888).unwrap();


    let mut slot = 1;

    let player_speed = 0.5;
    let mut player = Entity{
        size:[12,24],
        position:[0.,0.],
        speed: [0.,0.],
        is_on_floor:false,
        flip_h:false,
        animation:2,
    };
    
    let mut camera = FRect::new((player.position[0]/(CELL_SIZE*CELL_AMOUNT) as f32).floor() *(CELL_AMOUNT*CELL_SIZE) as f32, (player.position[1]/(CELL_SIZE*CELL_AMOUNT) as f32).floor() *(CELL_AMOUNT*CELL_SIZE) as f32, 480., 270.);



    let player_texture = texture_creator.load_texture("src/Sprites/player_sheet.png").unwrap();

    let mut map_on_screen = false;
    


    window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap(); 
    canvas.set_viewport(Rect::new(0, 0, camera.width() as u32, camera.height() as u32));
    
    let mut regions: Vec<Region> = vec![];
    for i in 0..REGION_AMOUNT*REGION_AMOUNT{
        let mut region = Region{
            id: i,
            generated: true,
            biome_changed:true,
            cell_to_generate: 0,
            position:[(i % REGION_AMOUNT) as i32 - REGION_AMOUNT as i32 /2 + (player.position[0]/(CELL_SIZE*CELL_AMOUNT) as f32) as i32,  (i / REGION_AMOUNT) as i32 - REGION_AMOUNT as i32 /2 + (player.position[1]/(CELL_SIZE*CELL_AMOUNT) as f32) as i32],
            cells: vec![], 
            biome: 0
        };
        region.biome = get_biome(&biome_noise1, &biome_noise2,region.position[0] as f32, region.position[1] as f32);
        region.generate(&texture_data);
        regions.push(region);
    };
    let mut big_cells = vec![];
    big_cells.reserve(CELL_AMOUNT*CELL_AMOUNT*ACTIVE_REGS*ACTIVE_REGS);
    
    make_big_cells(&mut regions, &mut big_cells);


    let mut map_texture = texture_creator.create_texture_static(PixelFormatEnum::ARGB8888, CELL_SIZE as u32, CELL_SIZE as u32).unwrap();
    map_texture.update(None,&update_map(&regions,&biome_noise1,&biome_noise2),PITCH).unwrap();

    let mut cells = regions[REGION_AMOUNT*REGION_AMOUNT/2].cells.clone();

    let mut shift_amount = [(camera.x()/(CELL_SIZE) as f32) as i32,(camera.y()/(CELL_SIZE) as f32) as i32];
    let mut shift_position = [0,0];
    let mut region_shift_position = [0,0];
    

    let mut time = 0.;
    let mut tickle = 0;

    let mut debug = false;
    'running: loop {
        
        time += 0.016;


        use std::time::Instant;
        let now = Instant::now();
        let mut rng = rand::rng();
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F4), .. } => {
                    if window.fullscreen_state() ==  sdl2::video::FullscreenType::Desktop{
                        window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap(); 
                    }
                    else {
                        window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap(); 
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } =>{
                    slot = 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } =>{
                    slot = 2;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } =>{
                    slot = 3;
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } =>{
                    slot = 4;
                },

                Event::KeyDown { keycode: Some(Keycode::M), ..} =>{
                    map_on_screen = !map_on_screen
                },




                Event::MouseWheel { y, .. } =>{
                    brush_size += y;
                    brush_size = brush_size.max(1);
                    brush_size = brush_size.min(CELL_SIZE as i32);
                    
                },

                Event::KeyDown { keycode: Some(Keycode::F12), .. } =>{
                    screenshot(&big_cells);
                },
                Event::KeyDown { keycode: Some(Keycode::Num0), .. } =>{
                    debug = !debug;
                },
                _ => {}
            }
        }

        camera.set_x(camera.x() + (player.position[0] - camera.width()/2. -camera.x())*0.075);
        camera.set_y(camera.y() + (player.position[1] - camera.height()/2. -camera.y())*0.075);

        let mouse_info = MouseState::new(&event_pump);
        let mouse_info_x = (mouse_info.x())/screen_scale as i32 + camera.x() as i32 % CELL_SIZE as i32 + (CELL_SIZE * OFFSET_X) as i32;
        let mouse_info_y = (mouse_info.y())/screen_scale as i32 + camera.y() as i32 % CELL_SIZE as i32 + (CELL_SIZE * OFFSET_Y) as i32;

        if shift_amount[0] != camera.x() as i32/CELL_SIZE as i32{
            let shift_delta = [-(shift_amount[0] - camera.x() as i32/CELL_SIZE as i32), -(shift_amount[1] - camera.y() as i32/(CELL_SIZE) as i32)];

            shift_position[0] += shift_delta[0];
            region_shift_position[0] += shift_delta[0];
            shift_position[0] %= CELL_AMOUNT as i32 ;
            mouseinfo_prev[0] -= CELL_SIZE as i32*shift_delta[0];

            if  region_shift_position[0].abs() >= CELL_AMOUNT as i32{
                region_shift_position[0] -= CELL_AMOUNT as i32 * region_shift_position[0].signum();
                update_regions_x(&mut regions, shift_delta[0]);
                make_big_cells(&mut regions, &mut big_cells);
                map_texture.update(None,&update_map(&regions,&biome_noise1,&biome_noise2),PITCH).unwrap();
            }
            if shift_delta[0] != 0{
                update_cells_x(&mut cells, &mut big_cells, shift_delta, shift_position);
                shift_amount[0] = camera.x() as i32/CELL_SIZE as i32;
            }

        }


        if shift_amount[1] != camera.y() as i32/CELL_SIZE as i32{
            let shift_delta = [-(shift_amount[0] - camera.x() as i32/CELL_SIZE as i32), -(shift_amount[1] - camera.y() as i32/(CELL_SIZE) as i32)];


            shift_position[1] += shift_delta[1];
            region_shift_position[1] += shift_delta[1];
            shift_position[1] %= CELL_AMOUNT as i32 ;
            mouseinfo_prev[1] -= CELL_SIZE as i32*shift_delta[1];

            if region_shift_position[1].abs() == CELL_AMOUNT as i32{
                region_shift_position[1] -= CELL_AMOUNT as i32 * region_shift_position[1].signum();
                update_regions_y(&mut regions, shift_delta[1]);
                make_big_cells(&mut regions, &mut big_cells);
                map_texture.update(None,&update_map(&regions,&biome_noise1,&biome_noise2),PITCH).unwrap();
            }

            if shift_delta[1] != 0{
                update_cells_y(&mut cells, &mut big_cells, shift_delta, shift_position);
                shift_amount[1] = camera.y() as i32/CELL_SIZE as i32;
            }


        }
        

        if mouse_info_x >= 0 && mouse_info_x <= (CELL_AMOUNT*CELL_SIZE-1) as i32 && mouse_info_y >= 0 && mouse_info_y <= (CELL_AMOUNT*CELL_SIZE-1) as i32{
            if mouse_info.left(){
                let saturation = rng.random_range(0..40);
                match slot {
                    1 =>{place_line(&mut cells, mouseinfo_prev[0], mouse_info_x, mouseinfo_prev[1], mouse_info_y, slot, [128 + 20 + saturation,200,200],brush_size)},
                    2 =>{place_line(&mut cells, mouseinfo_prev[0], mouse_info_x, mouseinfo_prev[1], mouse_info_y, slot, [20 + saturation,20 + saturation, 20 + saturation],brush_size)},
                    3 =>{place_line(&mut cells, mouseinfo_prev[0], mouse_info_x, mouseinfo_prev[1], mouse_info_y, slot, [180,50,50],brush_size)},
                    4 =>{place_line(&mut cells, mouseinfo_prev[0], mouse_info_x, mouseinfo_prev[1], mouse_info_y, slot, [40,180,40],brush_size)},
                    _=>{}
                }
            }
            if mouse_info.right(){
                place_line(&mut cells, mouseinfo_prev[0], mouse_info_x, mouseinfo_prev[1], mouse_info_y, 0, [70,70,70],brush_size);
            }
            mouseinfo_prev = [mouse_info_x, mouse_info_y];
        }
        
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Space) && player.is_on_floor{
            player.speed[1] = -8.0;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::A){
            player.speed[0] = player.speed[0].min(0.);
            player.speed[0] -= player_speed;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::D){
            player.speed[0] = player.speed[0].max(0.);
            player.speed[0] += player_speed;
        }
        if !event_pump.keyboard_state().is_scancode_pressed(Scancode::D) && !event_pump.keyboard_state().is_scancode_pressed(Scancode::A){
            player.speed[0] = (player.speed[0]).abs()*0.01*player.speed[0].signum()
        }

        check_collision(&mut player, &mut big_cells, regions[REGION_AMOUNT +1].position);

        for i in 0..REGION_AMOUNT*REGION_AMOUNT{
            if !regions[i].biome_changed{
                regions[i].biome =  get_biome(&biome_noise1,&biome_noise2, regions[i].position[0] as f32, regions[i].position[1] as f32);
                regions[i].biome_changed = true;
            }
            if !regions[i].generated && i != 0 && i != REGION_AMOUNT-1 && i != REGION_AMOUNT*REGION_AMOUNT-1 && i != REGION_AMOUNT*(REGION_AMOUNT-1){
                regions[i].generate_cell(&texture_data);
            }
            if i%REGION_AMOUNT > 0 && i%REGION_AMOUNT < REGION_AMOUNT-1 && i/REGION_AMOUNT > 0 && i/REGION_AMOUNT < REGION_AMOUNT-1{
                canvas.copy_f(&bg_texture, None, FRect::new(-camera.x() + (regions[i].position[0]*(CELL_AMOUNT*CELL_SIZE) as i32) as f32, -camera.y() + (regions[i].position[1]*(CELL_AMOUNT*CELL_SIZE) as i32) as f32, (CELL_AMOUNT*CELL_SIZE) as f32, (CELL_AMOUNT*CELL_SIZE) as f32)).unwrap();

            }
        }
        
        if player.speed[0] as i32 > 0{
            player.flip_h = false;
            player.animation = 0;
        }
        else if (player.speed[0] as i32) < 0{
            player.flip_h = true;
            player.animation = 0;
        }

        if player.speed[0] as i32 == 0{
            player.animation = 2;
        }

        canvas.copy_ex_f(&player_texture, Rect::new(((time*16.) as i32%4)*(player.size[0]+2) as i32, player.animation as i32*(player.size[1]+2) as i32, player.size[0] as u32, player.size[1] as u32), FRect::new(player.position[0]-camera.x(), player.position[1]-camera.y(), player.size[0] as f32, player.size[1] as f32),0., None,player.flip_h,false).unwrap();
        update_player(&mut player);

        tickle %= 16;
        for j in (0..4).rev(){
            let mut threads = vec![];

            for i in (j/2..CELL_AMOUNT).step_by(2){
                for z in (j%2..CELL_AMOUNT).step_by(2){
                    let mut cells = cells.clone();
                    let cell_arc = cells[i*CELL_AMOUNT + z].clone();
                    let cell_arc2 = cells[i*CELL_AMOUNT + z].clone();
                    let mut cell = cell_arc2.lock().unwrap();

                    cell.id = i*CELL_AMOUNT + z;
                    if cell.id%16 == tickle{
                        cell.rect = DirtyRect{x1:0, x2:CELL_SIZE,y1:0,y2:CELL_SIZE};
                        cell.calculated_rect = DirtyRect{x1:0, x2:CELL_SIZE,y1:0,y2:CELL_SIZE};
                    }
                    if cell.rect.x1 + cell.rect.y1 + cell.rect.x2 + cell.rect.y2 != 0{
                        let thread = thread::spawn(move ||{
                        let cells = &mut cells;
                        let cell = &mut cell_arc.lock().unwrap();
                        update_cell(cell, cells);});
                        threads.push(thread)
                    }
                }
            }
            for thread in threads{
                thread.join().unwrap();
            };
        }
        tickle += 1;


        for i in 2..CELL_AMOUNT-2{
            for j in 2..CELL_AMOUNT-2{
                let cell_position = [j as i32, i  as i32];
                let cell_arc = &cells[i*CELL_AMOUNT + j];
                let cell = &cell_arc.lock().unwrap();	
		cell_texture.update( Rect::new(cell_position[0]*CELL_SIZE as i32, cell_position[1]*CELL_SIZE as i32, CELL_SIZE as u32, CELL_SIZE as u32), &cell.pixel_data, PITCH).unwrap();
            }

        }	

        canvas.copy_f(&cell_texture, None,FRect::new( -camera.x %CELL_SIZE as f32 - (CELL_SIZE*OFFSET_X) as f32, -camera.y %CELL_SIZE as f32 - (CELL_SIZE*OFFSET_Y) as f32,(CELL_SIZE*CELL_AMOUNT) as f32, (CELL_SIZE*CELL_AMOUNT) as f32)).unwrap();
        if debug{
            for i in 0..(CELL_AMOUNT*CELL_AMOUNT){
                canvas.set_draw_color(Color::MAGENTA);
                let cell_position = [(i % CELL_AMOUNT) as i32, (i / CELL_AMOUNT) as i32];
                let cell_arc = &cells[i];
                let cell = &cell_arc.lock().unwrap();
                canvas.draw_rect(Rect::new(cell.rect.x1 as i32 + cell_position[0]*CELL_SIZE as i32 + (-camera.x %CELL_SIZE as f32 - (CELL_SIZE*OFFSET_X) as f32) as i32 , cell.rect.y1 as i32 + cell_position[1]*CELL_SIZE as i32 + (-camera.y %CELL_SIZE as f32 - (CELL_SIZE*OFFSET_Y) as f32) as i32,(cell.rect.x2 as i32 -cell.rect.x1 as i32) as u32, (cell.rect.y2 as i32 -cell.rect.y1 as i32) as u32)).unwrap();
            }
        }

        let elapsed = now.elapsed();
        debug_surface = font.render(&((1.0 / elapsed.as_secs_f64()).round().to_string() + " FPS")).solid(Color::WHITE).unwrap();
        canvas.copy(&debug_surface.as_texture(&texture_creator).unwrap(), None, Rect::new( 0, 0,60, 20)).unwrap();

        debug_surface = font.render(&(player.position[0].to_string() + " x")).solid(Color::WHITE).unwrap();
        canvas.copy(&debug_surface.as_texture(&texture_creator).unwrap(), None, Rect::new( 0, 20,60, 20)).unwrap();

        debug_surface = font.render(&(player.position[1].to_string() + " y")).solid(Color::WHITE).unwrap();
        canvas.copy(&debug_surface.as_texture(&texture_creator).unwrap(), None, Rect::new( 0, 40,60, 20)).unwrap();

        if map_on_screen{
            canvas.copy(&map_texture, None, Rect::new(camera.width() as i32 - CELL_SIZE as i32,0, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
        }


        canvas.set_draw_color(Color::WHITE);
        canvas.draw_frect(FRect::new( mouse_info.x() as f32 /screen_scale - (brush_size /2) as f32, mouse_info.y() as f32/screen_scale - (brush_size /2) as f32, brush_size as f32, brush_size as f32)).unwrap();
        


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 16000000 - (elapsed.as_secs_f64()*1000000.0) as u32));

    }

}
