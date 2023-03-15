use crate::utils::{exec_cmd, exec_git, get_screen_height, get_screen_width};
use crate::views::{menu, single_file_menu};

use cursive::traits::*;
use cursive::{
    align::HAlign,
    view::Scrollable,
    views::{Dialog, DummyView, LinearLayout, SelectView, TextView},
};

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();
    let mut title = "Gut - Git UI Tool".to_string();
    let current_screen = cs.active_screen();
    title.push_str(format!(" - [{}]", current_screen).as_str());

    let mut description = String::new();
    let (out, _, _) = exec_git(&["branch", "--show-current"]);
    description.push_str(&format!("Current branch: {}\n", out));
    let tv_description = TextView::new(&description);

    let mut unstaged = SelectView::<String>::new().with(|s| s.set_inactive_highlight(false));
    let mut staged = SelectView::<String>::new().with(|s| s.set_inactive_highlight(false));
    let unstaged_label = TextView::new("Unstaged").h_align(HAlign::Center);
    let staged_label = TextView::new("Staged").h_align(HAlign::Center);

    let (short_status, _, _) = exec_git(&["status", "--short"]);

    for line in short_status.lines() {
        let mut line_str = line.to_string();
        line_str = line_str[3..].to_string();
        let mut unstaged_item = vec![line_str.as_str()];
        let mut staged_item = vec![line_str.as_str()];

        match line.chars().nth(0).unwrap() {
            ' ' => staged_item.clear(), // Unmodified on local
            'M' => staged_item.push("Modified"),
            'T' => staged_item.push("Type changed"),
            'A' => staged_item.push("Added"),
            'D' => staged_item.push("Deleted"),
            'R' => staged_item.push("Renamed"),
            'C' => staged_item.push("Copied"),
            'U' => staged_item.push("Updated but unmerged"),
            '?' => staged_item.push("Untracked"),
            '!' => staged_item.push("Ignored"),
            _ => (),
        }

        if staged_item.len() > 0 {
            staged.add_item(staged_item.join(" - "), staged_item[0].to_string());
        }

        match line.chars().nth(1).unwrap() {
            ' ' => unstaged_item.clear(), // Unmodified on local
            'M' => unstaged_item.push("Modified"),
            'T' => unstaged_item.push("Type changed"),
            'A' => unstaged_item.push("Added"),
            'D' => unstaged_item.push("Deleted"),
            'R' => unstaged_item.push("Renamed"),
            'C' => unstaged_item.push("Copied"),
            'U' => unstaged_item.push("Updated but unmerged"),
            '?' => unstaged_item.push("Untracked"),
            '!' => unstaged_item.push("Ignored"),
            _ => (),
        }

        if unstaged_item.len() > 0 {
            unstaged.add_item(unstaged_item.join(" - "), unstaged_item[0].to_string());
        }
    }

    unstaged.set_on_submit(|s, file_path: &str| {
        single_file_menu::menu_unstaged(s, file_path);
    });

    staged.set_on_submit(|s, file_path: &str| {
        single_file_menu::menu_staged(s, file_path);
    });

    let left_section_title = exec_cmd("pwd", None).unwrap();
    let left_section = Dialog::around(
        LinearLayout::vertical()
            .child(DummyView)
            .child(tv_description)
            .child(DummyView)
            .child(unstaged_label)
            .child(unstaged)
            .child(DummyView)
            .child(staged_label)
            .child(staged),
    )
    .title(left_section_title.trim())
    .full_screen()
    .scrollable();

    let menu_items = vec![
        "PUSH",
        "PULL",
        "REBASE",
        "BRANCH",
        "COMMIT",
        "STASH",
        "CHERRY-PICK",
        "STATUS",
        "LOG",
        "QUIT",
    ];

    let menu = SelectView::new()
        .with_all_str(menu_items)
        .on_submit(menu::menu_handler)
        .with(|s| s.set_inactive_highlight(false))
        .scrollable();

    let buttons = LinearLayout::vertical().child(DummyView).child(menu);

    let right_section_title = "Menu";
    let right_section = Dialog::around(buttons)
        .title(right_section_title)
        .title_position(HAlign::Center)
        .min_size((get_screen_width(cs) / 4, get_screen_height(cs)));

    let content = LinearLayout::horizontal()
        .child(left_section)
        .child(DummyView)
        .child(right_section);

    let main_dialog = Dialog::around(content)
        .title(title)
        .h_align(HAlign::Center)
        .full_screen();

    cs.add_layer(main_dialog);
}
