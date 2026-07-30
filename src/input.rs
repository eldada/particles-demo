use winit::event::{ElementState, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub lmb: bool,
    pub rmb: bool,
    pub shift: bool,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_delta_x: f64,
    pub mouse_delta_y: f64,
    pub scroll_delta: f32,
    pub orbit_dragging: bool,
    pub want_quit: bool,
    pub reset_particles: bool,
    pub clear_gravity: bool,
    pub toggle_pause: bool,
    pub toggle_hud: bool,
    pub toggle_fullscreen: bool,
    pub slower: bool,
    pub faster: bool,
    pub count_down: bool,
    pub count_up: bool,
    pub preset_digit: Option<u8>,
}

impl InputState {
    pub fn begin_frame(&mut self) {
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
        self.scroll_delta = 0.0;
        self.reset_particles = false;
        self.clear_gravity = false;
        self.toggle_pause = false;
        self.toggle_hud = false;
        self.toggle_fullscreen = false;
        self.slower = false;
        self.faster = false;
        self.count_down = false;
        self.count_up = false;
        self.preset_digit = None;
    }

    pub fn on_key(&mut self, key: PhysicalKey, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        let PhysicalKey::Code(code) = key else {
            return;
        };

        match code {
            KeyCode::Escape => {
                if pressed {
                    self.want_quit = true;
                }
            }
            KeyCode::ArrowUp => self.up = pressed,
            KeyCode::ArrowDown => self.down = pressed,
            KeyCode::ArrowLeft => self.left = pressed,
            KeyCode::ArrowRight => self.right = pressed,
            KeyCode::ShiftLeft | KeyCode::ShiftRight => self.shift = pressed,
            KeyCode::Space => {
                if pressed {
                    self.reset_particles = true;
                }
            }
            KeyCode::KeyR => {
                if pressed {
                    self.clear_gravity = true;
                }
            }
            KeyCode::KeyP => {
                if pressed {
                    self.toggle_pause = true;
                }
            }
            KeyCode::KeyH => {
                if pressed {
                    self.toggle_hud = true;
                }
            }
            KeyCode::KeyF => {
                if pressed {
                    self.toggle_fullscreen = true;
                }
            }
            KeyCode::Comma => {
                if pressed {
                    self.slower = true;
                }
            }
            KeyCode::Period => {
                if pressed {
                    self.faster = true;
                }
            }
            KeyCode::BracketLeft => {
                if pressed {
                    self.count_down = true;
                }
            }
            KeyCode::BracketRight => {
                if pressed {
                    self.count_up = true;
                }
            }
            KeyCode::Digit1 => {
                if pressed {
                    self.preset_digit = Some(1);
                }
            }
            KeyCode::Digit2 => {
                if pressed {
                    self.preset_digit = Some(2);
                }
            }
            KeyCode::Digit3 => {
                if pressed {
                    self.preset_digit = Some(3);
                }
            }
            KeyCode::Digit4 => {
                if pressed {
                    self.preset_digit = Some(4);
                }
            }
            KeyCode::Digit5 => {
                if pressed {
                    self.preset_digit = Some(5);
                }
            }
            _ => {}
        }
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        match button {
            MouseButton::Left => self.lmb = pressed,
            MouseButton::Right => {
                self.rmb = pressed;
                self.orbit_dragging = pressed;
            }
            _ => {}
        }
    }

    pub fn on_cursor_moved(&mut self, x: f64, y: f64) {
        if self.mouse_x != 0.0 || self.mouse_y != 0.0 {
            self.mouse_delta_x += x - self.mouse_x;
            self.mouse_delta_y += y - self.mouse_y;
        }
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn on_scroll(&mut self, delta_y: f32) {
        self.scroll_delta += delta_y;
    }
}
