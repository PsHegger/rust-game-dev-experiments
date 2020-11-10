use serde::export::Option::Some;

#[derive(Debug, Clone)]
pub struct Map {
    tile_size: f64,
    pub tiles: Vec<Tile>,
    pub decorations: Vec<Tile>,
    pub flag: Tile,
    is_flag_reached: bool,
}

impl Map {
    pub fn new(tile_size: f64) -> Map {
        Map {
            tile_size,
            tiles: vec![
                Tile::new_tile("tileYellow_15.png", 0, 0),
                Tile::new_tile("tileYellow_09.png", 0, 1),
                Tile::new_tile("tileYellow_07.png", 0, 2),
                Tile::new_tile("tileYellow_06.png", 1, 0),
                Tile::new_tile("tileYellow_06.png", 2, 0),
                Tile::new_tile("tileYellow_14.png", 3, 0),
                Tile::new_tile("tileYellow_05.png", 3, 1),
                Tile::new_tile("tileYellow_15.png", 4, 0),
                Tile::new_tile("tileYellow_07.png", 4, 1),
                Tile::new_tile("tileYellow_16.png", 4, 4),
                Tile::new_tile("tileYellow_06.png", 5, 0),
                Tile::new_tile("tileYellow_06.png", 5, 4),
                Tile::new_tile("tileYellow_06.png", 6, 0),
                Tile::new_tile("tileYellow_17.png", 6, 4),
                Tile::new_tile("tileYellow_14.png", 7, 0),
                Tile::new_tile("tileYellow_05.png", 7, 1),
                Tile::new_tile("tileYellow_04.png", 8, 0),
                Tile::new_tile("tileYellow_14.png", 8, 1),
                Tile::new_tile("tileYellow_05.png", 8, 2),
                Tile::new_tile("tileYellow_04.png", 9, 0),
                Tile::new_tile("tileYellow_18.png", 9, 1),
                Tile::new_tile("tileYellow_06.png", 9, 2),
            ],
            decorations: vec![
                Tile::new_decoration("plantGreen_3.png", 3, 2, true, 0.0, 0.0),
                Tile::new_decoration("signArrow_right.png", 0, 3, true, 0.0, 0.0),
            ],
            flag: Tile::new_decoration("flagGreen_down.png", 4, 5, false, 0.3, -0.05),
            is_flag_reached: false,
        }
    }

    pub fn floor_under_position(&self, pos: [f64; 2]) -> f64 {
        let (x, y) = self.player_pos(pos);
        let highest_tile = self
            .tiles
            .iter()
            .filter(|t| t.x == x && t.y <= y)
            .max_by(|t, t2| t.y.cmp(&t2.y));
        if let Some(tile) = highest_tile {
            (tile.y + 1) as f64 * self.tile_size
        } else {
            0.0
        }
    }

    pub fn ceiling_over_position(&self, pos: [f64; 2], height: f64) -> f64 {
        let (x, y) = self.player_pos(pos);
        let lowest_tile = self
            .tiles
            .iter()
            .filter(|t| t.x == x && t.y >= y)
            .min_by(|t, t2| t.y.cmp(&t2.y));
        if let Some(tile) = lowest_tile {
            tile.y as f64 * self.tile_size
        } else {
            height
        }
    }

    pub fn wall_at_left(&self, pos: [f64; 2]) -> f64 {
        let (x, y) = self.player_pos(pos);
        let wall = self
            .tiles
            .iter()
            .filter(|t| t.y == y && t.x <= x)
            .max_by(|t, t2| t.x.cmp(&t2.x));
        if let Some(tile) = wall {
            (tile.x + 1) as f64 * self.tile_size
        } else {
            0.0
        }
    }

    pub fn wall_at_right(&self, pos: [f64; 2], width: f64) -> f64 {
        let (x, y) = self.player_pos(pos);
        let wall = self
            .tiles
            .iter()
            .filter(|t| t.y == y && t.x >= x)
            .min_by(|t, t2| t.x.cmp(&t2.x));
        if let Some(tile) = wall {
            tile.x as f64 * self.tile_size
        } else {
            width
        }
    }

    pub fn player_pos(&self, pos: [f64; 2]) -> (u32, u32) {
        (
            (pos[0] / self.tile_size).floor() as u32,
            (pos[1] / self.tile_size).floor() as u32,
        )
    }

    pub fn flag_reached(&mut self) {
        if !self.is_flag_reached {
            self.flag = Tile::new_decoration(
                "flagGreen_up.png",
                self.flag.x,
                self.flag.y,
                false,
                0.3,
                -0.05,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub sprite_name: String,
    pub x: u32,
    pub y: u32,
    align_to_center: bool,
    rat_x: f64,
    rat_y: f64,
}

impl Tile {
    pub fn new_tile(sprite_name: &str, x: u32, y: u32) -> Tile {
        Tile {
            sprite_name: sprite_name.to_string(),
            x,
            y,
            align_to_center: false,
            rat_x: 0.0,
            rat_y: 0.0,
        }
    }

    pub fn new_decoration(
        sprite_name: &str,
        x: u32,
        y: u32,
        align_to_center: bool,
        rat_x: f64,
        rat_y: f64,
    ) -> Tile {
        Tile {
            sprite_name: sprite_name.to_string(),
            x,
            y,
            align_to_center,
            rat_x,
            rat_y,
        }
    }

    pub fn coords(&self, screen_height: f64, tile_size: f64, sprite_size: [f64; 2]) -> [f64; 2] {
        if self.align_to_center {
            [
                tile_size * self.x as f64 + (tile_size - sprite_size[0]) / 2.0,
                screen_height - self.y as f64 * tile_size - sprite_size[1],
            ]
        } else {
            [
                tile_size * (self.x as f64 + self.rat_x),
                screen_height - (1.0 + self.y as f64 + self.rat_y) * tile_size,
            ]
        }
    }
}
