use std::process::Command;
use std::str;
// TUI
use std::io;
use termion::raw::IntoRawMode;
use tui::{Frame, Terminal};
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};
use tui::backend::{Backend, TermionBackend};
use tui::widgets::{Wrap, Paragraph, Block, Borders};
use tui::layout::{Alignment, Layout, Constraint, Direction, Rect};


fn git_cmd(cmd: &str, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .arg(cmd)
        .args(args)
        .output()
        .expect(format!("Prikaz '{} {}' selhal!", cmd, args.join(" ")).as_str())
}

fn draw_block<B>(output: std::process::Output, title: &str, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = str::from_utf8(&output.stdout[..])
        .expect("Nepodarilo se interpretovat stdout jako utf-8");
    let para = Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL))
        .wrap(Wrap {trim: true });
    f.render_widget(para, area);
}

fn split_by_percentages(pers: &[u16], dir: Direction,
                        margin: u16, area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(dir)
        .margin(margin)
        .constraints(
            pers.iter()
            .map(|&p| Constraint::Percentage(p))
            .collect::<Vec<_>>()
        )
        .split(area)
}

fn main() -> Result<(), io::Error> {
    let git_branches = git_cmd("branch", ["-a"].as_ref());
    let git_logs = git_cmd("log", [].as_ref());
    let git_tags = git_cmd("tag", ["-n"].as_ref());
    let git_stash = git_cmd("stash", ["list"].as_ref());
    let git_remotes = git_cmd("remote", ["-v"].as_ref());

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let cols = split_by_percentages(
            &[20, 40, 40], Direction::Horizontal, 1, f.size());
        let right_col = split_by_percentages(
            &[25, 25, 50], Direction::Vertical, 0, cols[2]);

        draw_block(git_branches, "branch", f, cols[0]);
        draw_block(git_logs, "log", f, cols[1]);
        draw_block(git_stash, "stash", f, right_col[0]);
        draw_block(git_remotes, "remote", f, right_col[1]);
        draw_block(git_tags, "tag", f, right_col[2]);
    })
}
