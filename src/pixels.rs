use std::{mem, sync::{Arc, Mutex}};
pub use crate::{structs::Cell, structs::DirtyRect};
use crate::{ CELL_AMOUNT, CELL_SIZE, structs::Pixel};


// 0-AIR, 1-SAND, 2-ROCK, 3-WATER, 4-PLANT

pub const TRANSPARENCY: [u8;5] = [0,255,255,80,255];
const DENSITIES: [isize; 5] = [0,2,3,1,3];
const FRICTIONS: [isize; 5] = [0,5,2,5,0];
const ACCEL: [isize;5] = [0,5,0,2,0];



pub fn try_move(cell:&mut Cell, cells: &mut Vec<Arc<Mutex<Cell>>>, x1:usize, y1:usize, mut x2: isize, mut y2: isize, random:u8)->bool{
    let mut friction = true;
    let mut air_accel = true;
    let mut cell2id = cell.id as isize;
    let random = random/25;
    if ACCEL[cell.grid[y1*CELL_SIZE + x1].id as usize] > random as isize %8{
        air_accel = false
    }
    if FRICTIONS[cell.grid[y1*CELL_SIZE + x1].id as usize] > random as isize %8{
        friction = false;
    }
    
    cell.grid[y1*CELL_SIZE + x1].speed[1] = (cell.grid[y1*CELL_SIZE + x1].speed[1] +  air_accel as i8) % (CELL_SIZE/2) as i8;
    cell.grid[y1*CELL_SIZE + x1].speed[0] = (cell.grid[y1*CELL_SIZE + x1].speed[0] +  friction as i8) % (CELL_SIZE/2) as i8;
    
    if x2 < 0{
        if cell.id % CELL_AMOUNT == 0 {
            return false;
        }
        x2 = CELL_SIZE as isize + x2 ;
        cell2id -= 1;
    }
    else if x2 >= CELL_SIZE as isize{
        if cell.id % CELL_AMOUNT == CELL_AMOUNT-1{
            return false;
        }
        x2 -= CELL_SIZE as isize ;
        cell2id += 1;
    }


    if y2 < 0 {
        if cell.id < CELL_AMOUNT{
            return false;
        }
        y2 = CELL_SIZE as isize + y2;
        cell2id -= CELL_AMOUNT as isize;
    }
    else if y2 >= CELL_SIZE as isize{
        if cell.id == CELL_AMOUNT*CELL_AMOUNT-1{
            return false;
        }
        y2 -= CELL_SIZE as isize;
        cell2id += CELL_AMOUNT as isize;
    }
    let x2 = x2 as usize;
    let y2 = y2 as usize;
    let cell2id = cell2id as usize;

    //swap inside
    if cell2id == cell.id {
        let density1 = DENSITIES[cell.grid[y1*CELL_SIZE + x1].id as usize];
        let density2 = DENSITIES[cell.grid[y2*CELL_SIZE + x2].id as usize];
        if density2 < density1{
            cell.grid[y2*CELL_SIZE + x2].speed = [1,1];

            let temp = cell.grid[y1*CELL_SIZE + x1];
            cell.grid[y1*CELL_SIZE + x1] = cell.grid[y2*CELL_SIZE + x2];
            cell.grid[y2*CELL_SIZE + x2] = temp;
            cell.rect_update(x2 as isize, y2 as isize);
            
            return true;
        }
    }

    //swap between
    else if cell2id < CELL_AMOUNT*CELL_AMOUNT{
        let cell_arc = &cells[cell2id];
        let cell2 = &mut cell_arc.lock().unwrap();
        let density1 = DENSITIES[cell.grid[y1*CELL_SIZE + x1].id as usize];
        let density2 = DENSITIES[cell2.grid[y2*CELL_SIZE + x2].id as usize];
        if density2 < density1{
            cell2.grid[y2*CELL_SIZE + x2].speed = [1,1];
            mem::swap(&mut cell.grid[y1*CELL_SIZE + x1], &mut cell2.grid[y2*CELL_SIZE + x2]);
            cell2.rect_update(x2 as isize, y2 as isize);
            return true;
        }
    }
    return false;
}




