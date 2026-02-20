use std::sync::{Arc, Mutex};

use fastnoise_lite::{FastNoiseLite,};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use crate::{CELL_AMOUNT, CELL_SIZE, PITCH, PITCH_SIZE, PIXEL_DATA_SIZE};

const TEXTURE_SIZE:usize = 32;



#[derive(Clone, Copy)]
pub struct DirtyRect{
    pub x1:usize,
    pub y1:usize,
    pub x2:usize,
    pub y2:usize,
}

#[derive(Clone, Copy)]
pub struct Cell{
    pub id:usize,
    pub pixel_data: [u8; PIXEL_DATA_SIZE],
    pub grid: [Pixel; CELL_SIZE*CELL_SIZE],
    pub rect: DirtyRect,
    pub calculated_rect:DirtyRect,
}

#[derive(Clone, Copy)]
pub struct Pixel{
    pub id:u8,
    pub color: [u8;3],
    pub speed:[i8;2],
    pub iteration:u8, 
    pub temperature:i8,
}

impl Pixel{
    pub fn new(new_id:u8, color_new:[u8;3])-> Pixel{
        let new_pixel = Pixel{
            id: new_id,
            color: color_new,
            speed:[0,0],
            iteration:0,
            temperature:0
        };
        return new_pixel;
    }
}

impl Cell{
    pub fn rect_update(&mut self,mut x:isize,mut y:isize){
        x = x * !(x < 0) as isize;
        y = y * !(y < 0) as isize;
        let mut x = x as usize;
        let mut y = y as usize;
        x = x * !(x > CELL_SIZE) as usize + (CELL_SIZE)*(x > CELL_SIZE) as usize;
        y = y * !(y > CELL_SIZE) as usize + (CELL_SIZE)*(y > CELL_SIZE) as usize;
        
        self.rect.x1 = x*(self.rect.x1 + self.rect.x2 == 0)as usize + self.rect.x1*!(self.rect.x1 + self.rect.x2 == 0)as usize;
        self.rect.y1 = y*(self.rect.y1 + self.rect.y2 == 0)as usize + self.rect.y1*!(self.rect.y1 + self.rect.y2 == 0)as usize;

        self.rect.x1 = x*(x<self.rect.x1) as usize + self.rect.x1*!(x<self.rect.x1) as usize;
        self.rect.x2 = x*(x>self.rect.x2) as usize + self.rect.x2*!(x>self.rect.x2) as usize;
        self.rect.y1 = y*(y<self.rect.y1) as usize + self.rect.y1*!(y<self.rect.y1) as usize;
        self.rect.y2 = y*(y>self.rect.y2) as usize + self.rect.y2*!(y>self.rect.y2) as usize;

	self.calculated_rect.x1 = x*(self.calculated_rect.x1 + self.calculated_rect.x2 == 0)as usize + self.calculated_rect.x1*!(self.calculated_rect.x1 + self.calculated_rect.x2 == 0)as usize;
        self.calculated_rect.y1 = y*(self.calculated_rect.y1 + self.calculated_rect.y2 == 0)as usize + self.calculated_rect.y1*!(self.calculated_rect.y1 + self.calculated_rect.y2 == 0)as usize;

        self.calculated_rect.x1 = x*(x<self.calculated_rect.x1) as usize + self.calculated_rect.x1*!(x<self.calculated_rect.x1) as usize;
        self.calculated_rect.x2 = x*(x>self.calculated_rect.x2) as usize + self.calculated_rect.x2*!(x>self.calculated_rect.x2) as usize;
        self.calculated_rect.y1 = y*(y<self.calculated_rect.y1) as usize + self.calculated_rect.y1*!(y<self.calculated_rect.y1) as usize;
        self.calculated_rect.y2 = y*(y>self.calculated_rect.y2) as usize + self.calculated_rect.y2*!(y>self.calculated_rect.y2) as usize;
    }

    pub fn rect_resize(&mut self, x:isize,y:isize){
        let cal_x1 = self.calculated_rect.x1 as isize - x;
        let cal_y1 = self.calculated_rect.y1 as isize - y;
        let rect_x1 = self.rect.x1 as isize - x;
        let rect_y1 = self.rect.y1 as isize - y;

        self.calculated_rect.x2 += x as usize;
        self.calculated_rect.y2 += y as usize;

        self.rect.x2 += x as usize;
        self.rect.y2 += y as usize;

        self.calculated_rect.x1 = cal_x1.max(0) as usize;
        self.calculated_rect.x2 = self.calculated_rect.x2.min(CELL_SIZE);
        self.calculated_rect.y1 = cal_y1.max(0) as usize;
        self.calculated_rect.y2 = self.calculated_rect.y2.min(CELL_SIZE);
        self.rect.x1 = rect_x1.max(0) as usize;
        self.rect.x2 = self.rect.x2.min(CELL_SIZE);
        self.rect.y1 = rect_y1.max(0) as usize;
        self.rect.y2 = self.rect.y2.min(CELL_SIZE);
    }
    pub fn place_pixel(&mut self,x:usize,y:usize,id:u8,color:[u8;3]){
        self.rect_update(x as isize -1, y as isize -1);
        self.rect_update(x as isize + 1, y as isize + 1);
        self.grid[y*CELL_SIZE + x] = Pixel::new(id, color);

        self.pixel_data[y*PITCH + x*PITCH_SIZE + 0] = self.grid[y*CELL_SIZE + x].color[0];
        self.pixel_data[y*PITCH + x*PITCH_SIZE + 1] = self.grid[y*CELL_SIZE + x].color[1];
        self.pixel_data[y*PITCH + x*PITCH_SIZE + 2] = self.grid[y*CELL_SIZE + x].color[2];
    }

