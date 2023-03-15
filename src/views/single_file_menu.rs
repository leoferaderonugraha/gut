use crate::popups::popup;
use crate::utils::{exec_git, get_screen_height, get_screen_width};

use cursive::traits::*;
use cursive::{
    align::HAlign,
    traits::With,
    view::Scrollable,
    views::{Dialog, DummyView, LinearLayout, SelectView, TextView},
};

pub fn menu_staged(cs: &mut cursive::Cursive, file_path: &str) {
    let menu_items = vec![
        ("RESET", "reset"),
        ("REMOVE", "remove"),
        ("DIFF", "diff"),
        ("LOG", "log"),
        ("BLAME", "blame"),
    ];

    menu(cs, file_path, menu_items);
}

pub fn menu_unstaged(cs: &mut cursive::Cursive, file_path: &str) {
    let menu_items = vec![
        ("ADD", "add"),
        ("REMOVE", "remove"),
        ("DIFF", "diff"),
        ("LOG", "log"),
        ("BLAME", "blame"),
    ];

    menu(cs, file_path, menu_items);
}

fn handler(cs: &mut cursive::Cursive, item: &(String, String)) {
    match item.0.to_lowercase().as_str() {
        "add" => git_add(cs, item.1.as_str()),
        "reset" => git_reset(cs, item.1.as_str()),
        "diff" => git_diff(cs, item.1.as_str()),
        "log" => git_log(cs, item.1.as_str()),
        _ => popup::not_implemented(cs),
    }
}

fn menu(cs: &mut cursive::Cursive, file_path: &str, menus: Vec<(&str, &str)>) {
    let mut menu_items = Vec::new();
    for menu in menus {
        menu_items.push((
            menu.0.to_string(),
            (menu.1.to_string(), file_path.to_string()),
        ));
    }

    let single_menu = SelectView::new()
        .with_all(menu_items)
        .on_submit(handler)
        .with(|s| s.set_inactive_highlight(false))
        .scrollable();

    let layout = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new(format!("File: {}", file_path)))
        .child(DummyView)
        .child(single_menu);

    let action_dlg = Dialog::around(layout)
        .title("Actions")
        .button("Close", |s| {
            s.pop_layer();
        })
        .h_align(HAlign::Center)
        .fixed_size((get_screen_width(cs) / 2, get_screen_height(cs) / 2));

    cs.add_layer(action_dlg);
}

fn git_add(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (_, err, code) = exec_git(&["add", file_path]);

    if code == 0 {
        popup::alert_back(cs, "Success!");
    } else {
        popup::alert_back(cs, &err);
    }
}

fn git_reset(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (_, err, code) = exec_git(&["reset", file_path]);

    if code == 0 {
        popup::alert_back(cs, "Success!");
    } else {
        popup::alert_back(cs, &err);
    }
}

fn git_diff(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (mut out, err, code) = exec_git(&["diff", file_path]);

    if code == 0 {
        if out.trim().is_empty() {
            out = "No changes".to_string();
        }
        popup::new_page(cs, &out, format!("Diff - [{}]", file_path).as_str());
    } else {
        popup::new_page(cs, &err, format!("Diff - [{}]", file_path).as_str());
    }
}

fn git_log(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (out, err, code) = exec_git(&["log", "--follow", file_path]);

    if code == 0 {
        popup::new_page(cs, &out, format!("Log - [{}]", file_path).as_str());
    } else {
        popup::new_page(cs, &err, format!("Log - [{}]", file_path).as_str());
    }
}
