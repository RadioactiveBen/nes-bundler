use self::{
    buttons::GamepadButton,
    gamepad::{Gamepads, JoypadGamepadMapping},
    gui::InputSettingsGui,
    keyboard::{JoypadKeyboardMapping, Keyboards},
    keys::{KeyCode, Mod},
    sdl2::Sdl2Gamepads,
    settings::InputConfigurationRef,
};
use crate::settings::{gui::GuiEvent, Settings, MAX_PLAYERS};
use ::sdl2::Sdl;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashSet, fmt::Debug, rc::Rc};

pub mod buttons;
pub mod gamepad;
pub mod gui;
pub mod keyboard;
pub mod keys;
pub mod sdl2;
pub mod settings;

#[derive(Debug)]
pub enum KeyEvent {
    Pressed(KeyCode, Mod),
    Released(KeyCode, Mod),
}

#[derive(Debug)]
pub enum GamepadEvent {
    ControllerAdded {
        which: InputId,
    },
    ControllerRemoved {
        which: InputId,
    },
    ButtonDown {
        which: InputId,
        button: GamepadButton,
    },
    ButtonUp {
        which: InputId,
        button: GamepadButton,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoypadButton {
    Up = 0b00010000,
    Down = 0b00100000,
    Left = 0b01000000,
    Right = 0b10000000,

    Start = 0b00001000,
    Select = 0b00000100,

    B = 0b00000010,
    A = 0b00000001,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct JoypadMapping<KeyType> {
    pub up: Option<KeyType>,
    pub down: Option<KeyType>,
    pub left: Option<KeyType>,
    pub right: Option<KeyType>,
    pub start: Option<KeyType>,
    pub select: Option<KeyType>,
    pub b: Option<KeyType>,
    pub a: Option<KeyType>,
}

impl<KeyType> JoypadMapping<KeyType>
where
    KeyType: PartialEq + Debug,
{
    pub fn lookup(&mut self, button: &JoypadButton) -> &mut Option<KeyType> {
        match button {
            JoypadButton::Up => &mut self.up,
            JoypadButton::Down => &mut self.down,
            JoypadButton::Left => &mut self.left,
            JoypadButton::Right => &mut self.right,
            JoypadButton::Start => &mut self.start,
            JoypadButton::Select => &mut self.select,
            JoypadButton::B => &mut self.b,
            JoypadButton::A => &mut self.a,
        }
    }

    fn insert_if_mapped(
        buttons: &mut HashSet<JoypadButton>,
        mapping: &Option<KeyType>,
        a_key: &KeyType,
        button: JoypadButton,
    ) {
        if let Some(key) = mapping {
            if a_key.eq(key) {
                buttons.insert(button);
            }
        }
    }
    fn reverse_lookup(&self, key: &KeyType) -> HashSet<JoypadButton> {
        let mut buttons = HashSet::new();
        JoypadMapping::insert_if_mapped(&mut buttons, &self.up, key, JoypadButton::Up);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.down, key, JoypadButton::Down);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.left, key, JoypadButton::Left);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.right, key, JoypadButton::Right);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.start, key, JoypadButton::Start);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.select, key, JoypadButton::Select);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.b, key, JoypadButton::B);
        JoypadMapping::insert_if_mapped(&mut buttons, &self.a, key, JoypadButton::A);
        buttons
    }

    fn calculate_state(&self, keys: &HashSet<KeyType>) -> JoypadInput {
        JoypadInput(keys.iter().fold(0_u8, |mut acc, key| {
            for button in self.reverse_lookup(key) {
                acc |= button as u8;
            }
            acc
        }))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JoypadInput(pub u8);

impl JoypadInput {
    pub fn is_pressed(&self, button: JoypadButton) -> bool {
        self.0 & (button as u8) != 0
    }
}

pub type InputId = String;
pub trait ToInputId {
    fn to_input_id(&self) -> InputId;
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct InputConfiguration {
    pub id: InputId,
    pub name: String,
    pub kind: InputConfigurationKind,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum InputConfigurationKind {
    Keyboard(JoypadKeyboardMapping),
    Gamepad(JoypadGamepadMapping),
}
pub struct Inputs {
    keyboards: Keyboards,
    gamepads: Box<dyn Gamepads>,
    joypads: [JoypadInput; MAX_PLAYERS],
    default_input_configurations: [InputConfigurationRef; MAX_PLAYERS],
}

impl Inputs {
    pub fn new(
        sdl_context: &Sdl,
        default_input_configurations: [InputConfigurationRef; MAX_PLAYERS],
    ) -> Self {
        let gamepads = Box::new(Sdl2Gamepads::new(sdl_context));
        let keyboards = Keyboards::new();

        Self {
            keyboards,
            gamepads,
            joypads: [JoypadInput(0), JoypadInput(0)],
            default_input_configurations,
        }
    }

    pub fn advance(&mut self, event: &GuiEvent, settings: Rc<RefCell<Settings>>) {
        match event {
            GuiEvent::Keyboard(key_event) => {
                self.keyboards.advance(key_event);
            }
            GuiEvent::Gamepad(gamepad_event) => {
                self.gamepads
                    .advance(gamepad_event, &mut settings.borrow_mut().input);
            }
        }
        let input_settings = &mut settings.borrow_mut().input;
        input_settings.reset_selected_disconnected_inputs(self);

        self.joypads[0] =
            self.get_joypad_for_input_configuration(&input_settings.selected[0].borrow());
        self.joypads[1] =
            self.get_joypad_for_input_configuration(&input_settings.selected[1].borrow());
    }

    pub fn get_joypad(&self, player: usize) -> JoypadInput {
        self.joypads[player]
    }

    pub fn get_default_conf(&self, player: usize) -> &InputConfigurationRef {
        &self.default_input_configurations[player]
    }

    fn get_joypad_for_input_configuration(
        &mut self,
        input_conf: &InputConfiguration,
    ) -> JoypadInput {
        match &input_conf.kind {
            InputConfigurationKind::Keyboard(mapping) => self.keyboards.get_joypad(mapping),
            InputConfigurationKind::Gamepad(mapping) => {
                self.gamepads.get_joypad(&input_conf.id, mapping)
            }
        }
    }

    pub fn is_connected(&self, input_conf: &InputConfiguration) -> bool {
        match &input_conf.kind {
            InputConfigurationKind::Keyboard(_) => true,
            InputConfigurationKind::Gamepad(_) => self
                .gamepads
                .get_gamepad_by_input_id(&input_conf.id)
                .map(|state| state.is_connected())
                .unwrap_or(false),
        }
    }

    pub fn remap_configuration(
        &mut self,
        input_configuration: &InputConfigurationRef,
        button: &JoypadButton,
    ) -> bool {
        let mut input_configuration = input_configuration.borrow_mut();
        let input_configuration_id = input_configuration.id.clone();
        match &mut input_configuration.kind {
            InputConfigurationKind::Keyboard(mapping) => {
                if let Some(code) = self.keyboards.pressed_keys.iter().next() {
                    //If there's any key pressed, use the first found.
                    let _ = mapping.lookup(button).insert(*code);
                    return true;
                }
            }
            InputConfigurationKind::Gamepad(mapping) => {
                if let Some(state) = &mut self
                    .gamepads
                    .get_gamepad_by_input_id(&input_configuration_id)
                {
                    if let Some(new_button) = state.get_pressed_buttons().iter().next() {
                        //If there's any button pressed, use the first found.
                        let _ = mapping.lookup(button).insert(*new_button);
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub struct Input {
    settings: Rc<RefCell<Settings>>,
    pub inputs: Inputs,
    gui: InputSettingsGui,
}
impl Input {
    pub fn new(inputs: Inputs, settings: Rc<RefCell<Settings>>) -> Self {
        Input {
            settings,
            inputs,
            gui: Default::default(),
        }
    }
}
