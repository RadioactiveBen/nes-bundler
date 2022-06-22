use crate::{
    input::{JoypadInput, JoypadKeyboardInput, JoypadKeyMap},
    MAX_PLAYERS
};
pub(crate) enum SelectedInput {
    Keyboard,
}

pub(crate) struct JoypadInputs {
    pub(crate) selected: SelectedInput,
    pub(crate) keyboard: JoypadKeyboardInput,
}

impl JoypadInputs {
    pub(crate) fn get_pad(&self) -> &dyn JoypadInput {
        match self.selected {
            SelectedInput::Keyboard => &self.keyboard,
        }
    }
}

pub(crate) struct Settings {
    pub(crate) audio_latency: u16,
    pub(crate) inputs: [JoypadInputs; MAX_PLAYERS],
}
const DEFAULT_INPUTS: [JoypadInputs; MAX_PLAYERS] = [
    JoypadInputs {
        selected: SelectedInput::Keyboard,
        keyboard: JoypadKeyboardInput::new(JoypadKeyMap::default_pad1()),
    },
    JoypadInputs {
        selected: SelectedInput::Keyboard,
        keyboard: JoypadKeyboardInput::new(JoypadKeyMap::default_pad2()),
    }
];

const DEFAULT_AUDIO_LATENCY: u16 = 20;

pub(crate) const DEFAULT: Settings = Settings {
    audio_latency: DEFAULT_AUDIO_LATENCY,
    inputs: DEFAULT_INPUTS,
};