use std::sync::{Arc, Mutex};


use rand::Rng;

use crate::{CELL_SIZE, Cell, DirtyRect, PITCH, PITCH_SIZE, pixels::{try_grow, try_grow_destroy, try_move, TRANSPARENCY}};

pub fn update_cell(cell:&mut Cell, cells: &mut Vec<Arc<Mutex<Cell>>>){
    let mut rng = rand::rng();
    let iter_x = cell.calculated_rect.x1..cell.calculated_rect.x2;
    let iter_y = cell.calculated_rect.y1..cell.calculated_rect.y2; 
    let old_rect = cell.rect;
    cell.rect = DirtyRect{x1:0, x2:0,y1:0,y2:0};
    for i in (iter_y).rev(){
        for z in iter_x.clone(){
            let mut pixel = cell.grid[i*CELL_SIZE + z];
            let random_num = rng.random_range(0..255);
            if pixel.iteration != random_num{
                for j in 0..3{
                    cell.pixel_data[i*PITCH + z*PITCH_SIZE + j]  = pixel.color[j];
                }
                cell.pixel_data[i*PITCH + z*PITCH_SIZE + 3] = TRANSPARENCY[pixel.id as usize];
                pixel.iteration = random_num;
                match pixel.id {
                    1 => {
                        if random_num > 1{
                            if try_move(cell, cells, z, i, z as isize, i as isize + pixel.speed[1] as isize, random_num){
                                continue;
                            }
                            if random_num > 128{
                                if try_move(cell, cells, z, i, z as isize - pixel.speed[0] as isize, i as isize + pixel.speed[1] as isize, random_num){
                                    continue;
                                }
                            }
                            if try_move(cell, cells, z, i, z as isize + pixel.speed[0] as isize, i as isize + pixel.speed[1] as isize, random_num){
                                continue;
                            }
                            if try_move(cell, cells, z, i, z as isize - pixel.speed[0] as isize, i as isize + pixel.speed[1] as isize, random_num){
                                continue;
                            }

                        }
                        else {
                            if random_num > 128{
                                if try_move(cell,cells, z, i, z as isize - pixel.speed[0] as isize, i as isize + pixel.speed[1] as isize, random_num){
                                    continue;
                                }
                            }
                            if try_move(cell,cells, z, i, z as isize + pixel.speed[0] as isize, i as isize + pixel.speed[1] as isize, random_num){
                                continue;
                            }
                            if try_move(cell,cells, z, i, z as isize - pixel.speed[0] as isize, i as isize + pixel.speed[1] as isize, random_num){
                                continue;
                            }
                            if try_move(cell,cells, z, i, z as isize, i as isize + pixel.speed[1] as isize, random_num){
                                continue;
                            }
                        }
                        cell.grid[i*CELL_SIZE + z].speed = [1,1];
                    },
                    3=>
                        {
                        
                            if random_num %24 == 4{
                                if try_move(cell, cells, z, i, z as isize +random_num as isize%8-3 , i as isize, random_num){
                                    continue;
                                }
                                if try_move(cell, cells, z, i, z as isize, i as isize + pixel.speed[1] as isize, random_num){
                                    continue;
                                }
                                cell.grid[i*CELL_SIZE + z].speed[1] = 1;
                                if try_move(cell,cells, z, i, z as isize + random_num as isize%8-3, i as isize + pixel.speed[1] as isize, random_num){
                                    continue;
                                }

                            }
                            else {
                                
                                if try_move(cell, cells, z, i, z as isize, i as isize + pixel.speed[1] as isize, random_num){
                                    continue;
                                }
                                cell.grid[i*CELL_SIZE + z].speed[1] = 1;
                                if try_move(cell, cells, z, i, z as isize + random_num as isize%8-3, i as isize, random_num){
                                    continue;
                                }
                                if try_move(cell,cells, z, i, z as isize + random_num as isize%8-3, i as isize + pixel.speed[1] as isize, random_num){
                                    continue;
                                }
                            }
                            cell.grid[i*CELL_SIZE + z].speed = [1,1];
            
                        },
                    4=>{
                        if random_num >= 120 && random_num <= 144{
                            if try_grow_destroy(cell, cells, z, i, z as isize + (random_num as isize%12-6 ).signum(), i as isize+(random_num as isize%24-12 ).signum(), random_num){
                                continue;
                            }
                        }
                        cell.grid[i*CELL_SIZE + z].speed = [1,1];
                    }
                    _ => continue,
                }
            }   
            else {
                continue;
            }
        }
    }
    cell.calculated_rect = DirtyRect{x1: old_rect.x1.min(cell.rect.x1),x2: old_rect.x2.max(cell.rect.x2),y1: old_rect.y1.min(cell.rect.y1),y2: old_rect.y2.max(cell.rect.y2)};
    cell.rect_resize(2, 2);
}
