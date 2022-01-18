use rltk::{ RGB, Rltk, RandomNumberGenerator, Point, Algorithm2D, BaseMap };
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
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>
}

impl Map { 
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }

    pub fn is_exit_valid(&self, x: i32, y:i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }

        let idx = self.xy_idx(x, y);

        self.tiles[idx as usize] != TileType::Wall
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
            height: 50,
            revealed_tiles: vec![false; 80*50],
            visible_tiles: vec![false; 80*50]
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
            height: 50,
            revealed_tiles: vec![false; 80*50],
            visible_tiles: vec![false; 80*50]
        };
    
        const MAX_ROOM: i32 = 20;
        const MIN_SIZE: i32 = 4;
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

    for (idx, tile) in map.tiles.iter().enumerate() {

        if map.revealed_tiles[idx] {

            let glyph;
            let mut fg;

            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                }

                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.0, 0.5, 0.0);
                }
            }

            if !map.visible_tiles[idx] {
                fg = RGB::from_f32(0.0, 0.1, 0.0);
            }

            ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);
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

    fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;
    
        // Cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + w, 1.0)) };
    
        exits 
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}