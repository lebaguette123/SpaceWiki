use crate::types::{Block, Document, Segment};

pub fn parse_body(text: &str) -> Document{
    let mut blocks: Vec<Block> = Vec::new();
    let mut buffer: Vec<String> = Vec::new();

    for line in text.lines(){
        let trimmed = line.trim();
        if trimmed.starts_with("#"){
            let mut level: u8 = 0;
            let mut text = String::new();
            line.splitn(2, " ").for_each(|part| {
                if part.starts_with("#"){
                    for c in part.chars(){
                        if c == '#' {
                            level += 1;
                        } else {
                            break;
                        }
                    }
                }
                else{
                    text = part.trim().to_string();
                }
            });
            let heading = Block::Heading { level, text };
            if let Some(block) = flush_buffer(&mut buffer){
                blocks.push(block);
            }
            blocks.push(heading);
        }
        else if trimmed.is_empty(){
            if let Some(block) = flush_buffer(&mut buffer){
                blocks.push(block);
            }
        } else {
            buffer.push(trimmed.to_string());
        }
    }
    if let Some(block) = flush_buffer(&mut buffer){
        blocks.push(block);
    }
    Document { blocks }
}

pub fn flush_buffer(buffer: &mut Vec<String>) -> Option<Block>{
    if buffer.is_empty() {
        return None
    }
    let paragraph = buffer.join(" ");
    let seg = segments_from_str(&paragraph);
    buffer.clear();
    Some(Block::Paragraph { segments: seg })
}

pub fn segments_from_str(line: &str) -> Vec<Segment>{
    let mut segments = Vec::new();
    let mut remaining = line;

    loop{
        match remaining.find("[["){
            None => {
                if !remaining.is_empty(){
                    segments.push(Segment::Text(remaining.to_string()));
                }
                break;
            }
            Some(start) =>{
                let before = &remaining[..start];
                if !before.is_empty(){
                    segments.push(Segment::Text(before.to_string()));
                }
                let after_open = &remaining[start + 2..];
                match after_open.find("]]"){
                    None => {
                        segments.push(Segment::Text(remaining[start..].to_string()));
                        break;
                    }
                    Some(close) => {
                        let contents = &after_open[..close];
                        let mut parts = contents.splitn(2, "|");
                        let target = parts.next().unwrap().trim().to_string();
                        let display = parts.next().map(|s| s.trim().to_string());
                        segments.push(Segment::Link { target, display });
                        remaining = &remaining[start + 2 + close + 2..];
                    }
                }
            }
        }
    }
    segments
}