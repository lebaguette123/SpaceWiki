use crate::types::{Block, Document, Segment};

pub fn parse_body(text: &str) -> Document{

}
pub fn flush_buffer(buffer: &mut Vec<String>) -> Option<Block>{
    if buffer.is_empty() {
        return None
    }
    let paragraph = buffer.join(" ");
    let seg = Segment::Text(paragraph);
    buffer.clear();
    Some(Block::Paragraph { segments: vec![seg] })
}