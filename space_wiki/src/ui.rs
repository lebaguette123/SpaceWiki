use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use crate::app::{type_display, App};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap};
use crate::parser::segments_from_str;
use crate::types::{ArticleType, Block as BodyBlock, Segment, SidebarEntry};

fn process_segment(
    segment: &Segment,
    focused_link: Option<usize>,
    link_counter: &mut usize,
) -> Span<'static> {
    match segment {
        Segment::Text(t) => Span::styled(t.clone(), Style::default().fg(Color::Gray)),
        Segment::Link { target, display } => {
            let current_idx = *link_counter;
            *link_counter += 1;

            let text = display.as_deref().unwrap_or(target).to_string();
            let mut style = Style::default().fg(Color::Cyan);

            if Some(current_idx) == focused_link {
                style = style.bg(Color::Cyan).fg(Color::Black);
            }
            Span::styled(text, style)
        }
    }
}

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
            let usable_width = main_inner.width.saturating_sub(2);
            let cols = (usable_width / 25).max(1) as usize;
            let stages_per_row = (usable_width / 40).max(1) as usize;
            let mut all_fields = Vec::new();
            for table in &article.infobox.subtables {
                let active_fields: Vec<_> = table.fields.iter()
                    .filter(|(_, value)| !value.trim().is_empty())
                    .collect();

                for (i, (label, value)) in active_fields.into_iter().enumerate() {
                    let section_header = if i == 0 { Some(table.title.as_str()) } else { None };
                    all_fields.push((section_header, label, value));
                }
            }
            let subtable_rows = (all_fields.len() + cols - 1) / cols;
            let subtable_h = if all_fields.is_empty() {
                0
            } else {
                (subtable_rows * 3 + 2) as u16
            };
            let stage_line_counts: Vec<usize> = article.infobox.stages.iter().map(|s| {
                let mut lines = 2;
                if !s.description.is_empty() {
                    let desc_len: usize = s.description.iter().map(|seg| match seg {
                        Segment::Text(t) => t.len(),
                        Segment::Link { display, target } => display.as_deref().unwrap_or(target).len(),
                    }).sum();
                    lines += (desc_len / 25).max(1) + 2;
                }
                if !s.engines.is_empty() { lines += 1 + s.engines.len(); }
                let prop = s.propellant.trim();
                if !prop.is_empty() && prop.to_uppercase() != "N/A" { lines += 1; }
                lines + 1
            }).collect();

            let row_heights: Vec<u16> = stage_line_counts
                .chunks(stages_per_row)
                .map(|chunk| *chunk.iter().max().unwrap_or(&0) as u16)
                .collect();

            let has_stages = !article.infobox.stages.is_empty();
            let total_stage_h = if has_stages { row_heights.iter().sum::<u16>() + 2 } else { 0 };
            let total_infobox_h = (subtable_h + total_stage_h).min(main_inner.height.saturating_sub(8));
            let main_chunks = Layout::vertical([
                Constraint::Length(2),
                Constraint::Length(total_infobox_h),
                Constraint::Min(0),
            ]).split(main_inner);
            let (t, st) = type_display_pretty(&article.article_type);
            let title_line = Line::from(vec![
                Span::styled(format!(" {}", &article.title), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(format!("{t} · {st}"), Style::default().fg(Color::DarkGray)),
            ]);
            let par = Paragraph::new(title_line).block(Block::default().borders(Borders::BOTTOM));
            frame.render_widget(par, main_chunks[0]);

            let infobox_block = Block::default()
                .borders(Borders::ALL).border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .style(Style::default().bg(Color::Rgb(20, 20, 20)));
            let mut link_counter: usize = 0;
            let infobox_inner = infobox_block.inner(main_chunks[1]);
            frame.render_widget(infobox_block, main_chunks[1]);

            if has_stages {
                let info_split = Layout::vertical([
                    Constraint::Length(subtable_h),
                    Constraint::Min(0)
                ]).split(infobox_inner);
                let sub_chunks = Layout::horizontal(vec![Constraint::Fill(1); cols]).split(info_split[0]);
                for col_idx in 0..cols {
                    let mut col_lines = Vec::new();
                    for (i, (header, label, value)) in all_fields.iter().enumerate() {
                        if i % cols == col_idx {
                            if let Some(h) = header {
                                if !col_lines.is_empty() {
                                    col_lines.push(Line::raw(""));
                                }
                                col_lines.push(Line::from(Span::styled(
                                    h.to_uppercase(),
                                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                                )));
                            } else if col_lines.is_empty() {
                                col_lines.push(Line::raw(""));
                            }
                            col_lines.push(Line::from(Span::styled(label.replace("_", " ").to_uppercase(), Style::default().fg(Color::DarkGray))));

                            // VALUE LOGIC
                            let segments = segments_from_str(value);
                            let spans: Vec<Span> = segments.iter()
                                .map(|s| process_segment(s, app.focused_link, &mut link_counter)).collect();
                            col_lines.push(Line::from(spans));
                            col_lines.push(Line::raw(""));
                        }
                    }
                    frame.render_widget(Paragraph::new(col_lines), sub_chunks[col_idx]);
                }
                let stages_block = Block::default()
                    .title(Span::styled(" STAGES ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(45, 45, 45)));
                let stages_inner = stages_block.inner(info_split[1]);
                frame.render_widget(stages_block, info_split[1]);

                let row_layout = Layout::vertical(row_heights.iter().map(|&h| Constraint::Length(h)).collect::<Vec<_>>()).split(stages_inner);
                for (r_idx, row) in article.infobox.stages.chunks(stages_per_row).enumerate() {
                    let col_layout = Layout::horizontal(vec![Constraint::Fill(1); row.len()]).split(row_layout[r_idx]);
                    for (c_idx, stage) in row.iter().enumerate() {
                        let mut s_lines = Vec::new();
                        let mut head = vec![Span::styled(format!("STAGE {}", (r_idx * stages_per_row) + c_idx + 1), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))];
                        if stage.bespoke { head.push(Span::raw("   ")); head.push(Span::styled("BESPOKE", Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD))); }
                        s_lines.push(Line::from(head));
                        s_lines.push(Line::from(process_segment(&stage.name, app.focused_link, &mut link_counter)));
                        if !stage.description.is_empty() {
                            let spans: Vec<Span> = stage.description.iter().map(|s| {
                                let mut span = process_segment(s, app.focused_link, &mut link_counter);
                                if matches!(s, Segment::Text(_)) { span.style = Style::default().fg(Color::Rgb(170, 170, 170)); }
                                span
                            }).collect();
                            s_lines.push(Line::from(spans));
                        }
                        s_lines.push(Line::raw(""));
                        if !stage.engines.is_empty() {
                            s_lines.push(Line::from(Span::styled("ENGINES", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD))));
                            for eng in &stage.engines {
                                let mut spans = vec![Span::raw(" • ")];
                                for s in eng { spans.push(process_segment(s, app.focused_link, &mut link_counter)); }
                                s_lines.push(Line::from(spans));
                            }
                        }
                        if !stage.propellant.trim().is_empty() {
                            s_lines.push(Line::from(vec![Span::styled("PROPELLANT: ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)), Span::styled(&stage.propellant, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC))]));
                        }
                        frame.render_widget(Paragraph::new(s_lines).wrap(Wrap { trim: true }).block(Block::default().borders(Borders::LEFT | Borders::RIGHT).border_style(Style::default().fg(Color::Rgb(40, 40, 40))).padding(Padding::horizontal(1))), col_layout[c_idx]);
                    }
                }
            } else {
                let sub_chunks = Layout::horizontal(vec![Constraint::Fill(1); cols]).split(infobox_inner);
                for col_idx in 0..cols {
                    let mut col_lines = Vec::new();
                    for (i, (header, label, value)) in all_fields.iter().enumerate() {
                        if i % cols == col_idx {
                            if let Some(h) = header {
                                col_lines.push(Line::from(Span::styled(h.to_uppercase(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
                            } else if col_lines.is_empty() {
                                col_lines.push(Line::raw(""));
                            }
                            col_lines.push(Line::from(Span::styled(label.replace("_", " ").to_uppercase(), Style::default().fg(Color::DarkGray))));
                            let segments = segments_from_str(value);
                            let spans: Vec<Span> = segments.iter()
                                .map(|s| process_segment(s, app.focused_link, &mut link_counter)).collect();
                            col_lines.push(Line::from(spans));
                            col_lines.push(Line::raw(""));
                        }
                    }
                    frame.render_widget(Paragraph::new(col_lines), sub_chunks[col_idx]);
                }
            }

            let mut body = Vec::new();
            for block in &article.body.blocks {
                match block {
                    BodyBlock::Heading { level, text } => {
                        let style = match level {
                            1 => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                            2 => Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
                            _ => Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)
                        };
                        body.push(Line::raw(""));
                        body.push(Line::styled(text, style));
                        body.push(Line::styled("─".repeat(main_chunks[2].width.saturating_sub(2) as usize), Style::default().fg(Color::Rgb(40, 40, 40))));
                    }
                    BodyBlock::Paragraph { segments } => {
                        body.push(Line::from(segments.iter()
                            .map(|s| process_segment(s, app.focused_link, &mut link_counter))
                            .collect::<Vec<_>>()));
                    }
                }
            }
            frame.render_widget(Paragraph::new(body).wrap(Wrap { trim: false }).block(Block::default().padding(Padding::horizontal(1))), main_chunks[2]);
        }
    }
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace().map(|word| {
        let low = word.to_lowercase();
        let sing = if low.ends_with('s') && low.len() > 1 {
            &low[..low.len() - 1]
        } else { &low };
        let mut c = sing.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().to_string() + c.as_str()
        }
    }).collect::<Vec<_>>().join(" ")
}
fn type_display_pretty(t: &ArticleType) -> (String, String){
    let (type_str, subtype_str) = type_display(t);
    (to_title_case(type_str), to_title_case(subtype_str))

}