use cursive::traits::Resizable;
use cursive::views::Dialog;

pub fn pad_str(s: &str, width: usize) -> String {
    return format!("{:^width$}", s, width = width);
}

pub fn alert_popup(s: &mut cursive::Cursive, data: &str) {
    let dialog = Dialog::text(data)
        .button("Close", |s| {
            s.pop_layer();
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}

pub fn not_implemented(s: &mut cursive::Cursive) {
    alert_popup(s, "Not implemented yet!");
}

pub fn get_output(cmd: &[&str]) -> String {
    let pipe = std::process::Command::new("git")
        .args(cmd)
        .output()
        .expect("Failed to run git");

    let mut merged_output = Vec::new();

    merged_output.extend(pipe.stderr);
    merged_output.extend(pipe.stdout);

    let output = String::from_utf8(merged_output).unwrap();

    return output;
}
