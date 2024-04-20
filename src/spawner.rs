pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point, player_number: i32) {
    ecs.push(
        (
            Player(player_number),
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}
