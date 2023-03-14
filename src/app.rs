use crate::views;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        let mut cs = cursive::default();

        cs.add_global_callback('q', |s| s.quit());
        views::main_screen(&mut cs);

        cs.run();
    }
}
