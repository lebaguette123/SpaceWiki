use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Style};
use crate::app::{type_display, App};
use ratatui::widgets::{List, ListItem, Paragraph};
use crate::types::{ArticleType, SidebarEntry};

pub fn draw(frame: &mut Frame, app: &App){
    const MIN_WIDTH: u16 = 80;
    const MIN_HEIGHT: u16 = 24;
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT{
        let warning = Paragraph::new("Terminal too small! Please resize to at least 80x24.");
        frame.render_widget(warning, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(area);

    let items: Vec<ListItem> = app.sidebar_entries()
        .iter()
        .enumerate()
        .map(|(i, entry)| match entry{
            SidebarEntry::TypeHeading(t_heading) =>
                ListItem::new(t_heading.clone())
                .style(Style::default().fg(Color::DarkGray)),
            SidebarEntry::SubtypeHeading(st_heading) =>
                ListItem::new(format!("  {}", st_heading))
                .style(Style::default().fg(Color::DarkGray)),
            SidebarEntry::Article { title, key} => {
                let is_current = app.current_article.as_deref() == Some(key.as_str());
                let is_selected = i == app.selected_sidebar_index;
                let text = if is_current { format!("▶ {title}")} else {format!("  {title}")};
                if is_selected{
                    ListItem::new(text).style(Style::default().fg(Color::Black).bg(Color::Gray))
                }
                else{
                    ListItem::new(text).style(Style::default().fg(Color::Gray))
                }
            }
        })
        .collect();

    let sidebar = List::new(items);
    frame.render_widget(sidebar, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(10), Constraint::Min(0)])
        .split(chunks[1]);

    if let Some(key) = &app.current_article{
        if let Some(article) = app.loaded_articles.get(key){
            let (t, st) = type_display_pretty(&article.article_type);
            let par = Paragraph::new(format!("{}  {} · {} ", article.title, t, st));
            frame.render_widget(par, main_chunks[0]);
        }
    }
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let lowercase = word.to_lowercase();
            let singular = if lowercase.ends_with('s') && lowercase.len() > 1 {
                &lowercase[..lowercase.len() - 1]
            } else {
                &lowercase
            };
            let mut chars = singular.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().to_string() + chars.as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
fn type_display_pretty(t: &ArticleType) -> (String, String){
    let (type_str, subtype_str) = type_display(t);
    (to_title_case(type_str), to_title_case(subtype_str))

}