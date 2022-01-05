use rltk::{ RGB, Rltk, RandomNumberGenerator };
use super::{Rect};
use std::cmp::{max, min};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map { 
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }
    
    fn carve_room(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        } 
    }
    
    fn carve_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 50 * 80 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn carve_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 50 * 80 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    pub fn new_map_test() -> Map {
        let mut map = Map{
            tiles: vec![TileType::Floor; 80*50],
            rooms: Vec::new(),
            width: 80,
            height: 50
        };
    
        for x in 0..80 {
            let idx1 = map.xy_idx(x, 0);
            let idx2 = map.xy_idx(x, 49);
            map.tiles[idx1] = TileType::Wall;
            map.tiles[idx2] = TileType::Wall;
        }
    
        for y in 0..50 {
            let idx1 = map.xy_idx(0, y);
            let idx2 = map.xy_idx(79, y);
            map.tiles[idx1] = TileType::Wall;
            map.tiles[idx2] = TileType::Wall;
        }
    
        let mut rng = rltk::RandomNumberGenerator::new();
    
        for _i in 0..200 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = map.xy_idx(x, y);
            if idx != map.xy_idx(40, 25) {
                map.tiles[idx] = TileType::Wall;
            }
        }
    
        map
    }
    
    pub fn new_map_rooms() -> Map {
        let mut map = Map{
            tiles: vec![TileType::Wall; 80*50],
            rooms: Vec::new(),
            width: 80,
            height: 50
        };
    
        const MAX_ROOM: i32 = 20;
        const MIN_SIZE: i32 = 3;
        const MAX_SIZE: i32 = 10;
    
        let mut rng = RandomNumberGenerator::new();
    
        for _ in 0..MAX_ROOM {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
    
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
    
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
    
            if ok {
                map.carve_room(&new_room);
    
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (pre_x, pre_y) = map.rooms[map.rooms.len() - 1].center();
    
                    if rng.range(0, 2) == 0 {
                        map.carve_h_tunnel(pre_x, new_x, pre_y);
                        map.carve_v_tunnel(pre_y, new_y, new_x);
                    }
    
                    else {
                        map.carve_v_tunnel( pre_y, new_y, pre_x);
                        map.carve_h_tunnel(pre_x, new_x, new_y);
                    }
                }
    
                map.rooms.push(new_room);
            }
        }
    
        map
    }
}

pub fn draw_map(ecs: &World , ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('.'));
            }

            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 0.5, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}