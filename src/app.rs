use anyhow::Result;

#[allow(unused_imports)]
use tklog::{debug, error, fatal, info, trace, warn};

use ratatui::crossterm::event as xEvent;
use ratatui::prelude::*;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Paragraph, Wrap};

//  //  //  //  //  //  //  //
pub struct App {
    status: String,
    ed_state: edtui::EditorState,
    ed_handler: edtui::EditorEventHandler,
    exiting: bool,
}

impl App {
    pub fn new() -> Self {
        debug!("\t+ App");
        App {
            status: String::new(),
            ed_state: edtui::EditorState::new(edtui::Lines::from("started text 2")),
            ed_handler: edtui::EditorEventHandler::default(),
            exiting: false,
        }
    }

    pub fn run(mut self, mut terminal: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        trace!("\t> App.run()");
        loop {
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
            })?;

            let event = xEvent::read()?;
            match event {
                xEvent::Event::Key(key) => {
                    if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                        // <C-c>
                        if key.code == xEvent::KeyCode::Char('c') {
                            self.exiting = true;
                            warn!("exiting by <C-c>");
                            return Ok(());
                        }
                        // <C-e>
                        if key.code == xEvent::KeyCode::Char('e') {
                            self.exiting = true;
                            error!("exiting with error by <C-e>");
                            return Ok(());
                        }
                        // <C-p>
                        if key.code == xEvent::KeyCode::Char('p') {
                            self.exiting = true;
                            panic!("panic by <C-p>");
                        }
                    }else{
                        // q
                        if key.code == xEvent::KeyCode::Char('q') {
                            self.exiting = true;
                            info!("exiting by <q>");
                            return Ok(());
                        }
                        trace!("?");
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_input(&mut self, events: Vec<xEvent::Event>) {
        self.status.push_str(&format!("<n={}>", events.len()));
        for event in events {
            self.ed_handler.on_event(event.clone(), &mut self.ed_state);
            match event {
                xEvent::Event::FocusGained => {
                    self.status.push_str("<+focus>");
                }
                xEvent::Event::FocusLost => {
                    self.status.push_str("<-focus>");
                }
                xEvent::Event::Key(key) => {
                    match key.code {
                        xEvent::KeyCode::Char(c) => {
                            self.status.push(c);
                        }
                        _ => {}
                    }
                    // <C-c>
                    if key.code == xEvent::KeyCode::Char('c') {
                        if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                            self.exiting = true;
                            return;
                            //return Err(anyhow::anyhow!("<C-c>"));
                        }
                    }
                    // q
                    /*if key.code == xEvent::KeyCode::Char('q') {
                        self.exiting = true;
                        return;
                    }*/
                }
                _ => {}
            }
        }
    }
}
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // layout
        let [title_area, main_area, status_area] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Min(19),
            Constraint::Min(2),
        ])
        .areas(area);

        Paragraph::new("main title here")
            .block(Block::bordered().title("title of Main Title"))
            .render(title_area, buf);
        let main_block = Block::bordered();
        {
            let main_inner = main_block.inner(main_area);
            let [main_left, main_right] =
                Layout::horizontal([Constraint::Length(3), Constraint::Min(16)]).areas(main_inner);

            main_block.render(main_area, buf);
            Paragraph::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\nA\nB\nC\nD\nE\nF\n<-->")
                //.block(Block::bordered().title("title of Main Title"))
                .render(main_left, buf);
            edtui::EditorView::new(&mut self.ed_state).render(main_right, buf);
        }
        // status info
        Paragraph::new(self.status.clone())
            .wrap(Wrap { trim: true })
            .block(Block::bordered().title("debug information:"))
            .render(status_area, buf);
    }
}

//  //  //  //  //  //  //  //
static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_millis(1); //from_secs(0);
fn collect_events() -> Result<Vec<xEvent::Event>> {
    let mut result = Vec::new();
    //    while xEvent::poll(POLL_WAIT_TIME)? {
    result.push(xEvent::read()?);
    //    }
    Ok(result)
}
