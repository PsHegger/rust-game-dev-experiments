use piston::WindowSettings;
use piston_window::*;
use utils::fps_counter::FpsCounter;
use utils::scene::Scene;

pub struct GameWindow<T: Scene + Clone> {
    window: PistonWindow,
    glyphs: Glyphs,
    fps_counter: FpsCounter,
    scene: T,
}

impl<T: Scene + Clone> GameWindow<T> {
    pub fn new(settings: WindowSettings, scene: T) -> GameWindow<T> {
        let mut window: PistonWindow = settings.build().unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let glyphs = window.load_font(assets.join("retro_gaming.ttf")).unwrap();

        GameWindow {
            window,
            glyphs,
            fps_counter: FpsCounter::default(),
            scene,
        }
    }

    pub fn game_loop(&mut self) {
        while let Some(e) = self.window.next() {
            match e {
                Event::Loop(l) => match l {
                    Loop::Render(_) => {
                        let fps = self.fps_counter.fps;
                        let glyphs = &mut self.glyphs;
                        let scene = self.scene.clone();
                        self.window.draw_2d(&e, |c, g, device| {
                            clear([1.0; 4], g);
                            scene.render(c, g);

                            let transform = c.transform.trans(5.0, 21.0);
                            text::Text::new(16)
                                .draw(
                                    format!("FPS: {}", fps).as_str(),
                                    glyphs,
                                    &c.draw_state,
                                    transform,
                                    g,
                                )
                                .unwrap();

                            glyphs.factory.encoder.flush(device);
                        });
                    }
                    Loop::Update(_) => {
                        self.scene.update();
                        self.fps_counter.on_update();
                    }
                    _ => {}
                },
                Event::Input(i, _) => match i {
                    Input::Resize(args) => {
                        self.scene.on_resize(args.draw_size[0], args.draw_size[1]);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