pub fn try_grow(cell:&mut Cell, cells: &mut Vec<Arc<Mutex<Cell>>>, x1:usize, y1:usize, mut x2: isize, mut y2: isize, random:u8)->bool{
    let mut cell2id = cell.id as isize;
 
    if x2 < 0{
        if cell.id % CELL_AMOUNT == 0 {
            return false;
        }
        x2 = CELL_SIZE as isize + x2 ;
        cell2id -= 1;
    }
    else if x2 >= CELL_SIZE as isize{
        if cell.id % CELL_AMOUNT == CELL_AMOUNT-1{
            return false;
        }
        x2 -= CELL_SIZE as isize ;
        cell2id += 1;
    }


    if y2 < 0 {
        if cell.id < CELL_AMOUNT{
            return false;
        }
        y2 = CELL_SIZE as isize + y2;
        cell2id -= CELL_AMOUNT as isize;
    }
    else if y2 >= CELL_SIZE as isize{
        if cell.id == CELL_AMOUNT*CELL_AMOUNT-1{
            return false;
        }
        y2 -= CELL_SIZE as isize;
        cell2id += CELL_AMOUNT as isize;
    }
    let x2 = x2 as usize;
    let y2 = y2 as usize;
    let cell2id = cell2id as usize;

    //swap inside
    if cell2id == cell.id {
        if cell.grid[y2*CELL_SIZE + x2].id == 0{


            cell.grid[y2*CELL_SIZE + x2] = cell.grid[y1*CELL_SIZE + x1];
            cell.grid[y2*CELL_SIZE + x2].color[0] = (cell.grid[y2*CELL_SIZE + x2].color[0] as i32 + (random as i32%32 -16)) as u8 %64;
            cell.grid[y2*CELL_SIZE + x2].color[1] = (cell.grid[y2*CELL_SIZE + x2].color[1] as i32 + (random as i32%32 -16)) as u8 % 150 + 64;
            cell.grid[y2*CELL_SIZE + x2].color[2] = (cell.grid[y2*CELL_SIZE + x2].color[2] as i32 + (random as i32%32 -16)) as u8 %64;
            cell.rect_update(x2 as isize, y2 as isize);
            
            return true;
        }
    }

    //swap between
    else if cell2id < CELL_AMOUNT*CELL_AMOUNT{
        let cell_arc = &cells[cell2id];
        let cell2 = &mut cell_arc.lock().unwrap();
        if cell2.grid[y2*CELL_SIZE + x2].id == 0{
            cell2.grid[y2*CELL_SIZE + x2] = cell.grid[y1*CELL_SIZE + x1];
            cell2.rect_update(x2 as isize, y2 as isize);
            return true;
        }
    }
    return false;
}


pub fn try_grow_destroy(cell:&mut Cell, cells: &mut Vec<Arc<Mutex<Cell>>>, x1:usize, y1:usize, mut x2: isize, mut y2: isize, random:u8)->bool{
    let mut cell2id = cell.id as isize;
 
    if x2 < 0{
        if cell.id % CELL_AMOUNT == 0 {
            return false;
        }
        x2 = CELL_SIZE as isize + x2 ;
        cell2id -= 1;
    }
    else if x2 >= CELL_SIZE as isize{
        if cell.id % CELL_AMOUNT == CELL_AMOUNT-1{
            return false;
        }
        x2 -= CELL_SIZE as isize ;
        cell2id += 1;
    }


    if y2 < 0 {
        if cell.id < CELL_AMOUNT{
            return false;
        }
        y2 = CELL_SIZE as isize + y2;
        cell2id -= CELL_AMOUNT as isize;
    }
    else if y2 >= CELL_SIZE as isize{
        if cell.id == CELL_AMOUNT*CELL_AMOUNT-1{
            return false;
        }
        y2 -= CELL_SIZE as isize;
        cell2id += CELL_AMOUNT as isize;
    }
    let x2 = x2 as usize;
    let y2 = y2 as usize;
    let cell2id = cell2id as usize;

    //swap inside
    if cell2id == cell.id {
        if cell.grid[y2*CELL_SIZE + x2].id == 0{


            cell.grid[y2*CELL_SIZE + x2] = cell.grid[y1*CELL_SIZE + x1];
            cell.grid[y2*CELL_SIZE + x2].color[0] = (cell.grid[y2*CELL_SIZE + x2].color[0] as i32 + (random as i32%33 -16)) as u8 ;
            cell.grid[y2*CELL_SIZE + x2].color[1] = (cell.grid[y2*CELL_SIZE + x2].color[1] as i32 + (random as i32%32 -16)) as u8  + 64;
            cell.grid[y2*CELL_SIZE + x2].color[2] = (cell.grid[y2*CELL_SIZE + x2].color[2] as i32 + (random as i32%31 -16)) as u8 ;
            cell.rect_update(x2 as isize, y2 as isize);
            cell.grid[y1*CELL_SIZE + x1] = Pixel::new(0, [0,0,0]);
            return true;
        }
    }

    //swap between
    else if cell2id < CELL_AMOUNT*CELL_AMOUNT{
        let cell_arc = &cells[cell2id];
        let cell2 = &mut cell_arc.lock().unwrap();
        if cell2.grid[y2*CELL_SIZE + x2].id == 0{
            cell2.grid[y2*CELL_SIZE + x2] = cell.grid[y1*CELL_SIZE + x1];
            cell2.rect_update(x2 as isize, y2 as isize);
            cell.grid[y1*CELL_SIZE + x1] = Pixel::new(0, [0,0,0]);
            return true;
        }
    }
    return false;
}
