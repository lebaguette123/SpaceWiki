mod types;
mod app;
mod article;
mod parser;

use std::io::stdout;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::Paragraph;
use crate::app::App;

fn cleanup(){
    execute!(stdout(),LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn main() {
    let mut app = App::new().unwrap();
    app.open_article("rs-25").unwrap();

    std::panic::set_hook(Box::new(|_| cleanup()));
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    loop{
        terminal.draw(|frame| {
            let area = frame.area();
            let paragraph = Paragraph::new("SpaceWiki - press q to quit");
            frame.render_widget(paragraph, area);
        }).unwrap();
       if let Ok(Event::Key(key)) = event::read() {
           if key.code == KeyCode::Char('q') {
               cleanup();
               break;
           }
       }
    }


}
