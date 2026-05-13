use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use crate::app::{type_display, App};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap};
use crate::parser::segments_from_str;
use crate::types::{ArticleType, Block as BodyBlock, Segment, SidebarEntry};

pub fn draw(frame: &mut Frame, app: &App){
    const MIN_WIDTH: u16 = 80;
    const MIN_HEIGHT: u16 = 24;
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT{
        let warning = Paragraph::new(Text::styled("Terminal too small! Please resize to at least 80x24.", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
        frame.render_widget(warning, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(27), Constraint::Min(0)])
        .split(area);

    let sidebar_block = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));
    let sidebar_inner = sidebar_block.inner(chunks[0]);
    frame.render_widget(sidebar_block, chunks[0]);
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0), Constraint::Length(3)])
        .split(sidebar_inner);


    let sidebar_title = Paragraph::new(Text::styled(" ARTICLE LIST", Style::default().fg(Color::DarkGray)))
        .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray)));
    frame.render_widget(sidebar_title, sidebar_chunks[0]);

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
                let text = if is_current { format!("  ▶ {title}")} else {format!("    {title}")};
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
    frame.render_widget(sidebar, sidebar_chunks[1]);

    let sidebar_footer = Paragraph::new(vec![
        Line::styled("(j/k) scroll (enter) sel", Style::default().fg(Color::Rgb(60,60,60))),
        Line::styled("( [/] ) prev/next type", Style::default().fg(Color::Rgb(60,60,60))),
    ]).block(Block::default().borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray)));
    frame.render_widget(sidebar_footer, sidebar_chunks[2]);



    let field_count = app.current_article
        .as_ref()
        .and_then(|k| app.loaded_articles.get(k))
        .map(|a| a.infobox.fields.iter().filter(|(_, v)| !v.is_empty()).count())
        .unwrap_or(0);
    const COL_WIDTH: u16 = 20;
    let cols = ((chunks[1].width.saturating_sub(2))/COL_WIDTH) as usize;

    let rows =  (field_count + cols-1)/cols;

    let infobox_height = (rows * 2 + 2) as u16;


    const SPLASH_ART: &str = r#"███████╗██████╗  █████╗  ██████╗███████╗    ██╗    ██╗██╗██╗  ██╗██╗
    ██╔════╝██╔══██╗██╔══██╗██╔════╝██╔════╝    ██║    ██║██║██║ ██╔╝██║
    ███████╗██████╔╝███████║██║     █████╗      ██║ █╗ ██║██║█████╔╝ ██║
    ╚════██║██╔═══╝ ██╔══██║██║     ██╔══╝      ██║███╗██║██║██╔═██╗ ██║
    ███████║██║     ██║  ██║╚██████╗███████╗    ╚███╔███╔╝██║██║  ██╗██║
    ╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝╚══════╝     ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚═╝"#;
    let main_block = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));
    let main_inner = main_block.inner(chunks[1]);
    frame.render_widget(main_block, chunks[1]);
    
    if app.current_article.is_none(){
        let splash_screen_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(main_inner);
        let logo_lines: Vec<Line> = SPLASH_ART
        .lines()
        .map(|l| {
            let cleaned = l.trim_matches(|c: char| c.is_whitespace()); 
            Line::from(Span::raw(cleaned))
        })
        .collect();
        let splash = Paragraph::new(logo_lines)
        .style(Style::default().fg(Color::Rgb(100, 160, 225)))
        .alignment(Alignment::Center);
        frame.render_widget(splash, splash_screen_chunk[0]);
        let splash_intro = Paragraph::new(vec![
            Line::styled("A terminal-based wiki for rockets, spacecraft, and engines.", Style::default().fg(Color::Gray)),
            Line::raw(""),
            Line::styled("Select an article from the sidebar with (enter) and (j/k) to begin.", Style::default().fg(Color::Rgb(60,60,60))),
        ]).alignment(Alignment::Center);
        frame.render_widget(splash_intro, splash_screen_chunk[1]);
    }
    else if let Some(key) = &app.current_article{
        if let Some(article) = app.loaded_articles.get(key){
            let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Length(infobox_height), Constraint::Min(0)])
            .split(main_inner);
            let (t, st) = type_display_pretty(&article.article_type);
            let title_line = Line::from(vec![
                Span::styled(format!(" {}", &article.title), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(format!("{t} · {st}"), Style::default().fg(Color::DarkGray)),
            ]);
            let par = Paragraph::new(title_line).block(Block::default().borders(Borders::BOTTOM));
            frame.render_widget(par, main_chunks[0]);
            let infobox_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .style(Style::default().bg(Color::Rgb(22, 22, 22)));
            let infobox_inner = infobox_block.inner(main_chunks[1]);
            frame.render_widget(infobox_block, main_chunks[1]);
            let infobox_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(std::iter::repeat(Constraint::Fill(1)).take(cols).collect::<Vec<_>>())
                .split(infobox_inner);

            let mut link_counter: usize = 0;
            for col_index in 0..cols{
                let mut lines: Vec<Line> = Vec::new();

                for (i, (label, value)) in article.infobox.fields.iter().enumerate(){
                    if i % cols == col_index{
                        let mut spans: Vec<Span> =  Vec::new();
                        for seg in segments_from_str(value){
                            match seg{
                                Segment::Text(s) => spans.push(Span::styled(s, Style::default().fg(Color::Gray))),
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
                    BodyBlock::Heading { level, text } =>{
                        let style = match level{
                            1 => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                            2 => Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
                            _ => Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                        };
                        body.push(Line::raw(""));
                        body.push(Line::styled(text, style));
                        let underline_width = (main_chunks[2].width.saturating_sub(2)) as usize;
                        let underline = "─".repeat(underline_width);
                        body.push(Line::styled(underline, Style::default().fg(Color::Rgb(40,40,40))));
                    },
                    BodyBlock::Paragraph { segments } => {
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

            let body_widget = Paragraph::new(body)
                .wrap(Wrap { trim: false })
                .block(Block::default().padding(Padding::horizontal(1)));
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