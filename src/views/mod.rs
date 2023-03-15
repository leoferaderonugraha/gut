use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    traits::With,
    view::{scroll::Scroller, Scrollable},
    views::{Dialog, DummyView, LinearLayout, OnEventView, Panel, SelectView, TextView},
};

use crate::utils::{alert_popup, get_output, not_implemented, pad_str};
use cursive::traits::*;

const SCREEN_WIDTH: usize = 100;
const SCREEN_HEIGHT: usize = 30;
const SCREEN_SIZE: (usize, usize) = (SCREEN_WIDTH, SCREEN_HEIGHT);
const PAD_WIDTH: usize = 10;

enum Menu {
    Branch,
    Status,
    Log,
    Quit,
    NotImplemented,
}

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();
    let title = "Gut - Git UI Tool";

    let mut description = String::new();
    let current_branch = get_output(&["branch", "--show-current"]);
    description.push_str(&format!("Current branch: {}\n", current_branch));
    let tv_description = TextView::new(&description).scrollable();

    let mut desc_status = SelectView::<String>::new();

    for line in get_output(&["status", "--short"]).lines() {
        let mut line_str = line.to_string();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        match parts[0] {
            "M" => line_str.push_str(" - Modified"),
            "T" => line_str.push_str(" - Type changed"),
            "A" => line_str.push_str(" - Added"),
            "D" => line_str.push_str(" - Deleted"),
            "R" => line_str.push_str(" - Renamed"),
            "C" => line_str.push_str(" - Copied"),
            "U" => line_str.push_str(" - Updated but unmerged"),
            "??" => line_str.push_str(" - Untracked"),
            category => line_str.push_str(&format!(" - Unknown category: {}", category)),
        }

        desc_status.add_item(line_str[3..].to_string(), parts[1].to_string());
    }

    desc_status.set_on_submit(|s, file_path: &str| {
        let single_file_menu = SelectView::new()
            .with_all_str(vec![
                "ADD",
                "REMOVE",
                "BLAME",
                "DIFF",
                "CHECKOUT",
                "RESET",
                "REVERT",
                "LOG",
                "MERGE CONFLICT",
            ])
            .on_submit(single_file_handler)
            .scrollable();

        let layout = LinearLayout::vertical()
            .child(DummyView)
            .child(TextView::new(format!("File: {}", file_path)))
            .child(DummyView)
            .child(single_file_menu);

        let blame_dialog = Dialog::around(layout)
            .title("Actions")
            .button("Close", |s| {
                s.pop_layer();
            })
            .h_align(HAlign::Center)
            .fixed_size((SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2));
        s.add_layer(blame_dialog);
    });

    let left_section_title = exec_cmd("pwd", None).unwrap();
    let left_section = Dialog::around(
        LinearLayout::vertical()
            .child(DummyView)
            .child(tv_description)
            .child(desc_status),
    )
    .title(left_section_title.trim())
    .full_width();

    let mut menu = SelectView::<Menu>::new().on_submit(&menu_handler);

    menu.add_item(pad_str("PUSH", PAD_WIDTH), Menu::NotImplemented);
    menu.add_item(pad_str("PULL", PAD_WIDTH), Menu::NotImplemented);
    menu.add_item(pad_str("BRANCH", PAD_WIDTH), Menu::Branch);
    menu.add_item(pad_str("COMMIT", PAD_WIDTH), Menu::Branch);
    menu.add_item(pad_str("STASH", PAD_WIDTH), Menu::NotImplemented);
    menu.add_item(pad_str("CHERRY-PICK", PAD_WIDTH), Menu::NotImplemented);
    menu.add_item(pad_str("STATUS", PAD_WIDTH), Menu::Status);
    menu.add_item(pad_str("LOG", PAD_WIDTH), Menu::Log);
    menu.add_item(pad_str("QUIT", PAD_WIDTH), Menu::Quit);

    let buttons = LinearLayout::vertical().child(DummyView).child(menu);

    let right_section_title = "Menu";
    let right_section = Dialog::around(buttons)
        .title(right_section_title)
        .title_position(HAlign::Center);

    let content = LinearLayout::horizontal()
        .child(left_section)
        .child(DummyView)
        .child(right_section);

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

fn menu_handler(cs: &mut cursive::Cursive, item: &Menu) {
    match item {
        Menu::Branch => git_branches(cs),
        Menu::Status => git_status(cs),
        Menu::Log => git_log(cs),
        Menu::Quit => cs.quit(),
        Menu::NotImplemented => not_implemented(cs),
    }
}

fn single_file_handler(cs: &mut cursive::Cursive, item: &str) {
    match item {
        "PUSH" => not_implemented(cs),
        "PULL" => not_implemented(cs),
        "BRANCH" => not_implemented(cs),
        "COMMIT" => not_implemented(cs),
        "STASH" => not_implemented(cs),
        "CHERRY-PICK" => not_implemented(cs),
        "STATUS" => not_implemented(cs),
        "LOG" => not_implemented(cs),
        "QUIT" => cs.quit(),
        _ => (),
    }
}

fn exec_cmd(cmd: &str, args: Option<&[&str]>) -> Result<String, Box<dyn std::error::Error>> {
    let pipe = std::process::Command::new(cmd)
        .args(args.unwrap_or(&[]))
        .output()
        .expect("Failed to run command");

    let output = String::from_utf8(pipe.stdout).unwrap();

    return Ok(output);
}

fn switch_branch(s: &mut cursive::Cursive, branch: &str) {
    let mut target_branch = branch.to_string();

    if target_branch.starts_with('*') || target_branch.starts_with(' ') {
        target_branch = target_branch[2..].to_string();
    }

    let cmd = vec!["checkout", target_branch.as_str()];

    alert_popup(s, &cmd.join("|"));

    let output = get_output(&cmd);

    let dialog = Dialog::text(output)
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}
