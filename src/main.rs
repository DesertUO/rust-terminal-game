use std::time::{Instant, Duration};
use std::thread::sleep;
use std::hint::spin_loop;

fn center_string_with_width_given_filler(string: String, width: usize, filler: char) -> String {
    let string_chars_count = string.chars().count();

    if string_chars_count >= width {
        return string;
    }

    let space_remaining = width - string_chars_count;
    let half_remaining = space_remaining / 2;

    let left: String = std::iter::repeat(filler).take(half_remaining).collect();
    let right: String = std::iter::repeat(filler).take(space_remaining - half_remaining).collect();

    let to_return = format!("{}{}{}", left, string, right);
    return to_return
}

fn center_string_with_width(string: String, width: usize) -> String {
    return center_string_with_width_given_filler(string, width, ' ')
}

struct Grid {
    width: i32,
    height: i32,
    data: Vec<char>
}

impl Grid {
    #[allow(dead_code)]
    fn initialize_data(&mut self) {
        self.data = vec![' '; (self.width * self.height) as usize];
    }
    fn initialize_data_with_value(&mut self, val: char) {
        self.data = vec![val; (self.width * self.height) as usize];
    }

    fn get(&self, x: i32, y: i32) -> Result<char, String> {
        if (x < 0) || (x > (self.width - 1)) {
            return Err("X position is out of bounds".to_string());
        }
        if (y < 0) || (y > (self.height - 1)) {
            return Err("Y position is out of bounds".to_string());
        }

        let c: char = self.data[(y * self.width + x) as usize];
        Ok(c)
    }

    fn get_rows_as_string(&self) -> Vec<String> {
        let width = self.width as usize;
        let height = self.height as usize;

        let mut rows_strings: Vec<String> = Vec::with_capacity(height);
        for j in 0..self.height {
            let mut row = String::with_capacity(width);
            for i in 0..self.width {
                row.push(self.get(i, j).unwrap());
            }
            rows_strings.push(row);
        }
        return rows_strings
    }
}

struct GameRenderer;

#[allow(dead_code)]
impl GameRenderer {
    pub fn render(grid: &Grid) {
    }

}

pub struct GameOptions {
    fps_target: Option<u32>
}

#[allow(dead_code)]
impl GameOptions {
    pub fn new() -> Self {
        Self {
            fps_target: Some(60),
        }
    }

    fn new_uncapped_fps() -> Self {
        Self {
            fps_target: None
        }
    }

    fn new_capped_fps(fps: u32) -> Self {
        Self {
            fps_target: Some(fps)
        }
    }
}

struct Game {
    game_options: GameOptions,
    grid: Grid,
    is_running: bool
}

impl Game {
    #[allow(dead_code)]
    fn initialize(&mut self) {
        self.game_options = GameOptions::new();
        self.grid.initialize_data();
        self.is_running = false;
    }
}

fn main() {
    let mut game = Game {
        game_options: GameOptions::new_capped_fps(60),
        grid: Grid {width: 30, height: 10, data: Vec::new()},
        is_running: false
    };

    game.grid.initialize_data_with_value(' ');
    let data_test = game.grid.get_rows_as_string();

    // Clear console
    // print!("\x1B[2J\x1B[H");
    // Hide cursor
    println!("\x1B[?25l");
    // Set cursor to the top line
    print!("\x1B[0;0H");
    println!("Program running...");

    // Gameloop
    let target_frame_duration = game.game_options.fps_target.map(|fps| Duration::from_secs_f64(1.0/ fps as f64));

    let mut last_frame_time = Instant::now();

    game.is_running = true;

    let mut count: f32 = 0.0;

    while game.is_running {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = now;

        // Update

        count += 1.0 * delta_time;

        // Render
        // Move cursor to the line bellow home (0, 0)
        print!("\x1B[1;0H");
        let curr_title = format!("counter {}", count.to_string());
        print!("{}\x1B[K\n", center_string_with_width(curr_title, (game.grid.width + 2) as usize));
        print!("┌{}┐\x1b[K\n", "─".repeat(game.grid.width as usize));
        let test_height = game.grid.height as usize;
        for j in 0..test_height {
            print!("│{}│\x1b[K\n", data_test[j]);
        }
        print!("└{}┘\x1b[K\n", "─".repeat(game.grid.width as usize));

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
    print!("\x1B[?25h");
}
