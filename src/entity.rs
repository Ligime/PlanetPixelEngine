use std::sync::{Arc, Mutex};


use crate::{pixels::Cell, ACTIVE_REGS, CELL_AMOUNT, CELL_SIZE};


pub struct Entity{
    pub size: [u8;2],
    pub position:[f32;2], 
    pub speed:[f32;2],
    pub is_on_floor:bool,
    pub flip_h:bool,
    pub animation:i8
}




pub fn check_collision(entity:&mut Entity, big_cells: &mut Vec<Arc<Mutex<Cell>>>, world_position:[i32;2]){
    let mut check_is_on_floor = false;
    let check_amount_y = 7;
    for j in 0..check_amount_y{
        for i in 2..(entity.size[0]-2){
            let x = (entity.position[0] - world_position[0] as f32*(CELL_SIZE*CELL_AMOUNT) as f32) as i32 + i as i32; //local position
            let y = (entity.position[1] - world_position[1] as f32*(CELL_SIZE*CELL_AMOUNT) as f32) as i32 + j;


            let mut in_cell_x =  x % CELL_SIZE as i32;
            let mut in_cell_y =  (y + entity.size[1] as i32 - check_amount_y) % CELL_SIZE as i32;
            let mut in_cell2_y =  (y) % CELL_SIZE as i32;

            if in_cell_x < 0{
                in_cell_x = CELL_SIZE as i32 + in_cell_x;
            }
            if in_cell_y < 0{
                in_cell_y = CELL_SIZE as i32 + in_cell_y;
            }
            if in_cell2_y < 0{
                in_cell2_y = CELL_SIZE as i32 + in_cell2_y;
            }

            let cell_x = x / CELL_SIZE as i32+3;
            let cell_y = (y + entity.size[1] as i32  - check_amount_y) / CELL_SIZE as i32+4;
            let cell2_y = (y) / CELL_SIZE as i32+4;

            let mut cell = big_cells[ (cell_x  + cell_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize].lock().unwrap();
            if cell.grid[in_cell_x as usize +  (in_cell_y as usize)*CELL_SIZE].id == 2 || cell.grid[in_cell_x as usize +  (in_cell_y as usize)*CELL_SIZE].id == 1{
                entity.position[1] -= 0.5;
                check_is_on_floor = true;
                //cell.grid[in_cell_x as usize +  (in_cell_y as usize)*CELL_SIZE].color = [255,0,0,255];
                if !entity.is_on_floor{
                    entity.speed[1] = 0.;
                }
            }
            if ((cell_x  + cell_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize) !=  (cell_x  + cell2_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize{
                cell = big_cells[ (cell_x  + cell2_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize].lock().unwrap();
            }
            if cell.grid[in_cell_x as usize +  (in_cell2_y as usize)*CELL_SIZE].id == 2 {
                entity.position[1] += 0.5;
                entity.speed[1] = 0.;
            }
        }
        entity.is_on_floor = check_is_on_floor;
        for i in 1..(entity.size[1]-5){
            let x = entity.position[0] as i32 - world_position[0]*(CELL_SIZE*CELL_AMOUNT) as i32 + j%4; //local position
            let y = entity.position[1] as i32 - world_position[1]*(CELL_SIZE*CELL_AMOUNT) as i32 + i as i32;


            let mut in_cell_x =  (x) % CELL_SIZE as i32;
            let mut in_cell_y =  (y-1) % CELL_SIZE as i32;
            let mut in_cell2_x =  (x + entity.size[0] as i32 - 4) % CELL_SIZE as i32;

            if in_cell_x < 0{
                in_cell_x = CELL_SIZE as i32 + in_cell_x;
            }
            if in_cell_y < 0{
                in_cell_y = CELL_SIZE as i32 + in_cell_y;
            }
            if in_cell2_x < 0{
                in_cell2_x = CELL_SIZE as i32 + in_cell2_x;
            }

            let cell_x = (x) / CELL_SIZE as i32+3;
            let cell_y = (y-1) / CELL_SIZE as i32+4;
            let cell2_x = (x + entity.size[0] as i32 - 4) / CELL_SIZE as i32+3;

            let mut cell = big_cells[ (cell_x  + cell_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize].lock().unwrap();
            if cell.grid[in_cell_x as usize +  (in_cell_y as usize)*CELL_SIZE].id == 2{
                entity.position[0] += 0.5;
                entity.speed[0] = 0.;
            }

            if ((cell_x  + cell_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize) !=  (cell2_x  + cell_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize{
                cell = big_cells[ (cell2_x  + cell_y*(CELL_AMOUNT*ACTIVE_REGS) as i32) as usize].lock().unwrap();
            }
            if cell.grid[in_cell2_x as usize +  (in_cell_y as usize)*CELL_SIZE].id == 2 {
                entity.position[0] -= 0.5;
                entity.speed[0] = 0.;
            }
        }
    }
    

    

}



pub fn update_player(player: &mut Entity){
    let air_accel = 0.5;
    let max_speed_x = 5.;
    let max_speed_y = 7.;

    player.speed[0] = (player.speed[0]).abs().min(max_speed_x)*player.speed[0].signum();
    player.speed[1] = (player.speed[1]).abs().min(max_speed_y)*player.speed[1].signum();
    player.position[0] += player.speed[0];
    player.position[1] += player.speed[1];
    if !player.is_on_floor{
        player.speed[1] += air_accel
    }
}
