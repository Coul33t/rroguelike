use rltk::{GameState, Rltk, RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod monster_ai_system;
use monster_ai_system::MonsterAI;





#[derive(PartialEq, Copy, Clone)]
pub enum RunState {Paused, Running}

pub struct State {
    ecs: World,
    pub runstate: RunState
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        }

        else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            if map.visible_tiles[map.xy_idx(pos.x, pos.y)] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        ctx.print(1, 1, "Hello RLTK world!");
    }
} 

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Name>();
    
    // Tags
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
}

fn create_monster(gs: &mut State, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB, bg: RGB, name: String) {
    
    gs.ecs.create_entity()
        .with(Position {x: x, y: y})
        .with(Renderable {
            glyph: glyph,
            fg: fg,
            bg: bg,
        })
        .with(Viewshed{visible_tiles : Vec::new(), range : 8, dirty: true})
        .with(Monster{})
        .with(Name{name: name})
    .build();
}

fn create_player(gs: &mut State, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB, bg: RGB) {

        gs.ecs.create_entity()
            .with(Position {x: x, y: y})
            .with(Renderable {
                glyph: glyph,
                fg: fg,
                bg: bg,
            })
            .with(Player{})
            .with(Viewshed{visible_tiles : Vec::new(), range : 8, dirty: true})
            .with(Name{name: "Player".to_string()})
        .build();

        gs.ecs.insert(Point::new(x, y));
}

fn populate_rooms(gs: &mut State, map: &Map) {
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let glyph: rltk::FontCharType;
        let name: String;

        let mut rng = rltk::RandomNumberGenerator::new();
        let roll = rng.roll_dice(1, 10);

        match roll {
            1..=8 => { glyph = rltk::to_cp437('x'); name = format!("Small x #{}", i).to_string(); }
            _     => { glyph = rltk::to_cp437('X'); name = format!("Big X #{}", i).to_string(); }
        }

        create_monster(gs, x, y, glyph, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), name);
    }
}

fn main() -> rltk::BError {
    println!("Hello, world!");

    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("RRoguelike").build()?;

    let mut gs = State{
        ecs: World::new(),
        runstate: RunState::Running
    };

    register_components(&mut gs);

    let map: Map = Map::new_map_rooms();
    let (x, y) = map.rooms[0].center();
    populate_rooms(&mut gs, &map);
    gs.ecs.insert(map);

    create_player(&mut gs, x, y, rltk::to_cp437('@'), RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK));

    rltk::main_loop(context, gs)
}
