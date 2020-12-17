use std::process::Command;
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
    let text = String::from_utf8_lossy(&output.stdout[..]);
    let para = Paragraph::new(&text[..])
        .block(Block::default().title(title).borders(Borders::ALL))
        .wrap(Wrap {trim: true });
    f.render_widget(para, area);
}


fn main() -> Result<(), io::Error> {
    let git_branches = git_cmd("branch", [].as_ref());
    let git_logs = git_cmd("log", [].as_ref());
    let git_tags = git_cmd("tag", ["-n"].as_ref());
    let git_stash = git_cmd("stash", [].as_ref());
    let git_remotes = git_cmd("remote", ["-v"].as_ref());

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40)
                ].as_ref()
            )
            .split(f.size());

        let chunks2 = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(chunks[2]);

        draw_block(git_branches, "branch", f, chunks[0]);
        draw_block(git_tags, "tag", f, chunks2[1]);
        draw_block(git_logs, "log", f, chunks[1]);
        draw_block(git_stash, "stash", f, chunks2[0]);
    })
}
