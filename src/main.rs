mod game;

use game::{center_string_with_width_given_filler, Game, GameOptions, Grid};

use std::time::{Instant, Duration};
use std::thread::sleep;
use std::hint::spin_loop;
use::std::io::{stdout, BufWriter, Write};

use crossterm::{
    cursor::{Hide, Show, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
};


fn handle_events(game: &mut Game) -> Result<bool, String> {
    if event::poll(Duration::from_millis(0)).map_err(|e| e.to_string())? {
        if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => game.is_running = false,
                    _ => {}
                }
            }
        }
    }
    Ok(true)

}

fn render_game(game: &Game, stdout: &mut BufWriter<std::io::Stdout>, count_tmp: &f32) -> Result<bool, String> {
    let data = game.grid.get_rows_as_string();

    queue!(stdout, MoveTo(0,0)).map_err(|e| e.to_string())?;

    let title = format!(" RTG ");
    let top_border = format!("┌{}┐\n", center_string_with_width_given_filler(title, (game.grid.width) as usize, '─'));
    queue!(stdout, Print(top_border), Clear(ClearType::UntilNewLine)).map_err(|e| e.to_string())?;

    let test_height = game.grid.height as usize;
    for j in 0..test_height {
        let formatted_row = format!("│{}│\n", data[j]);
        queue!(stdout, Print(formatted_row), Clear(ClearType::UntilNewLine)).map_err(|e| e.to_string())?;
    }

    let bottom_border = format!("└{}┘\n", "─".repeat(game.grid.width as usize));
    queue!(stdout, Print(bottom_border), Clear(ClearType::UntilNewLine)).map_err(|e| e.to_string())?;

    queue!(stdout, Print("Press Esq to exit.\n\n"), Clear(ClearType::UntilNewLine)).map_err(|e| e.to_string())?;

    // Test
    let counter_tmp_test = format!("counter {}", count_tmp.to_string());
    queue!(stdout, Print(counter_tmp_test), Clear(ClearType::UntilNewLine)).map_err(|e| e.to_string())?;


    stdout.flush().map_err(|e| e.to_string())?;

    Ok(true)
}

fn main() {
    let mut game = Game {
        game_options: GameOptions::new_capped_fps(60),
        grid: Grid {width: 30, height: 10, data: Vec::new()},
        is_running: false
    };

    game.grid.initialize_data_with_value(' ');

    game.grid.data[1*30+5] = '0';
    game.grid.data[2*30+5] = '0';

    // Setting up terminal thingys
    let mut stdout = BufWriter::new(stdout());
    if let Err(err) = execute!(stdout, EnterAlternateScreen, Hide) {
        eprint!("Enter alternate screen error: {}", err);
        return;
    }
    if let Err(err) = crossterm::terminal::enable_raw_mode() {
        eprint!("Enable raw mode error: {}", err);
        return;
    }

    // Gameloop
    let target_frame_duration = game.game_options.fps_target.map(|fps| Duration::from_secs_f64(1.0/ fps as f64));
    let mut last_frame_time = Instant::now();

    game.is_running = true;

    let mut count: f32 = 0.0;

    while game.is_running {
        // Do some delta time start thingys
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = now;

        // Update
        if let Err(err) = handle_events(&mut game) {
            eprint!("Terminal event error: {}", err);
            break;
        }

        count += 1.0 * delta_time;

        // Render
        if let Err(err) = render_game(&game, &mut stdout, &count) {
            eprint!("Terminal rendering error: {}", err);
            break;
        }

        // Do some delta time end thingys
        if let Some(budget) = target_frame_duration {
            let sleep_padding = Duration::from_millis(2);

            while now.elapsed() < budget {
                let remaining = budget.saturating_sub(now.elapsed());

                if remaining > sleep_padding {
                    sleep(remaining - sleep_padding);
                } else {
                    spin_loop();
                }
            }
        }
    }

    // Disabling the terminal thingys
    if let Err(err) = crossterm::terminal::disable_raw_mode().map_err(|e| e.to_string()) {
        eprint!("Disable raw mode error: {}", err);
    }
    if let Err(err) = execute!(stdout, LeaveAlternateScreen, Show).map_err(|e| e.to_string()) {
        eprint!("Leave alternate screen error: {}", err);
    }

    println!("Program closed successfully.");
}
