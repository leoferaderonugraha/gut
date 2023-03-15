use crate::popups::{alert, not_implemented};
use crate::utils::exec_git;
use crate::views::main::main_screen;

use cursive::traits::*;
use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    traits::With,
    view::{scroll::Scroller, Scrollable},
    views::{Dialog, OnEventView, Panel, SelectView, TextView},
};

pub fn menu_handler(cs: &mut cursive::Cursive, item: &str) {
    match item.to_lowercase().as_str() {
        "branch" => git_branches(cs),
        "status" => git_status(cs),
        "log" => git_log(cs),
        "quit" => cs.quit(),
        _ => not_implemented(cs),
    }
}

fn git_branches(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Branches";
    let cmd = vec!["branch", "-l"];
    let (mut out, _, _) = exec_git(&cmd);

    if out.trim().is_empty() {
        out = "No branch(es) found".to_string();
    }

    let select = SelectView::<String>::new()
        .with(|s| {
            for line in out.lines() {
                s.add_item(line, line.to_string());
            }
        })
        .on_submit(switch_branch);

    let dialog = Dialog::around(select)
        .title(title)
        .button("Back", main_screen)
        .full_screen();

    cs.add_layer(dialog);
}

fn git_status(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Status";
    let cmd = vec!["status"];
    let (out, _, _) = exec_git(&cmd);

    let text_view = TextView::new(&out)
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
        .full_screen();

    cs.add_layer(dialog);
}

fn git_log(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Log";
    let cmd = vec!["log", "--oneline", "--decorate", "--graph"];
    let (out, _, _) = exec_git(&cmd);

    let tv_output = TextView::new(out).no_wrap().scrollable().scroll_x(true);

    let dialog = Dialog::around(Panel::new(tv_output))
        .title(title)
        .h_align(HAlign::Center)
        .button("Back", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .full_screen();

    cs.add_layer(dialog);
}

fn switch_branch(s: &mut cursive::Cursive, branch: &str) {
    let mut target_branch = branch.to_string();

    if target_branch.starts_with('*') || target_branch.starts_with(' ') {
        target_branch = target_branch[2..].to_string();
    }

    let cmd = vec!["checkout", target_branch.as_str()];

    alert(s, &cmd.join("|"));

    let (out, _, _) = exec_git(&cmd);

    let dialog = Dialog::text(out)
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}
