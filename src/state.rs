#[derive(Debug, PartialEq)]
pub enum State {
    Idle,
    Drag,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Idle => write!(f, "Idle")?,
            State::Drag => write!(f, "Drag")?,
        }
        Ok(())
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, text: String) {
        ui.horizontal(|ui| {
            ui.label(text);
            ui.radio_value(self, State::Idle, "Idle");
            ui.radio_value(self, State::Drag, "Drag");
        });
    }
}
