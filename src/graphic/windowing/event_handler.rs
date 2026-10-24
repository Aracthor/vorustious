use std::collections::HashMap;

type EventReceiver = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    A,
    D,
    E,
    P,
    Q,
    S,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F8,
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum GamepadButton {
    A,
    B,
    L1,
    L3,
    R1,
    R3,
    X,
    Y,
}

impl GamepadButton {
    pub fn to_glfw_button(&self) -> glfw::GamepadButton {
        match self {
            Self::A => glfw::GamepadButton::ButtonA,
            Self::B => glfw::GamepadButton::ButtonB,
            Self::L1 => glfw::GamepadButton::ButtonLeftBumper,
            Self::L3 => glfw::GamepadButton::ButtonLeftThumb,
            Self::R1 => glfw::GamepadButton::ButtonRightBumper,
            Self::R3 => glfw::GamepadButton::ButtonRightThumb,
            Self::X => glfw::GamepadButton::ButtonX,
            Self::Y => glfw::GamepadButton::ButtonY,
        }
    }
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
    gamepad_button_status: HashMap<GamepadButton, Status>,
    scroll_status: f32,
    cursor_movement: (f64, f64),
    left_stick_movement: (f32, f32),
    right_stick_movement: (f32, f32),
}

impl EventHandler {
    pub fn new(core: glfw::Glfw, event_receiver: EventReceiver) -> Self {
        let mut gamepad_button_status = HashMap::<GamepadButton, Status>::default();
        [
            GamepadButton::A,
            GamepadButton::B,
            GamepadButton::L1,
            GamepadButton::L3,
            GamepadButton::R1,
            GamepadButton::R3,
            GamepadButton::X,
            GamepadButton::Y,
        ].iter().for_each(|button| { gamepad_button_status.insert(*button, Status::Unpressed); });

        Self {
            core: core,
            event_receiver: event_receiver,

            key_status: Default::default(),
            mouse_button_status: Default::default(),
            gamepad_button_status: gamepad_button_status,
            scroll_status: 0.0,
            cursor_movement: (0.0, 0.0),
            left_stick_movement: (0.0, 0.0),
            right_stick_movement: (0.0, 0.0),
        }
    }

    fn glfw_key_to_key(&self, key: glfw::Key) -> Option<Key> {
        match key {
            glfw::Key::A => Some(Key::A),
            glfw::Key::D => Some(Key::D),
            glfw::Key::E => Some(Key::E),
            glfw::Key::P => Some(Key::P),
            glfw::Key::Q => Some(Key::Q),
            glfw::Key::S => Some(Key::S),
            glfw::Key::W => Some(Key::W),
            glfw::Key::X => Some(Key::X),
            glfw::Key::Y => Some(Key::Y),
            glfw::Key::Z => Some(Key::Z),
            glfw::Key::F1 => Some(Key::F1),
            glfw::Key::F2 => Some(Key::F2),
            glfw::Key::F3 => Some(Key::F3),
            glfw::Key::F4 => Some(Key::F4),
            glfw::Key::F5 => Some(Key::F5),
            glfw::Key::F8 => Some(Key::F8),
            glfw::Key::F9 => Some(Key::F9),
            glfw::Key::LeftControl => Some(Key::LeftCtrl),
            glfw::Key::RightControl => Some(Key::RightCtrl),
            glfw::Key::LeftShift => Some(Key::LeftShift),
            glfw::Key::RightShift => Some(Key::RightShift),
            _ => None,
        }
    }

    pub fn update(&mut self) {
        let update_status = |status: &mut Status| {
            if *status == Status::JustPressed {
                *status = Status::Pressed;
            }
            if *status == Status::JustReleased {
                *status = Status::Unpressed;
            }
        };
        self.key_status.values_mut().for_each(update_status);
        self.mouse_button_status.values_mut().for_each(update_status);
        self.gamepad_button_status.values_mut().for_each(update_status);
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

        self.left_stick_movement = (0.0, 0.0);
        self.right_stick_movement = (0.0, 0.0);
        let gamepad = self.core.get_joystick(glfw::JoystickId::Joystick1);
        if gamepad.is_present() && gamepad.is_gamepad() {
            if gamepad.get_gamepad_state().is_some() {
                let state = gamepad.get_gamepad_state().unwrap();

                for (button, status) in self.gamepad_button_status.iter_mut() {
                    match state.get_button_state(button.to_glfw_button()) {
                        glfw::Action::Release => {
                            if *status == Status::Pressed {
                                *status = Status::JustReleased;
                            }
                        },
                        glfw::Action::Press => {
                            if *status == Status::Unpressed {
                                *status = Status::JustPressed;
                            }
                        },
                        glfw::Action::Repeat => {}
                    }
                }

                let get_smoothed_stick_axis = |axis| {
                    let axis_value = state.get_axis(axis);
                    if axis_value.abs() < 0.1 {
                        return 0.0;
                    }
                    axis_value
                };
                self.left_stick_movement.0 = get_smoothed_stick_axis(glfw::GamepadAxis::AxisLeftX);
                self.left_stick_movement.1 = get_smoothed_stick_axis(glfw::GamepadAxis::AxisLeftY);
                self.right_stick_movement.0 = get_smoothed_stick_axis(glfw::GamepadAxis::AxisRightX);
                self.right_stick_movement.1 = get_smoothed_stick_axis(glfw::GamepadAxis::AxisRightY);
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
        button_status.is_some_and(|button| [Status::JustPressed, Status::Pressed].contains(button))
    }

    pub fn is_gamepad_button_pressed(&self, button: GamepadButton) -> bool {
        let button_status = self.gamepad_button_status.get(&button);
        button_status.is_some_and(|button| [Status::JustPressed, Status::Pressed].contains(button))
    }

    pub fn scroll_status(&self) -> f32 { self.scroll_status }
    pub fn cursor_movement(&self) -> (f64, f64) { self.cursor_movement }
    pub fn left_stick_movement(&self) -> (f32, f32) { self.left_stick_movement }
    pub fn right_stick_movement(&self) -> (f32, f32) { self.right_stick_movement }
}
