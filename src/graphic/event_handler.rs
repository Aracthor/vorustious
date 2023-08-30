use std::collections::HashMap;

type EventReceiver = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    A,
    D,
    S,
    W,
}

#[derive(PartialEq, Eq)]
pub enum KeyStatus {
    Pressed,
    Unpressed,
}

pub struct EventHandler {
    core: glfw::Glfw,
    event_receiver: EventReceiver,

    glfw_key_table: HashMap<glfw::Key, Key>,

    key_status: HashMap<Key, KeyStatus>,
}

impl EventHandler {
    pub fn new(core: glfw::Glfw, event_receiver: EventReceiver) -> Self {
        Self {
            core:core,
            event_receiver:event_receiver,

            glfw_key_table: HashMap::from([
                (glfw::Key::A, Key::A),
                (glfw::Key::D, Key::D),
                (glfw::Key::S, Key::S),
                (glfw::Key::W, Key::W),
            ]),

            key_status: Default::default(),
            }
    }

    pub fn update(&mut self) {
        self.core.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::Key(glfw_key, _, action, _) => {
                    let key = self.glfw_key_table.get(&glfw_key);
                    if key.is_some() {
                        match action {
                            glfw::Action::Press => {
                                self.key_status.insert(*key.unwrap(), KeyStatus::Pressed);
                            }
                            glfw::Action::Release => {
                                self.key_status.insert(*key.unwrap(), KeyStatus::Unpressed);
                            }
                            _ => {}
                        }
                    }
                },
                _ => {},
            }
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let key_status = self.key_status.get(&key);
        key_status.is_some() && *key_status.unwrap() == KeyStatus::Pressed
    }
}
