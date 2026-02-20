use std::sync::{Arc, Mutex};

use crate::{ACTIVE_REGS, CELL_AMOUNT, CENT_ACT_REG32, REGION_AMOUNT, pixels::Cell, structs::Region};

pub fn make_big_cells(regions: &mut Vec<Region>, big_cells: &mut Vec<Arc<Mutex<Cell>>>){
    big_cells.clear();
    assert!(big_cells.is_empty());
    for i in 0..ACTIVE_REGS{
        for y in 0..CELL_AMOUNT{
            for j in 0..ACTIVE_REGS{
                for x in 0..CELL_AMOUNT{
                    big_cells.push(regions[REGION_AMOUNT + REGION_AMOUNT/2 + i * REGION_AMOUNT + j -1].cells[x + y*CELL_AMOUNT].clone());
                }
            }
        }
    }
}

pub fn update_regions_x(regions: &mut Vec<Region>,shift_delta: i32){
    if shift_delta > 0{
        regions[0].position[1] += REGION_AMOUNT as i32;
        regions.rotate_left(1); //right
        for i in 0..REGION_AMOUNT{
            regions[REGION_AMOUNT*i+REGION_AMOUNT-1].position[0] += REGION_AMOUNT as i32;
            regions[REGION_AMOUNT*i+REGION_AMOUNT-1].position[1] -= 1;
            regions[REGION_AMOUNT*i+REGION_AMOUNT-1].generated = false;
            regions[REGION_AMOUNT*i+REGION_AMOUNT-1].biome_changed = false;
            regions[REGION_AMOUNT*i+REGION_AMOUNT-1].cell_to_generate = 0;
        }
    }
    else{
        regions[REGION_AMOUNT*REGION_AMOUNT -1].position[1] -= REGION_AMOUNT as i32;
        regions.rotate_right(1); //left
        for i in 0..REGION_AMOUNT{
            regions[REGION_AMOUNT*i].position[0] -= REGION_AMOUNT as i32;
            regions[REGION_AMOUNT*i].position[1] += 1;
            regions[REGION_AMOUNT*i].generated = false;
            regions[REGION_AMOUNT*i].biome_changed = false;
            regions[REGION_AMOUNT*i].cell_to_generate = 0;
        }
    }
}

pub fn update_regions_y(regions: &mut Vec<Region>,shift_delta: i32){
    if shift_delta < 0{ //Up
        regions.rotate_right(REGION_AMOUNT); 
        for i in 0..REGION_AMOUNT{
            regions[i].position[1] -= REGION_AMOUNT as i32;
            regions[i].generated = false;
            regions[i].biome_changed = false;
            regions[i].cell_to_generate = 0;
        }
    }
    else{ //Down
        regions.rotate_left(REGION_AMOUNT); 
        for i in 0..REGION_AMOUNT{
            regions[i+REGION_AMOUNT*(REGION_AMOUNT-1)].position[1] += REGION_AMOUNT as i32;
            regions[i+REGION_AMOUNT*(REGION_AMOUNT-1)].generated = false;
            regions[i+REGION_AMOUNT*(REGION_AMOUNT-1)].biome_changed = false;
            regions[i+REGION_AMOUNT*(REGION_AMOUNT-1)].cell_to_generate = 0;
        }
    }
}

pub fn update_cells_x(cells: &mut Vec<Arc<Mutex<Cell>>>, big_cells: &mut Vec<Arc<Mutex<Cell>>>, shift_delta: [i32;2], shift_position: [i32;2]){
    if shift_delta[0] > 0{ //right
        cells.rotate_left(1);
        for i in 0..CELL_AMOUNT{ 
            cells[(i+1)*CELL_AMOUNT-1] = big_cells[(CENT_ACT_REG32 + CELL_AMOUNT as i32 - 1  + (shift_position[1] + i as i32) * (CELL_AMOUNT * ACTIVE_REGS) as i32 + shift_position[0]) as usize].clone(); 
        }
    }
    else{ //left
        cells.rotate_right(1);
        for i in 0..CELL_AMOUNT{
            cells[i*CELL_AMOUNT] = big_cells[(CENT_ACT_REG32  + (shift_position[1] + i as i32) * (CELL_AMOUNT * ACTIVE_REGS) as i32 + shift_position[0]) as usize].clone();
        }
    }
}

pub fn update_cells_y(cells: &mut Vec<Arc<Mutex<Cell>>>, big_cells: &mut Vec<Arc<Mutex<Cell>>>,shift_delta: [i32;2], shift_position: [i32;2]){
    if shift_delta[1] < 0{ //UP
        cells.rotate_right(CELL_AMOUNT); 
        for i in 0..CELL_AMOUNT{ 
            cells[i] = big_cells[(CENT_ACT_REG32 + i as i32  + shift_position[1] * (CELL_AMOUNT * ACTIVE_REGS) as i32 + shift_position[0]) as usize].clone();
        }
    }
    else{ //Down
        cells.rotate_left(CELL_AMOUNT); 
        for i in 0..CELL_AMOUNT{ 
            cells[i + CELL_AMOUNT*(CELL_AMOUNT-1)] = big_cells[(CENT_ACT_REG32 + i  as i32 + (shift_position[1] + CELL_AMOUNT as i32 -1) * (CELL_AMOUNT * ACTIVE_REGS) as i32 + shift_position[0]) as usize].clone();
        }
    }
}

