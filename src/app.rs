use crate::canvas::Canvas;
use crate::state::State;

pub struct App {
    state: State,

    canvas: Canvas,
    show_canvas: bool,

    show_ui: bool,

    dark_mode: bool,

    dx: f32,
    dy: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::default(),

            canvas: Canvas::default(),
            show_canvas: true,

            show_ui: true,

            dark_mode: true,
            dx: 10.0,
            dy: 10.0,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}
        Default::default()
    }

    pub fn set_default(&mut self) {
        *self = Default::default();
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            state,

            canvas,
            show_canvas,

            show_ui,

            dark_mode,
            dx,
            dy,
        } = self;

        //// UPDATE APP VALUE

        // Window with a tuggle button to show or hide the UI
        egui::Window::new("UI")
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.toggle_value(show_ui, "Show UI");
            });

        if *show_ui {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // button to change the theme of the app
                if ui.button("Change theme").clicked() {
                    *dark_mode = !*dark_mode;
                    if *dark_mode {
                        ctx.set_visuals(egui::Visuals::dark());
                    } else {
                        ctx.set_visuals(egui::Visuals::light());
                    }
                }
                // button clear canvas
                if ui
                    .button("Clear canvas")
                    .on_hover_text("Clear the canvas")
                    .clicked()
                {
                    canvas.clear();
                }

                ui.label("- Choose the size of the canvas and the color you want to draw");
                ui.label("- Then click on the canvas to draw");
                ui.label("- You can moove the canvas with the arrow keys or ");
            });

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                canvas.stroke_ui(ui);
                canvas.state_ui(ui, "State canvas".to_string());
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            // Handle keyboard events
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                canvas.move_canvas(0.0, -*dy);
            }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                canvas.move_canvas(0.0, *dy);
            }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                canvas.move_canvas(-*dx, 0.0);
            }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                canvas.move_canvas(*dx, 0.0);
            }
            // Handle graph events
            let events = ui.input(|i| i.clone().events);
            for event in events.iter() {
                canvas.handle_event(event, state);
            }

            // Draw the App
            if *show_canvas {
                canvas.draw(ui);
            }
        });
    }
}
