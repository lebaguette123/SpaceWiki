pub mod types;
pub mod app;
pub mod article;
pub mod parser;
pub mod ui;
pub mod link;

use std::io::stdout;
use std::path::PathBuf;
use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use crossterm::event::KeyEventKind::{Press, Repeat};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;
use crate::types::SidebarEntry;

fn cleanup(){
    execute!(stdout(),LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
pub fn run(){
    let article_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("articles");
    let mut app = App::new(&article_path).unwrap();

    std::panic::set_hook(Box::new(|_| cleanup()));
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    loop{
        terminal.draw(|frame| ui::draw(frame, &mut app)).unwrap();
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => match key.code{
                KeyCode::Up | KeyCode::Char('k') if key.kind == Press || key.kind == Repeat => {
                    app.move_sidebar_up();
                },
                KeyCode::Down | KeyCode::Char('j') if key.kind == Press || key.kind == Repeat => {
                    app.move_sidebar_down();
                },
                KeyCode::Enter =>{
                    match app.focused_link{
                        Some(_) => app.follow_focused_link(),
                        None => {
                            let art = &app.sidebar_entries()[app.selected_sidebar_index];
                            match art{
                                SidebarEntry::Article{ key, .. } => app.open_article(key.as_str()).unwrap(),
                                _ => ()
                            }
                        }
                    }
                },
                KeyCode::Backspace if key.kind == Press =>{
                    app.go_back();
                },
                KeyCode::Char('q') => {
                    cleanup();
                    break;
                },
                KeyCode::Left | KeyCode::Char('h') if key.kind == Press || key.kind == Repeat => {
                    app.cycle_link_prev();
                },
                KeyCode::Right | KeyCode::Char('l') if key.kind == Press || key.kind == Repeat => {
                    app.cycle_link_next();
                },
                KeyCode::Char('[') if key.kind == Press => app.jump_to_prev_type_heading(),
                KeyCode::Char(']') if key.kind == Press => app.jump_to_next_type_heading(),
                KeyCode::Char('i') if key.kind == Press => app.toggle_infobox(),
                KeyCode::Esc if key.kind == Press => app.focused_link = None,
                KeyCode::PageUp if key.kind == Press || key.kind == Repeat => app.scroll_up(app.page_scroll),
                KeyCode::PageDown if key.kind == Press || key.kind == Repeat => app.scroll_down(app.page_scroll),
                _ => ()
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollUp => app.scroll_up(3),
                    MouseEventKind::ScrollDown => app.scroll_down(3),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}