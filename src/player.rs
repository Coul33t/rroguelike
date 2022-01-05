use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, TileType, State, Map};
use std::cmp::{min, max};

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);
        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(79, max(0, pos.y + dy));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Q       => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D       => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Z       => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S       => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::A       => try_move_player(0, 0, &mut gs.ecs),
            _ => {}
        },
    }
}