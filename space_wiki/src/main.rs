mod types;

use std::collections::HashMap;
use std::io::stdout;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::Paragraph;
use crate::types::ArticleType::Engine;

fn cleanup(){
    execute!(stdout(),LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
fn main() {
    std::panic::set_hook(Box::new(|_| cleanup()));
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let flat_fields = HashMap::new();
    let field_order = Vec::<String>::new();
    let infobox = types::Infobox{
        flat_fields,
        field_order,
    };

    let block = vec![
        types::Block::Heading { level: 1, text: String::from("RS-25") },
        types::Block::Paragraph { segments: vec![types::Segment::Text(String::from("The RS-25, also known as the Space Shuttle Main Engine (SSME), is a liquid-fuel rocket engine that was used on the Space Shuttle and is now being repurposed for NASA's Space Launch System (SLS). It is a cryogenic engine that burns liquid hydrogen and liquid oxygen, and it is known for its high performance and reliability. The RS-25 has a long history of successful missions and continues to be an important part of NASA's space exploration efforts."))] },
    ];


    let document = types::Document{
      blocks: block,
    };

    let article = types::Article{
        title: String::from("RS-25"),
        article_type: Engine(types::EngineSubtype::Cryogenic),
        infobox,
        body: document,


    };

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
