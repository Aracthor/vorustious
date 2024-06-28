use std::collections::HashMap;

type EventReceiver = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    A,
    D,
    P,
    S,
    W,
    X,
    Y,
    Z,
    F5,
    F9,
    LeftCtrl,
    RightCtrl,
    LeftShift,
    RightShift,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Eq)]
pub enum Status {
    JustPressed,
    Pressed,
    JustReleased,
    Unpressed,
}

fn glfw_button_to_mouse_button(button: glfw::MouseButton) -> Option<MouseButton> {
    match button {
        glfw::MouseButtonLeft => Some(MouseButton::Left),
        glfw::MouseButtonRight => Some(MouseButton::Right),
        glfw::MouseButtonMiddle => Some(MouseButton::Middle),
        _ => None,
    }
}

fn glfw_action_to_status(action: glfw::Action) -> Option<Status> {
    match action {
        glfw::Action::Press => Some(Status::JustPressed),
        glfw::Action::Release => Some(Status::JustReleased),
        _ => None,
    }
}

pub struct EventHandler {
    core: glfw::Glfw,
    event_receiver: EventReceiver,

    key_status: HashMap<Key, Status>,
    mouse_button_status: HashMap<MouseButton, Status>,
    scroll_status: f32,
    cursor_movement: (f64, f64),
}

impl EventHandler {
    pub fn new(core: glfw::Glfw, event_receiver: EventReceiver) -> Self {
        Self {
            core: core,
            event_receiver: event_receiver,

            key_status: Default::default(),
            mouse_button_status: Default::default(),
            scroll_status: 0.0,
            cursor_movement: (0.0, 0.0),
        }
    }

    fn glfw_key_to_key(&self, key: glfw::Key) -> Option<Key> {
        match key {
            glfw::Key::A => Some(Key::A),
            glfw::Key::D => Some(Key::D),
            glfw::Key::P => Some(Key::P),
            glfw::Key::S => Some(Key::S),
            glfw::Key::W => Some(Key::W),
            glfw::Key::X => Some(Key::X),
            glfw::Key::Y => Some(Key::Y),
            glfw::Key::Z => Some(Key::Z),
            glfw::Key::F5 => Some(Key::F5),
            glfw::Key::F9 => Some(Key::F9),
            glfw::Key::LeftControl => Some(Key::LeftCtrl),
            glfw::Key::RightControl => Some(Key::RightCtrl),
            glfw::Key::LeftShift => Some(Key::LeftShift),
            glfw::Key::RightShift => Some(Key::RightShift),
            _ => None,
        }
    }

    pub fn update(&mut self) {
        for key in &mut self.key_status.values_mut() {
            if *key == Status::JustPressed {
                *key = Status::Pressed;
            }
            if *key == Status::JustReleased {
                *key = Status::Unpressed;
            }
        }
        for button in &mut self.mouse_button_status.values_mut() {
            if *button == Status::JustPressed {
                *button = Status::Pressed;
            }
            if *button == Status::JustReleased {
                *button = Status::Unpressed;
            }
        }
        self.scroll_status = 0.0;

        self.core.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::Key(glfw_key, _, action, _) => {
                    let key = self.glfw_key_to_key(glfw_key);
                    let status = glfw_action_to_status(action);
                    if key.is_some() && status.is_some() {
                        self.key_status.insert(key.unwrap(), status.unwrap());
                    }
                },

                glfw::WindowEvent::MouseButton(button, action, _) => {
                    let mouse_button = glfw_button_to_mouse_button(button);
                    let status = glfw_action_to_status(action);
                    if mouse_button.is_some() && status.is_some() {
                        self.mouse_button_status.insert(mouse_button.unwrap(), status.unwrap());
                    }
                }

                glfw::WindowEvent::Scroll(_x, y) => {
                    self.scroll_status = y as f32;
                },

                _ => {},
            }
        }
    }

    pub fn set_cursor_movement(&mut self, movement: (f64, f64)) {
        self.cursor_movement = movement;
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let key_status = self.key_status.get(&key);
        key_status.is_some() && [Status::JustPressed, Status::Pressed].contains(key_status.unwrap())
    }

    pub fn is_ctrl_pressed(&self) -> bool {
        self.is_key_pressed(Key::LeftCtrl) || self.is_key_pressed(Key::RightCtrl)
    }

    pub fn is_shift_pressed(&self) -> bool {
        self.is_key_pressed(Key::LeftShift) || self.is_key_pressed(Key::RightShift)
    }

    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        let key_status = self.key_status.get(&key);
        key_status.is_some_and(|status| *status == Status::JustPressed)
    }

    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        let button_status = self.mouse_button_status.get(&button);
        button_status.is_some_and(|status| *status == Status::JustReleased)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        let button_status = self.mouse_button_status.get(&button);
        button_status.is_some() && [Status::JustPressed, Status::Pressed].contains(button_status.unwrap())
    }

    pub fn scroll_status(&self) -> f32 {
        self.scroll_status
    }

    pub fn cursor_movement(&self) -> (f64, f64) { self.cursor_movement }
}
