use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use crate::app::{type_display, App};
use ratatui::widgets::{List, ListItem, Paragraph};
use crate::parser::segments_from_str;
use crate::types::{ArticleType, Block, Segment, SidebarEntry};

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
        .constraints([Constraint::Length(24), Constraint::Min(0)])
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

    let field_count = app.current_article
        .as_ref()
        .and_then(|k| app.loaded_articles.get(k))
        .map(|a| a.infobox.fields.len())
        .unwrap_or(0);
    const COL_WIDTH: u16 = 20;
    let cols = (chunks[1].width/COL_WIDTH) as usize;

    let rows =  (field_count + cols-1)/cols;

    let infobox_height = (rows * 2 + 2) as u16;


    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(infobox_height), Constraint::Min(0)])
        .split(chunks[1]);

    if let Some(key) = &app.current_article{
        if let Some(article) = app.loaded_articles.get(key){
            let (t, st) = type_display_pretty(&article.article_type);
            let par = Paragraph::new(format!("{}  {} · {} ", article.title, t, st));
            frame.render_widget(par, main_chunks[0]);
            let infobox_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(std::iter::repeat(Constraint::Fill(1)).take(cols).collect::<Vec<_>>())
                .split(main_chunks[1]);

            let mut link_counter: usize = 0;
            for col_index in 0..cols{
                let mut lines: Vec<Line> = Vec::new();

                for (i, (label, value)) in article.infobox.fields.iter().enumerate(){
                    if i % cols == col_index{
                        let mut spans: Vec<Span> =  Vec::new();
                        for seg in segments_from_str(value){
                            match seg{
                                Segment::Text(s) => spans.push(Span::styled(s, Style::default().fg(Color::DarkGray))),
                                Segment::Link{ target, display } => {
                                    let link = display.unwrap_or(target);
                                    if app.focused_link == Some(link_counter){
                                        spans.push(Span::styled(link, Style::default().fg(Color::Black).bg(Color::Cyan)))
                                    }
                                    else{
                                        spans.push(Span::styled(link, Style::default().fg(Color::Cyan)))
                                    }
                                    link_counter += 1;
                                }
                            }
                        }
                        if !value.is_empty(){
                            lines.push(Line::styled(label.replace("_", " ").to_uppercase(), Style::default().fg(Color::DarkGray)));
                            lines.push(Line::from(spans));
                        }
                    }
                }
                let par = Paragraph::new(lines);
                frame.render_widget(par, infobox_chunks[col_index]);
            }

            let mut body: Vec<Line> = Vec::new();
            let mut link_counter: usize = app.infobox_link_count;
            for block in &article.body.blocks{
                match block{
                    Block::Heading { level, text } =>{
                        let style = match level{
                            1 => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                            2 => Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
                            _ => Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                        };
                        body.push(Line::styled(text, style))
                    },
                    Block::Paragraph { segments } => {
                        let mut text: Vec<Span> = Vec::new();
                        for seg in segments.iter(){
                            match seg{
                                Segment::Text(s) => text.push(Span::styled(s, Style::default().fg(Color::DarkGray))),
                                Segment::Link{ target, display } => {
                                    let link = display.as_deref().unwrap_or(target);
                                    if app.focused_link == Some(link_counter){
                                        text.push(Span::styled(link, Style::default().fg(Color::Black).bg(Color::Cyan)))
                                    }
                                    else{
                                        text.push(Span::styled(link, Style::default().fg(Color::Cyan)))
                                    }
                                    link_counter += 1;
                                }
                            }

                        }
                        body.push(Line::from(text))
                    }
                }
            }

            let body_widget = Paragraph::new(body);
            frame.render_widget(body_widget, main_chunks[2]);
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