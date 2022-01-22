mod search_bar;
mod jq;

use std::io::{self, Read, Write};
use std::thread;
use std::time;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use crate::jq::run;
use search_bar::SearchBar;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut input).unwrap();

    let mut keys = termion::async_stdin().keys();

    let stdout = io::stdout().into_raw_mode()?;
    let mut stdout = AlternateScreen::from(stdout);

    let mut search_bar = SearchBar::new();

    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), search_bar,)?;
    stdout.flush()?;

    let mut result = String::new();
    loop {
        let key = keys.next();
        if let Some(key) = key {
            match key.unwrap() {
                Key::Char('\n') => break,
                Key::Char(c) => search_bar.insert(c),
                Key::Backspace => search_bar.backspace(),
                Key::Delete => search_bar.delete(),
                Key::Left => search_bar.move_cursor_left(),
                Key::Right => search_bar.move_cursor_right(),
                // Key::Tab => search_bar.completion(),
                // Key::Ctrl('d')
                Key::Esc => break,
                _ => continue,
            };

            let (width, height) = termion::terminal_size().unwrap();
            if let Ok(res) = run(&search_bar.query(), &input) {
                result = res
                    .split('\n')
                    .take((height - 1) as usize)
                    .map(|s| truncate(s, width as usize))
                    .collect::<Vec<String>>()
                    .join("\n\r");
            }

            write!(
                stdout,
                "{}{}{}{}{}{}{}",
                termion::clear::All,
                termion::style::Reset,
                termion::cursor::Goto(1, 1),
                search_bar,
                termion::cursor::Goto(1, 2),
                result,
                termion::cursor::Goto((search_bar.cursor() + 1) as u16, 1),
            )?;
            stdout.flush()?;
        }
        thread::sleep(time::Duration::from_millis(20));
    }

    write!(stdout, "{}", termion::clear::All)?;
    stdout.flush()?;

    drop(stdout);
    println!("{}", search_bar.query());
    Ok(())
}

fn truncate(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_string(),
        Some((idx, _)) => s[..idx].to_string(),
    }
}
