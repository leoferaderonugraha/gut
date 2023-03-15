use crate::views;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        let mut cs = cursive::default();

        cs.add_global_callback('q', |s| s.quit());
        cs.add_global_callback(cursive::event::Key::Esc, |s| s.quit());
        cs.add_global_callback('r', views::main_screen);

        // enable debug
        cs.add_global_callback('d', |s| {
            s.toggle_debug_console();
        });

        views::main_screen(&mut cs);

        cs.run();
    }
}