    pub fn new(new_id:usize)-> Cell{
        let pixel = Pixel::new(0, [70,70,70]);
        let new_cell = Cell{
            id: new_id,
            pixel_data: [70;PIXEL_DATA_SIZE],
            grid: [pixel;CELL_SIZE*CELL_SIZE],
            rect: DirtyRect { x1: 0, y1: 0, x2: CELL_SIZE, y2: CELL_SIZE},
            calculated_rect: DirtyRect { x1: 0, y1: 0, x2: CELL_SIZE, y2: CELL_SIZE},
        };
        return new_cell;
    }

    pub fn generate(&mut self, region_position:[i32;2], cell_position:[i32;2], texture_data: &Vec<u8>, biome: u32){
        let mut noise = FastNoiseLite::new();
        let mut noise2 = FastNoiseLite::new();
        noise2.frequency = 1.;
        noise.set_noise_type(Some(fastnoise_lite::NoiseType::Value));
        let mut world_position:[i32;2] = [0,0];
        noise.frequency = biome as f32/10.;

        world_position[0] = (cell_position[0]+region_position[0]*CELL_AMOUNT as i32)*CELL_SIZE as i32;
        world_position[1] = (cell_position[1]+region_position[1]*CELL_AMOUNT as i32)*CELL_SIZE as i32;
        
        if world_position[1] > (CELL_AMOUNT *CELL_SIZE) as i32{
            self.grid.par_iter_mut()
            .enumerate()
            .for_each(| (i,pixel)| {
                let x = (i % CELL_SIZE) as i32;
                let y = (i / CELL_SIZE) as i32;
                let texture_position = (x as usize % TEXTURE_SIZE + (y as usize % TEXTURE_SIZE) * TEXTURE_SIZE)*PITCH_SIZE;
                let id = ((noise.get_noise_2d( (x + world_position[0]) as f32,(y + world_position[1]) as f32)+1.)*2.) as u8 % 3 ;
                let mut color:[u8;3] = [70,70,70];

                let mut saturation =((noise2.get_noise_2d( (x + world_position[0]) as f32,(y + world_position[1]) as f32)+0.3)*240.) as u8/4;

                if id == 1{
                    color = [120 + saturation,200,200];
                }
                else if id == 2{
                    saturation = saturation/4;
                    color[2] = texture_data[texture_position+2] + saturation;
                    color[1] = texture_data[texture_position+1] + saturation;
                    color[0] = texture_data[texture_position] + saturation;
                }
                else if id == 3{
                    color = [200,0,0];
                }
                pixel.id = id;
                pixel.color = color;
            }); 
        }
        else {
            {
                self.grid = [Pixel::new(0, [0,0,0]);CELL_SIZE*CELL_SIZE];
            }
        }

        
    }
}


#[derive(Clone)]
pub struct Region{
    pub id: usize,
    pub generated: bool,
    pub biome_changed:bool,
    pub biome:u32,
    pub cell_to_generate: usize,
    pub position: [i32;2],
    pub cells: Vec<Arc<Mutex<Cell>>>,
}

impl Region{
    pub fn generate(&mut self, texture_data: &Vec<u8>){
        if self.cells.len() == 0{
            self.cells = vec![];
            for i in 0..CELL_AMOUNT*CELL_AMOUNT{
                let cell_position = [(i % CELL_AMOUNT) as i32, (i / CELL_AMOUNT) as i32];
                let mut cell = Cell::new(i);
                cell.generate(self.position, cell_position,&texture_data,self.biome);
                self.cells.insert(i,Arc::new(Mutex::new(cell)));
            }
        }
        else{
            for i in 0..CELL_AMOUNT*CELL_AMOUNT{
                let cell_position = [(i % CELL_AMOUNT) as i32, (i / CELL_AMOUNT) as i32];
                let mut cell = self.cells[i].lock().unwrap();
                cell.generate(self.position, cell_position,&texture_data, self.biome);
            }
        }
    }

    pub fn generate_cell(&mut self, texture_data: &Vec<u8>){
        let cell_position = [(self.cell_to_generate % CELL_AMOUNT) as i32, (self.cell_to_generate / CELL_AMOUNT) as i32];
        let mut cell = self.cells[self.cell_to_generate].lock().unwrap();
        cell.generate(self.position, cell_position, &texture_data,self.biome);
        self.cell_to_generate += 1;
        if self.cell_to_generate >= CELL_AMOUNT*CELL_AMOUNT{
            self.generated = true;
            self.cell_to_generate = 0;
        }
    }

    pub fn set_biome(){

    }
}
