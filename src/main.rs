use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};


use rand::Rng;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;

pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        ctx.print(1, 1, "Hello RLTK world!");
    }
} 

impl State {
    fn run_systems(&mut self) {

    }
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
}

fn create_entity(gs: &mut State, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB, bg: RGB, is_player: bool) {
    if is_player {
        gs.ecs.create_entity()
            .with(Position {x: x, y: y})
            .with(Renderable {
                glyph: glyph,
                fg: fg,
                bg: bg,
            })
            .with(Player{})
        .build();
    }

    else {
        gs.ecs.create_entity()
            .with(Position {x: x, y: y})
            .with(Renderable {
                glyph: glyph,
                fg: fg,
                bg: bg,
            })
        .build();
    }
}

fn main() -> rltk::BError {
    println!("Hello, world!");

    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("RRoguelike").build()?;

    let mut gs = State{
        ecs: World::new()
    };

    register_components(&mut gs);

    create_entity(&mut gs, 5, 5, rltk::to_cp437('@'), RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), true);

    for i in 0..10 {
        for j in 0..10 {
            if i == 5 && j == 5 {
                create_entity(&mut gs, 10 + i*2, 10 + j*2, rltk::to_cp437('X'), RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), false);
            }

            else {
                create_entity(&mut gs, 10 + i*2, 10 + j*2, rltk::to_cp437('x'), RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), false);
            }
        }
    }

    gs.ecs.insert(new_map_test());

    rltk::main_loop(context, gs)
}
