use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{error::Error, io, time::Duration};

mod app;
mod ascii;
mod menu;
mod ui;
use app::Cycle;

fn main() -> Result<(), Box<dyn Error>> {
    let result = std::panic::catch_unwind(|| {
        // setup terminal
        enable_raw_mode().expect("failed to enable raw mode");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .expect("failed to enter alternate screen");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("failed to create terminal");

        // create app and run it
        let mut app = Cycle::new();
        let res = run_app(&mut terminal, &mut app);

        // restore terminal
        disable_raw_mode().expect("failed to disable raw mode");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("failed to leave alternate screen");
        terminal.show_cursor().expect("failed to show cursor");

        if let Err(err) = res {
            println!("Error: {err:?}\nPress any key to exit.");
            event::read().expect("failed to read event");
        }
    });

    if let Err(panic) = result {
        // restore terminal
        disable_raw_mode().ok();
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).ok();
        eprintln!("Panic: {panic:?}\nPress any key to exit.");
        event::read().expect("failed to read event");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<impl Backend>, app: &mut Cycle) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => app.toggle_pause(),
                    KeyCode::Char('m') => app.toggle_menu(),
                    _ => {}
                }
            }
        }

        app.update();
    }
}
