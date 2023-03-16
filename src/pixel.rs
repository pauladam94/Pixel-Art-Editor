#[derive(Debug, Default)]
pub struct Pixel {
    color: egui::Color32,
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            color: egui::Color32::WHITE,
        }
    }

    pub fn draw(&self, ui: &egui::Ui, pos: egui::Pos2, size: egui::Vec2) {
        let rect = egui::Rect::from_min_size(pos, size);
        ui.painter().rect_filled(rect, 0.0, self.color);
    }

    pub fn fix_color(&mut self, color: egui::Color32) {
        self.color = color;
    }
}
