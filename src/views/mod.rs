use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    traits::With,
    view::{scroll::Scroller, Scrollable},
    views::{Dialog, DummyView, LinearLayout, OnEventView, Panel, SelectView, TextView},
};

use crate::utils::pad_str;
use cursive::traits::*;

const SCREEN_WIDTH: usize = 100;
const SCREEN_HEIGHT: usize = 30;
const SCREEN_SIZE: (usize, usize) = (SCREEN_WIDTH, SCREEN_HEIGHT);
const PAD_WIDTH: usize = 10;

enum Menu {
    Branch,
    Stash,
    CherryPick,
    Status,
    Log,
    Quit,
}

fn menu_handler(cs: &mut cursive::Cursive, item: &Menu) {
    match item {
        Menu::Branch => git_branches(cs),
        Menu::Stash => not_implemented(cs),
        Menu::CherryPick => not_implemented(cs),
        Menu::Status => git_status(cs),
        Menu::Log => git_log(cs),
        Menu::Quit => cs.quit(),
    }
}

fn get_output(cmd: &[&str]) -> String {
    let pipe = std::process::Command::new("git")
        .args(cmd)
        .output()
        .expect("Failed to run git");

    let output = String::from_utf8(pipe.stdout).unwrap();

    return output;
}

fn switch_branch(s: &mut cursive::Cursive, branch: &str) {
    let mut target_branch = branch.to_string();

    if target_branch.starts_with('*') || target_branch.starts_with(' ') {
        target_branch = target_branch[2..].to_string();
    }

    let cmd = vec!["checkout", target_branch.as_str()];
    let output = get_output(&cmd);

    let dialog = Dialog::text(output)
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}

fn not_implemented(s: &mut cursive::Cursive) {
    let dialog = Dialog::text("Not implemented yet")
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();
    let title = "Gut - Git UI Tool";

    let mut description = String::new();
    let current_branch = get_output(&["branch", "--show-current"]);

    description.push_str(&format!("Current branch: {}\n", current_branch));

    // let description = get_output(&["log", "-1", "--pretty=%B"]);
    description.push_str(get_output(&["status", "-su"]).as_str());

    let tv_description = TextView::new(&description).scrollable();

    let mut menu = SelectView::<Menu>::new().on_submit(&menu_handler);

    menu.add_item(pad_str("BRANCH", PAD_WIDTH), Menu::Branch);
    menu.add_item(pad_str("STASH", PAD_WIDTH), Menu::Stash);
    menu.add_item(pad_str("CHERRY-PICK", PAD_WIDTH), Menu::CherryPick);
    menu.add_item(pad_str("STATUS", PAD_WIDTH), Menu::Status);
    menu.add_item(pad_str("LOG", PAD_WIDTH), Menu::Log);
    menu.add_item(pad_str("QUIT", PAD_WIDTH), Menu::Quit);

    let buttons = LinearLayout::vertical().child(DummyView).child(menu);

    let dlg_left_title = get_output(&["rev-parse", "--show-toplevel"]);
    let dlg_left = Dialog::around(tv_description)
        .title(dlg_left_title.trim())
        .full_width();
    let dlg_right_title = "Menu";
    let dlg_right = Dialog::around(buttons)
        .title(dlg_right_title)
        .title_position(HAlign::Center);

    let content = LinearLayout::horizontal()
        .child(dlg_left)
        .child(DummyView)
        .child(dlg_right);

    let main_dialog = Dialog::around(content)
        .title(title)
        .h_align(HAlign::Center)
        .fixed_size(SCREEN_SIZE);

    cs.add_layer(main_dialog);
}

fn git_branches(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Branches";
    let cmd = vec!["branch", "-l"];
    let mut output = get_output(&cmd);

    if output.trim().is_empty() {
        output = "No branch(es) found".to_string();
    }

    let select = SelectView::<String>::new()
        .with(|s| {
            for line in output.lines() {
                s.add_item(line, line.to_string());
            }
        })
        .on_submit(switch_branch);

    let dialog = Dialog::around(select)
        .title(title)
        .button("Back", main_screen)
        .fixed_size(SCREEN_SIZE);

    cs.add_layer(dialog);
}

fn git_status(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Status";
    let cmd = vec!["status"];
    let output = get_output(&cmd);

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
        .button("Back", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size(SCREEN_SIZE);

    cs.add_layer(dialog);
}

fn git_log(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Log";
    let cmd = vec!["log", "--oneline", "--decorate", "--graph"];
    let output = get_output(&cmd);

    let tv_output = TextView::new(output).scrollable();

    let dialog = Dialog::around(Panel::new(tv_output))
        .title(title)
        .h_align(HAlign::Center)
        .button("Back", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size(SCREEN_SIZE);

    cs.add_layer(dialog);
}
