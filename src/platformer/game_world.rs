use piston_window::{clear, Context, G2d};
use platformer::constants::BACKGROUND;
use platformer::map::{Map, Tile};
use platformer::player::{Player, PlayerUpdateArgs};
use utils::entity::*;
use utils::input::GameInput;
use utils::sprite_sheet::SpriteSheet;

#[derive(Clone)]
pub struct GameWorld {
    pub width: u32,
    pub height: u32,
    sprite_sheet: SpriteSheet,
    player: Player,
    pub input: GameInput,
    map: Map,
}

impl GameWorld {
    pub fn new(width: u32, height: u32, sprite_sheet: SpriteSheet) -> GameWorld {
        let player = Player::new(
            sprite_sheet
                .sprite_size(&"playerRed_stand.png".to_string())
                .unwrap(),
        );

        GameWorld {
            width,
            height,
            sprite_sheet,
            player,
            input: GameInput::new(),
            map: Map::new(64.0),
        }
    }

    fn render_sprite(&self, t: &Tile, c: Context, g: &mut G2d) {
        let sprite_size = self.sprite_sheet.sprite_size(&t.sprite_name).unwrap();
        self.sprite_sheet.render_sprite(
            &t.sprite_name,
            t.coords(self.height as f64, 64.0, sprite_size),
            c,
            g,
        );
    }
}

impl Renderable for GameWorld {
    fn render(self, c: Context, g: &mut G2d) {
        clear(BACKGROUND, g);

        self.map
            .tiles
            .iter()
            .for_each(|t| self.render_sprite(t, c, g));

        self.map
            .decorations
            .iter()
            .for_each(|t| self.render_sprite(t, c, g));
        self.render_sprite(&self.map.flag, c, g);

        let player_args = self.player.render_args();
        let player_size = self.sprite_sheet.sprite_size(&player_args.sprite).unwrap();
        self.sprite_sheet.render_sprite(
            &player_args.sprite,
            [
                player_args.pos[0] - player_size[0] / 2.0,
                self.height as f64 - player_args.pos[1] - player_args.size[1],
            ],
            c,
            g,
        );
    }
}

impl Updatable for GameWorld {
    type Args = GameWorldUpdateArgs;

    fn update(&mut self, args: Self::Args) {
        self.player.update(PlayerUpdateArgs {
            dt: args.dt,
            input: self.input.clone(),
            screen_size: [self.width as f64, self.height as f64],
            map: self.map.clone(),
        });

        let (player_x, player_y) = self.map.player_pos(self.player.pos);
        if player_x == self.map.flag.x && player_y == self.map.flag.y {
            self.map.flag_reached();
        }
    }
}

pub struct GameWorldUpdateArgs {
    pub dt: f64,
}
