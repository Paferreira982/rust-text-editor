use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {

        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);

        if self.should_quit {
            Terminal::clear_screen();
            println!("Closing the application.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        if let Key::Ctrl('q') = Terminal::read_key()? {
            self.should_quit = true;
        }

        Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_wc_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_wc_message(&self) {
        let mut wc_message = format!("Rust Vi editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let padding = width.saturating_sub(wc_message.len()) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        wc_message = format!("~{}{}", spaces, wc_message);
        wc_message.truncate(width);
        println!("{}\r", wc_message);
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}