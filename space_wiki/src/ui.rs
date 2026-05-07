use ratatui::Frame;
use crate::app::App;  
use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut Frame, app: &App){
    const MIN_WIDTH: u16 = 80;
    const MIN_HEIGHT: u16 = 24;
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT{
        let warning = Paragraph::new("Terminal too small! Please resize to at least 80x24.");
        frame.render_widget(warning, area);
        return;
    }
}