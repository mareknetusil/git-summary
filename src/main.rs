use std::process::Command;
// TUI
use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};
use tui::backend::TermionBackend;
use tui::widgets::{Wrap, Paragraph, Block, Borders};
use tui::layout::{Alignment, Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let git_branches = Command::new("git")
        .arg("branch")
        .output()
        .expect("Prikaz 'git branch' selhal");

    let git_logs = Command::new("git")
        .arg("log")
        .output()
        .expect("Prikaz 'git log' selhal");

    let git_tags = Command::new("git")
        .arg("tag")
        .output()
        .expect("Prikaz 'git tag' selhal");

    let git_stash = Command::new("git")
        .arg("stash")
        .arg("list")
        .output()
        .expect("Prikaz 'git stash' selhal");

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
            .split(chunks[0]);

        let text = String::from_utf8_lossy(&git_branches.stdout[..]);
        let para = Paragraph::new(&text[..])
            .block(Block::default().title("branch").borders(Borders::ALL))
            .wrap(Wrap {trim: true });
        f.render_widget(para, chunks2[0]);

        let text = String::from_utf8_lossy(&git_tags.stdout[..]);
        let para = Paragraph::new(&text[..])
            .block(Block::default().title("tag").borders(Borders::ALL))
            .wrap(Wrap {trim: true });
        f.render_widget(para, chunks2[1]);

        let text = String::from_utf8_lossy(&git_logs.stdout[..]);
        let para = Paragraph::new(&text[..])
            .block(Block::default().title("log").borders(Borders::ALL))
            .wrap(Wrap {trim: true });
        f.render_widget(para, chunks[1]);

        let text = String::from_utf8_lossy(&git_stash.stdout[..]);
        let para = Paragraph::new(&text[..])
            .block(Block::default().title("stash").borders(Borders::ALL))
            .wrap(Wrap {trim: true });
        f.render_widget(para, chunks[2]);
    })
}
