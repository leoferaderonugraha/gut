use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    traits::With,
    view::{scroll::Scroller, Scrollable},
    views::{Button, Dialog, LinearLayout, OnEventView, Panel, TextView},
};

use cursive::traits::*;

fn get_output(cmd: &[&str]) -> String {
    let pipe = std::process::Command::new("git")
        .args(cmd)
        .output()
        .expect("Failed to run git");

    let output = String::from_utf8(pipe.stdout).unwrap();

    return output;
}

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let description = "Prototype of git wrapper";
    let tv_description = TextView::new(description).fixed_size((50, 10));

    let btn_git_test = Button::new("Branches", git_branches);
    let btn_git_status = Button::new("Status", git_status);
    let btn_quit = Button::new("Quit", |s| s.quit());

    let buttons = LinearLayout::vertical()
        .child(btn_git_test)
        .child(btn_git_status)
        .child(btn_quit);

    let dlg_left = Dialog::around(tv_description.fixed_size((50, 10)));
    let dlg_right = Dialog::around(buttons.fixed_size((50, 10)));

    let main_dialog = Dialog::around(LinearLayout::horizontal().child(dlg_left).child(dlg_right))
        .title("Main Menu");

    cs.add_layer(main_dialog);
}

fn git_branches(cs: &mut cursive::Cursive) {
    cs.pop_layer();
    let title = "Git Branches";

    let mut output = get_output(&["branch", "-a"]);

    if output.trim().is_empty() {
        output = "No branches found".to_string();
    }

    let tv_output = TextView::new(output).scrollable();

    let dialog = Dialog::around(Panel::new(tv_output))
        .title(title)
        .h_align(HAlign::Center)
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        });

    cs.add_layer(dialog);
}

fn git_status(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Status";
    let output = get_output(&["status"]);

    let text_view = TextView::new(&output)
        .scrollable()
        .wrap_with(OnEventView::new)
        // Enable page up/down
        .on_pre_event_inner(Key::PageUp, |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_up() {
                scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
            }
            return Some(EventResult::Consumed(None));
        })
        .on_pre_event_inner(Key::PageDown, |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_down() {
                scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
            }

            return Some(EventResult::Consumed(None));
        });

    let dialog = Dialog::around(Panel::new(text_view))
        .title(title)
        .h_align(HAlign::Center)
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        });

    cs.add_layer(dialog);
}
