use piston::Key;
use platformer::constants::*;
use platformer::map::Map;
use std::time::SystemTime;
use utils::entity::Updatable;
use utils::input::GameInput;

#[derive(Clone)]
pub struct Player {
    pub size: [f64; 2],
    pub pos: [f64; 2],
    state: PlayerState,
    frame_ctr: usize,
    frames: Vec<String>,
    last_frame_time: SystemTime,
    action_timer: f64,
    speed: f64,
    can_jump: bool,
}

impl Player {
    pub fn new(size: [f64; 2]) -> Player {
        Player {
            size,
            pos: [32.0, 192.0],
            state: PlayerState::Stand,
            frame_ctr: 0,
            frames: PlayerState::Stand.frames(),
            last_frame_time: SystemTime::now(),
            action_timer: 0.0,
            speed: 0.0,
            can_jump: true,
        }
    }

    pub fn render_args(&self) -> RenderArgs {
        let sprite = self.frames[self.frame_ctr].clone();
        RenderArgs {
            sprite,
            pos: self.pos,
            size: self.size,
        }
    }

    fn set_state(&mut self, state: PlayerState) {
        self.frames = state.frames();
        self.state = state;
        self.frame_ctr = 0;
    }

    fn update_frame(&mut self) {
        let dt = SystemTime::now()
            .duration_since(self.last_frame_time)
            .unwrap()
            .as_secs_f64();
        if dt >= 1.0 / PLAYER_FPS as f64 {
            self.frame_ctr = (self.frame_ctr + 1) % self.frames.len();
            if self.state == PlayerState::AscendStart && self.frame_ctr == 0 {
                self.set_state(PlayerState::Ascend)
            }
            self.last_frame_time = SystemTime::now();
        }
    }
}

impl Updatable for Player {
    type Args = PlayerUpdateArgs;

