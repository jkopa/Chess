use eframe::{egui::CentralPanel, epi::App, run_native, NativeOptions};

struct Game;

impl App for Game {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Game");
        });
    }

    fn name(&self) -> &str {
        return "Chess";
    }
}

pub fn run() {
    println!("Hi, welcome to chess");
    let app = Game;
    let win_options = NativeOptions::default();

    run_native(Box::new(app), win_options);
    //loop {}
}
