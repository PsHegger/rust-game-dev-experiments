use piston_window::*;
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    atlas: TextureAtlas,
    texture: G2dTexture,
}

#[allow(dead_code)]
impl SpriteSheet {
    pub fn new(
        assets_folder: &str,
        sprites: &str,
        texture_context: &mut G2dTextureContext,
    ) -> SpriteSheet {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder(assets_folder)
            .expect("Assets folder missing");

        let file = File::open(assets.join(sprites)).expect("Missing descriptor for sprites");
        let reader = BufReader::new(file);

        let atlas: TextureAtlas = from_reader(reader).expect("Invalid sprite descriptor format");

        let texture_path = assets.join(&atlas.image_path);
        let texture: G2dTexture = Texture::from_path(
            texture_context,
            &texture_path,
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Cannot load sprites as texture");

        SpriteSheet { atlas, texture }
    }

    pub fn render_sprite(&self, name: &String, pos: [f64; 2], c: Context, g: &mut G2d) {
        let sub_texture = self.atlas.sub_textures.iter().find(|t| t.name == *name);

        if let Some(sprite) = sub_texture {
            let src_rect = [sprite.x, sprite.y, sprite.width, sprite.height];
            Image::new().src_rect(src_rect).draw(
                &self.texture,
                &c.draw_state,
                c.transform.trans(pos[0], pos[1]),
                g,
            );
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct TextureAtlas {
    pub image_path: String,
    #[serde(rename = "SubTexture")]
    pub sub_textures: Vec<SubTexture>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct SubTexture {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub frame_x: f64,
    pub frame_y: f64,
    pub frame_width: f64,
    pub frame_height: f64,
}
