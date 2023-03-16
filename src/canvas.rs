use crate::pixel::Pixel;
use crate::state::State;

#[derive(Debug, PartialEq)]
pub enum StateCanvas {
    Drag,
    Draw,
}
impl Default for StateCanvas {
    fn default() -> Self {
        Self::Draw
    }
}

#[derive(Debug)]
pub struct Canvas {
    pos: egui::Pos2,
    pub state: StateCanvas,

    stroke: egui::Stroke,
    pub width_pixel: f32,
    pub height_pixel: f32,
    max_line: usize,
    max_col: usize,
    pub line: usize,
    pub col: usize,
    pixels: Vec<Vec<Pixel>>,
    color_pick_right_click: egui::Color32,
    color_pick_left_click: egui::Color32,

    drag_start: egui::Pos2,
    previous_drag: egui::Pos2,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new(
            200,
            200,
            egui::Stroke::new(2.0, egui::Color32::BLUE),
            egui::Pos2::new(200.0, 200.0),
        )
    }
}

impl Canvas {
    pub fn new(max_line: usize, max_col: usize, stroke: egui::Stroke, pos: egui::Pos2) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..max_line {
            let mut row = Vec::new();
            for _ in 0..max_col {
                row.push(Pixel::new());
            }
            pixels.push(row);
        }
        Self {
            pos,
            state: StateCanvas::default(),
            stroke,
            width_pixel: 10.0,
            height_pixel: 10.0,
            max_line,
            max_col,
            line: max_line / 2,
            col: max_col / 2,
            pixels,
            color_pick_left_click: egui::Color32::BLACK,
            color_pick_right_click: egui::Color32::WHITE,
            drag_start: egui::Pos2::ZERO,
            previous_drag: egui::Pos2::ZERO,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.line {
            for j in 0..self.col {
                self.pixels[i][j].fix_color(egui::Color32::WHITE);
            }
        }
    }

    pub fn contains(&self, pos: egui::Pos2) -> bool {
        let size = egui::Vec2::new(
            self.width_pixel * self.col as f32,
            self.height_pixel * self.line as f32,
        );
        let rect = egui::Rect::from_min_size(self.pos, size);
        rect.contains(pos)
    }

    pub fn move_canvas(&mut self, dx: f32, dy: f32) {
        self.pos.x += dx;
        self.pos.y += dy;
    }

    // Bresenham algorithm to calculate a line in the canvas
    // from two floating points in the canvas
    // we have to take into account the height and width of the pixels
    pub fn bresenham(&self, start: egui::Pos2, end: egui::Pos2) -> Vec<(usize, usize)> {
        let mut line = Vec::new();
        let mut x0 = (start.x - self.pos.x) / self.width_pixel;
        let mut y0 = (start.y - self.pos.y) / self.height_pixel;
        let mut x1 = (end.x - self.pos.x) / self.width_pixel;
        let mut y1 = (end.y - self.pos.y) / self.height_pixel;

        let mut steep = false;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2.0;
        let mut error2 = 0.0;
        let mut y = y0 as i32;
        for x in x0 as i32..=x1 as i32 {
            if steep {
                line.push((y as usize, x as usize));
            } else {
                line.push((x as usize, y as usize));
            }
            error2 += derror2;
            if error2 > dx {
                if y1 > y0 {
                    y += 1;
                } else {
                    y -= 1;
                }
                error2 -= dx * 2.0;
            }
        }
        line
    }

    pub fn draw(&self, ui: &egui::Ui) {
        // Draw the grid of pixels
        for i in 0..self.line {
            for j in 0..self.col {
                let pos = egui::Pos2::new(
                    self.pos.x + self.width_pixel * j as f32,
                    self.pos.y + self.height_pixel * i as f32,
                );
                let size = egui::Vec2::new(self.width_pixel, self.height_pixel);
                self.pixels[i][j].draw(ui, pos, size);
            }
        }

        // Draw the vertical lines
        for j in 0..self.col + 1 {
            let pos = egui::Pos2::new(self.pos.x + self.width_pixel * j as f32, self.pos.y);
            ui.painter().line_segment(
                [
                    pos,
                    pos + egui::Vec2::new(0.0, self.height_pixel * self.line as f32),
                ],
                self.stroke,
            );
        }

        // Draw the horizontal lines
        for i in 0..self.line + 1 {
            let pos = egui::Pos2::new(self.pos.x, self.pos.y + self.height_pixel * i as f32);
            ui.painter().line_segment(
                [
                    pos,
                    pos + egui::Vec2::new(self.width_pixel * self.col as f32, 0.0),
                ],
                self.stroke,
            );
        }
    }

    pub fn stroke_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("canvas").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Color");
                ui.color_edit_button_srgba(&mut self.color_pick_left_click)
                    .on_hover_text("Left click color");
                ui.color_edit_button_srgba(&mut self.color_pick_right_click)
                    .on_hover_text("Right click color");
            });
            ui.end_row();
            egui::stroke_ui(ui, &mut self.stroke, "Stroke canvas");
            ui.end_row();
            ui.add(
                egui::Slider::new(&mut self.width_pixel, 1.0..=100.0)
                    .text("Width and height of a pixel"),
            );
            ui.end_row();

            self.height_pixel = self.width_pixel;

            ui.add(egui::Slider::new(&mut self.line, 1..=self.max_line).text("Number of lines"));
            ui.end_row();

            ui.add(egui::Slider::new(&mut self.col, 1..=self.max_col).text(" Number of columns"));
            ui.end_row();
        });
    }

    pub fn state_ui(&mut self, ui: &mut egui::Ui, text: String) {
        ui.horizontal(|ui| {
            ui.label(text);
            ui.radio_value(&mut self.state, StateCanvas::Drag, "Drag");
            ui.radio_value(&mut self.state, StateCanvas::Draw, "Draw");
        });
    }

    pub fn handle_event(&mut self, event: &egui::Event, state: &mut State) {
        match state {
            State::Idle => {
                if let egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    ..
                } = event
                {
                    if self.contains(*pos) & *pressed {
                        self.drag_start = *pos;
                        self.previous_drag = *pos;
                        *state = State::Drag;
                    }
                }
            }
            State::Drag => match event {
                egui::Event::PointerButton { pressed, .. } => {
                    if !*pressed {
                        *state = State::Idle;
                    }
                }
                egui::Event::PointerMoved(pos) => match self.state {
                    StateCanvas::Draw => {
                        if self.contains(*pos) {
                            let delta = *pos - self.pos;
                            let line = (delta.y / self.height_pixel) as usize;
                            let col = (delta.x / self.width_pixel) as usize;

                            if line < self.line && col < self.col {
                                for (i, j) in self.bresenham(self.previous_drag, *pos) {
                                    self.pixels[j][i].fix_color(self.color_pick_left_click);
                                    // symetry features
                                    // if self.symetry {
                                    //    self.pixels[i][j].fix_color(self.color_pick_left_click);
                                    // }
                                }
                            }
                            self.previous_drag = *pos;
                        }
                    }
                    StateCanvas::Drag => {
                        let delta = *pos - self.drag_start;
                        self.drag_start = *pos;
                        self.pos += delta;
                    }
                },
                _ => {}
            },
        }
    }
}
