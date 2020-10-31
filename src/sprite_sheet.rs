extern crate piston;
extern crate piston_window;
extern crate quick_xml;
extern crate rand;
extern crate serde;

mod utils;

use piston::WindowSettings;
use piston_window::*;
use utils::game_window::GameWindow;
use utils::scene::Scene;
use utils::sprite_sheet::SpriteSheet;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 384;
const CORNFLOWER_BLUE: [f32; 4] = [0.392, 0.584, 0.929, 1.0];

#[derive(Clone)]
pub struct Tile {
    sprite_name: String,
    x: u32,
    y: u32,
}

impl Tile {
    pub fn new(sprite_name: &str, x: u32, y: u32) -> Tile {
        Tile {
            sprite_name: sprite_name.to_string(),
            x,
            y,
        }
    }

    pub fn coords(&self, height: f64, tile_size: f64) -> [f64; 2] {
        [
            tile_size * self.x as f64,
            height - (1.0 + self.y as f64) * tile_size,
        ]
    }
}

#[derive(Clone)]
pub struct SpriteSheetApp {
    width: u32,
    height: u32,
    sprite_sheet: SpriteSheet,
    tiles: Vec<Tile>,
}

impl SpriteSheetApp {
    pub fn new(width: u32, height: u32, sprite_sheet: SpriteSheet) -> SpriteSheetApp {
        SpriteSheetApp {
            width,
            height,
            sprite_sheet,
            tiles: vec![
                Tile::new("tileYellow_04.png", 0, 0),
                Tile::new("tileYellow_04.png", 0, 1),
                Tile::new("tileYellow_04.png", 0, 2),
                Tile::new("tileYellow_06.png", 0, 3),
                Tile::new("tileYellow_04.png", 1, 0),
                Tile::new("tileYellow_09.png", 1, 1),
                Tile::new("tileYellow_20.png", 1, 2),
                Tile::new("tileYellow_11.png", 1, 3),
                Tile::new("tileYellow_04.png", 2, 0),
                Tile::new("tileYellow_20.png", 2, 1),
                Tile::new("tileYellow_11.png", 2, 2),
                Tile::new("tileYellow_04.png", 3, 0),
                Tile::new("tileYellow_06.png", 3, 1),
                Tile::new("tileYellow_18.png", 4, 0),
                Tile::new("tileYellow_06.png", 4, 1),
                Tile::new("tileYellow_04.png", 5, 0),
                Tile::new("tileYellow_07.png", 5, 1),
                Tile::new("tileYellow_06.png", 6, 0),
                Tile::new("tileYellow_06.png", 7, 0),
                Tile::new("tileYellow_04.png", 8, 0),
                Tile::new("tileYellow_04.png", 8, 1),
                Tile::new("tileYellow_05.png", 8, 2),
                Tile::new("tileYellow_04.png", 9, 0),
                Tile::new("tileYellow_09.png", 9, 1),
                Tile::new("tileYellow_06.png", 9, 2),
            ],
        }
    }
}

impl Scene for SpriteSheetApp {
    fn render(self, c: Context, g: &mut G2d) {
        let h = self.height as f64;
        clear(CORNFLOWER_BLUE, g);
        let tiles = &self.tiles;
        tiles.iter().for_each(|t| {
            self.sprite_sheet
                .render_sprite(&t.sprite_name, t.coords(h, 64.0), c, g);
        });
    }

    fn update(&mut self) {}

    fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("sprite-sheet", [WIDTH, HEIGHT])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // assets from Kenney, find out more at https://kenney.nl/
    let sheet = SpriteSheet::new(
        "assets",
        "sprites.xml",
        &mut window.create_texture_context(),
    );

    let app = SpriteSheetApp::new(WIDTH, HEIGHT, sheet);
    let mut game_window = GameWindow::new(window, app);

    game_window.game_loop();
}
