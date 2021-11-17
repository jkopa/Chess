pub mod graphics {
    use eframe::{egui::CentralPanel, epi::App, run_native, NativeOptions};

    struct Game;

    enum _GameState {
        Active,
        BlackWin,
        WhiteWin,
        Forfeit,
        Stalemate,
        Resignation,
    }

    impl App for Game {
        // Window contents
        fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Game");
            });
        }

        // Window title
        fn name(&self) -> &str {
            return "Chess";
        }
    }

    pub fn run_app() {
        let win_options = NativeOptions::default();
        run_native(Box::new(Game), win_options);
    }
}
