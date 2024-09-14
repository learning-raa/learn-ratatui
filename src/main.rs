use anyhow::Result;
use ratatui::crossterm::event as xEvent;

mod app;
use app::*;

fn main() -> Result<()> {
    println!("start..");

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut app = App::new();
    while !app.should_quit() {
        // draw
        terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;

        // input
        loop {
            let events = collect_events()?;
            if !events.is_empty() {
                app.handle_input(events);
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}

//  //  //  //  //  //  //  //
static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_millis(1); //from_secs(0);
fn collect_events() -> Result<Vec<xEvent::Event>> {
    let mut result = Vec::new();
    while xEvent::poll(POLL_WAIT_TIME)? {
        result.push(xEvent::read()?);
    }
    Ok(result)
}
