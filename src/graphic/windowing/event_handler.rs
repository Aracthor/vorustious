use std::collections::HashMap;

type EventReceiver = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    A,
    D,
    S,
    W,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Eq)]
pub enum Status {
    Pressed,
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
        glfw::Action::Press => Some(Status::Pressed),
        glfw::Action::Release => Some(Status::Unpressed),
        _ => None,
    }
}

pub struct EventHandler {
    core: glfw::Glfw,
    event_receiver: EventReceiver,

    glfw_key_table: HashMap<glfw::Key, Key>,

    key_status: HashMap<Key, Status>,
    mouse_button_status: HashMap<MouseButton, Status>,
    cursor_movement: (f64, f64),
}

impl EventHandler {
    pub fn new(core: glfw::Glfw, event_receiver: EventReceiver) -> Self {
        Self {
            core: core,
            event_receiver: event_receiver,

            glfw_key_table: HashMap::from([
                (glfw::Key::A, Key::A),
                (glfw::Key::D, Key::D),
                (glfw::Key::S, Key::S),
                (glfw::Key::W, Key::W),
            ]),

            key_status: Default::default(),
            mouse_button_status: Default::default(),
            cursor_movement: (0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.core.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::Key(glfw_key, _, action, _) => {
                    let key = self.glfw_key_table.get(&glfw_key);
                    let status = glfw_action_to_status(action);
                    if key.is_some() && status.is_some() {
                        self.key_status.insert(*key.unwrap(), status.unwrap());
                    }
                },

                glfw::WindowEvent::MouseButton(button, action, _) => {
                    let mouse_button = glfw_button_to_mouse_button(button);
                    let status = glfw_action_to_status(action);
                    if mouse_button.is_some() && status.is_some() {
                        self.mouse_button_status.insert(mouse_button.unwrap(), status.unwrap());
                    }
                }

                _ => {},
            }
        }
    }

    pub fn set_cursor_movement(&mut self, movement: (f64, f64)) {
        self.cursor_movement = movement;
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let key_status = self.key_status.get(&key);
        key_status.is_some() && *key_status.unwrap() == Status::Pressed
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        let button_status = self.mouse_button_status.get(&button);
        button_status.is_some() && *button_status.unwrap() == Status::Pressed
    }

    pub fn cursor_movement(&self) -> (f64, f64) { self.cursor_movement }
}
