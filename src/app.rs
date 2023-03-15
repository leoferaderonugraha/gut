use crate::views;

pub struct App {
    cs: cursive::CursiveRunnable,
}

impl App {
    pub fn new() -> Self {
        Self {
            cs: cursive::default(),
        }
    }

    pub fn run(&mut self) {
        // Initiate 2 screens for the app
        self.cs.add_screen();

        self.cs
            .add_global_callback(cursive::event::Key::Esc, |s| s.quit());
        self.cs.add_global_callback('q', |s| s.quit());
        self.cs.add_global_callback('r', views::main::main_screen);
        self.cs
            .add_global_callback('n', |s| s.set_screen(s.active_screen() + 1));
        self.cs
            .add_global_callback('p', |s| s.set_screen(s.active_screen() - 1));

        // enable debug
        self.cs.add_global_callback('d', |s| {
            s.toggle_debug_console();
        });

        self.set_theme("base16-eighties.dark");

        views::main::main_screen(&mut self.cs);

        self.cs.run();
    }

    fn set_theme(&mut self, name: &str) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = &theme_set.themes[name];

        self.cs.with_theme(|t| {
            if let Some(background) = theme
                .settings
                .background
                .map(cursive_syntect::translate_color)
            {
                t.palette[cursive::theme::PaletteColor::Background] = background;
                t.palette[cursive::theme::PaletteColor::View] = background;
            }
            if let Some(foreground) = theme
                .settings
                .foreground
                .map(cursive_syntect::translate_color)
            {
                t.palette[cursive::theme::PaletteColor::Primary] = foreground;
                t.palette[cursive::theme::PaletteColor::TitlePrimary] = foreground;
            }

            if let Some(highlight) = theme
                .settings
                .highlight
                .map(cursive_syntect::translate_color)
            {
                t.palette[cursive::theme::PaletteColor::Highlight] = highlight;
            }
        });
    }
}
