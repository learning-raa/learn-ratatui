
use ratatui::crossterm::event as xEvent;
use ratatui::prelude::*;
use ratatui::widgets::Widget;
use ratatui::widgets::{Paragraph,Block};

pub struct App {
    exiting: bool,
}

impl App {
    pub fn new() -> Self {
        App { exiting: false }
    }

    pub fn should_quit(&self) -> bool {
        self.exiting
    }
    pub fn handle_input(&mut self, events: Vec<xEvent::Event>) {
        for event in events {
            match event {
                xEvent::Event::Key(key) => {
                    // <C-c>
                    if key.code == xEvent::KeyCode::Char('c') {
                        if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                            self.exiting = true;
                            return;
                            //return Err(anyhow::anyhow!("<C-c>"));
                        }
                    }
                    // q
                    if key.code == xEvent::KeyCode::Char('q') {
                        self.exiting = true;
                        return;
                    }
                }
                _ => {}
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rec1 = Rect{
            x: area.x,
            y: area.height-10,
            width: area.width,
            height: 10,
        };
        Paragraph::new("--> hi here")
            .white()
            .on_red()
            .render(rec1, buf);
        // blocks
        let rec2 = Rect{
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height-10,
        };
        let mega_block = Block::bordered()
            .white()
            .on_blue()
            .title("title?")
            .title_top("top")
            .title_bottom("bottom");

        let under_block = Block::bordered().title("underBlock-k-k!")
            .on_yellow();
        let under_para = Paragraph::new("some meta info\njjll?").centered();

        let under_rec = mega_block.inner(rec2);
        //
            mega_block.render(rec2, buf);
            //under_block.render(under_rec, buf);
            under_para.block(under_block).render(under_rec, buf);
    }
}
