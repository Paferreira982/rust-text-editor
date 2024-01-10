use crate::{Position, Theme};
use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub enum TerminalStruct {
    StatusBarBgColor,
    StatusBarFgColor,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size =  termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position{x, y} = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x,y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_color(terminal_struct: TerminalStruct, theme: &Theme) {
        let color = match terminal_struct {
            TerminalStruct::StatusBarBgColor => &theme.colors.background_status_bar,
            _ => &theme.colors.background,
        };

        print!("{}", color::Bg(color::Rgb(color.r, color.g, color.b)));
    }

    pub fn reset_bg_color(theme: &Theme) {
        print!("{}", color::Bg(theme.bg_reset()));
    }

    pub fn set_fg_color(terminal_struct: TerminalStruct, theme: &Theme) {
        let color = match terminal_struct {
            TerminalStruct::StatusBarFgColor => &theme.colors.text_status_bar,
            _ => &theme.colors.texts,
        };

        print!("{}", color::Fg(color::Rgb(color.r, color.g, color.b)));
    }

    pub fn reset_fg_color(theme: &Theme) {
        print!("{}", color::Fg(theme.fg_reset()));
    }
}