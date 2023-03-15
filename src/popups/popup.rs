use crate::utils::{switch_next_screen, switch_prev_screen};
use crate::views::main::main_screen;

use cursive::traits::{Resizable, Scrollable};
use cursive::views::{Dialog, Panel, TextView};

pub fn alert(s: &mut cursive::Cursive, data: &str) {
    let dialog = Dialog::text(data)
        .button("Close", |s| {
            s.pop_layer();
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}

pub fn alert_back(cs: &mut cursive::Cursive, data: &str) {
    let dialog = Dialog::text(data)
        .button("Close", |s| {
            s.pop_layer();
        })
        .fixed_size((50, 10));

    cs.add_layer(dialog);
    main_screen(cs);
}

pub fn scrollable(cs: &mut cursive::Cursive, data: &str, title: &str) {
    let tv = TextView::new(data).no_wrap().scrollable().scroll_x(true);
    let panel = Panel::new(tv).title(title);

    let dialog = Dialog::around(panel)
        .button("Close", |s| {
            s.pop_layer();
        })
        .fixed_size((70, 20));

    cs.add_layer(dialog);
}

pub fn new_page(cs: &mut cursive::Cursive, data: &str, title: &str) {
    switch_next_screen(cs);

    let tv = TextView::new(data).no_wrap().scrollable().scroll_x(true);

    let dialog = Dialog::around(tv)
        .button("Back", |s| {
            s.pop_layer();
            switch_prev_screen(s);

            // In case previous screen was cleared
            main_screen(s);
        })
        .title(title)
        .full_screen();

    cs.add_layer(dialog);
}

pub fn not_implemented(s: &mut cursive::Cursive) {
    alert(s, "Not implemented yet!");
}
