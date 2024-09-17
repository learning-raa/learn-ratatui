use anyhow::Result;

#[allow(unused_imports)]
use tklog::{debug, error, fatal, info, trace, warn};

mod app;
use app::App;

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    log_init();
    info!("STARTED ----->");

    let terminal = ratatui::init();
    let result = App::new().run(terminal);

    ratatui::restore();
    if let Err(ref e) = result {
        error!(e.to_string());
    }

    info!("<-----");
    info!("############");
    info!(" ");
    result
}

//  //  //  //  //  //  //  //
fn log_init() {
    tklog::LOG
        .set_console(false)
        .set_level(tklog::LEVEL::Trace)
        .set_formatter("{level}\t{time} : {message}\n")
        .set_cutmode_by_size("/tmp/rust_debug.log", 1000000, 3, false);

    set_panic_hook();
    info!(" ");
    info!("############");
}

//  //  //  //  //  //  //  //
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        info!("<-----");
        for line in info.to_string().lines() {
            fatal!(line);
        }
        fatal!("############");
        fatal!(" ");
        hook(info);
    }));
}