    fn update(&mut self, args: Self::Args) {
        let floor = args.map.floor_under_position(self.pos);
        match self.state {
            PlayerState::Stand => {
                if args.input.is_key_pressed(Key::Space) && self.can_jump {
                    self.set_state(PlayerState::AscendStart);
                    self.action_timer = ASCEND_TIME;
                } else if args.input.is_key_pressed(Key::D) {
                    self.set_state(PlayerState::Move);
                    self.speed = (args.dt / SPEED_UP_TIME) * MAX_SPEED;
                } else if args.input.is_key_pressed(Key::A) {
                    self.set_state(PlayerState::Move);
                    self.speed = (args.dt / SPEED_UP_TIME) * -MAX_SPEED;
                }
                if self.pos[1] > floor {
                    self.set_state(PlayerState::Descend);
                }
            }
            PlayerState::Move => {
                if args.input.is_key_pressed(Key::Space) && self.can_jump {
                    self.set_state(PlayerState::AscendStart);
                    self.action_timer = ASCEND_TIME;
                }
                if self.pos[1] > floor {
                    self.set_state(PlayerState::Descend);
                }
            }
            PlayerState::AscendStart | PlayerState::Ascend => {
                if self.action_timer > 0.0 {
                    let dt = if args.dt < self.action_timer {
                        args.dt
                    } else {
                        self.action_timer
                    };
                    let ceiling = args
                        .map
                        .ceiling_over_position(self.pos, args.screen_size[1]);
                    self.pos[1] += (dt / ASCEND_TIME) * MAX_ASCEND;
                    self.action_timer -= args.dt;
                    if self.pos[1] > ceiling - self.size[1] {
                        self.pos[1] = ceiling - self.size[1];
                        self.set_state(PlayerState::Float);
                        self.action_timer = 0.0;
                    }
                } else {
                    self.set_state(PlayerState::Float);
                    self.action_timer = FLOAT_TIME;
                }
            }
            PlayerState::Float => {
                if self.action_timer > 0.0 {
                    self.action_timer -= args.dt;
                } else {
                    self.set_state(PlayerState::Descend);
                }
            }
            PlayerState::Descend => {
                if self.pos[1] > floor {
                    let desc_dt = args.dt * DESCEND_SPEED;
                    let desc_floor = self.pos[1] - floor;
                    self.pos[1] -= if desc_dt < desc_floor {
                        desc_dt
                    } else {
                        desc_floor
                    };
                    if self.pos[1] <= floor {
                        self.pos[1] = floor
                    }
                } else {
                    if self.speed == 0.0 {
                        self.set_state(PlayerState::Stand)
                    } else {
                        self.set_state(PlayerState::Move)
                    }
                    self.action_timer = JUMP_COOL_DOWN;
                    self.can_jump = false;
                }
            }
        }
        let speed_change = (args.dt / SPEED_UP_TIME) * MAX_SPEED;
        if args.input.is_key_pressed(Key::D) {
            self.speed += speed_change;
            if self.speed > MAX_SPEED {
                self.speed = MAX_SPEED
            }
        } else if args.input.is_key_pressed(Key::A) {
            self.speed -= speed_change;
            if self.speed < -MAX_SPEED {
                self.speed = -MAX_SPEED
            }
        } else {
            if self.speed < 0.0 {
                self.speed += speed_change;
                if self.speed > 0.0 {
                    self.speed = 0.0;
                    if self.state == PlayerState::Move {
                        self.set_state(PlayerState::Stand);
                    }
                }
            } else if self.speed > 0.0 {
                self.speed -= speed_change;
                if self.speed < 0.0 {
                    self.speed = 0.0;
                    if self.state == PlayerState::Move {
                        self.set_state(PlayerState::Stand);
                    }
                }
            }
        }
        if !self.can_jump {
            self.action_timer -= args.dt;
            if self.action_timer <= 0.0 {
                self.action_timer = 0.0;
                self.can_jump = true;
            }
        }
        if self.speed != 0.0 {
            self.pos[0] += args.dt * self.speed;
        }
        let left = args.map.wall_at_left(self.pos);
        let right = args.map.wall_at_right(self.pos, args.screen_size[0]);
        let half_width = self.size[0] / 2.0;
        if self.pos[0] > right - half_width {
            self.pos[0] = right - half_width;
            self.speed = 0.0;
            if self.state == PlayerState::Move {
                self.set_state(PlayerState::Stand);
            }
        }
        if self.pos[0] < left + half_width {
            self.pos[0] = left + half_width;
            self.speed = 0.0;
            if self.state == PlayerState::Move {
                self.set_state(PlayerState::Stand);
            }
        }
        self.update_frame();
    }
}

pub struct PlayerUpdateArgs {
    pub dt: f64,
    pub input: GameInput,
    pub screen_size: [f64; 2],
    pub map: Map,
}

#[derive(Copy, Clone, PartialEq)]
enum PlayerState {
    Stand,
    Move,
    AscendStart,
    Ascend,
    Float,
    Descend,
}

impl PlayerState {
    pub fn frames(self) -> Vec<String> {
        match self {
            PlayerState::Stand => vec!["playerRed_stand.png".to_string()],
            PlayerState::Move => vec![
                "playerRed_walk1.png".to_string(),
                "playerRed_walk2.png".to_string(),
                "playerRed_walk3.png".to_string(),
                "playerRed_walk2.png".to_string(),
            ],
            PlayerState::AscendStart => vec![
                "playerRed_up1.png".to_string(),
                "playerRed_up2.png".to_string(),
            ],
            PlayerState::Ascend => vec!["playerRed_up3.png".to_string()],
            PlayerState::Float => vec!["playerRed_up3.png".to_string()],
            PlayerState::Descend => vec!["playerRed_fall.png".to_string()],
        }
    }
}

#[derive(Debug)]
pub struct RenderArgs {
    pub sprite: String,
    pub pos: [f64; 2],
    pub size: [f64; 2],
}
