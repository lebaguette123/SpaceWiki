use crate::types::{Segment, Block, Document};

pub struct BodyParser{
    current_paragraph: Option<Vec<Segment>>,
    blocks: Vec<Block>,
}
impl BodyParser{
    pub fn new() -> Self{
        BodyParser{
            current_paragraph: None,
            blocks: Vec::new(),
        }
    }

    pub fn parse(mut self, text: &str) -> Document{
        for line in text.lines(){
            let trimmed = line.trim();
            if trimmed.is_empty(){
                self.flush_paragraph();
                continue;
            }
            if trimmed.starts_with('#'){
                self.flush_paragraph();

                let mut level = 0;
                for c in trimmed.chars(){
                    if c == '#'{
                        level += 1;
                    } else {
                        break;
                    }
                }

                let text = trimmed.trim_start_matches('#').trim().to_string();
                self.blocks.push(Block::Heading{level, text});
            }
            else{
                let new_segments = self.segments_from_str(line);
                if let Some(ref mut existing_segments) = self.current_paragraph{
                    existing_segments.push(Segment::Text(" ".to_string()));
                    existing_segments.extend(new_segments);
                }
                else{
                    self.current_paragraph = Some(new_segments);
                }
            }
        }
        self.flush_paragraph();
        Document{ blocks: self.blocks }
    }
    pub fn flush_paragraph(&mut self){
        if let Some(paragraph) = self.current_paragraph.take(){
            let block = Block::Paragraph { segments: paragraph };
            self.blocks.push(block);
        }
        
    }
    pub fn segments_from_str()
}