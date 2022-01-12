use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use super::{Position, Player, TileType, State, Map, Viewshed, RunState};
use std::cmp::{min, max};

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);
        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(79, max(0, pos.y + dy));

            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => {return RunState::Paused}
        Some(key) => match key {
            VirtualKeyCode::Q | VirtualKeyCode::Left     => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D | VirtualKeyCode::Right    => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Z | VirtualKeyCode::Up       => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S | VirtualKeyCode::Down     => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::A                            => try_move_player(0, 0, &mut gs.ecs),
            _ => {return RunState::Paused}
        },
    }

    RunState::Running
}