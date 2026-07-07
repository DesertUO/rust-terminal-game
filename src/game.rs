pub fn center_string_with_width_given_filler(string: String, width: usize, filler: char) -> String {
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

pub fn center_string_with_width(string: String, width: usize) -> String {
    return center_string_with_width_given_filler(string, width, ' ')
}

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub data: Vec<char>
}

impl Grid {
    #[allow(dead_code)]
    pub fn initialize_data(&mut self) {
        self.data = vec![' '; (self.width * self.height) as usize];
    }
    pub fn initialize_data_with_value(&mut self, val: char) {
        self.data = vec![val; (self.width * self.height) as usize];
    }
    pub fn get(&self, x: i32, y: i32) -> Result<char, String> {
        if (x < 0) || (x > (self.width - 1)) {
            return Err("X position is out of bounds".to_string());
        }
        if (y < 0) || (y > (self.height - 1)) {
            return Err("Y position is out of bounds".to_string());
        }

        let c: char = self.data[(y * self.width + x) as usize];
        Ok(c)
    }
    pub fn get_rows_as_string(&self) -> Vec<String> {
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

pub struct Entity {
    pub id: usize
}
pub struct EntityStore {
    pub entities: Vec<Entity>,
    pub next_id: usize
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 0
        }
    }
    pub fn spawn_entity(&mut self) {
        let curr_id = self.next_id;
        self.next_id += 1;

        let new_entity = Entity { id: curr_id };
        self.entities.push(new_entity);
    }
    pub fn remove_by_id(&mut self, id: usize) -> Option<Entity> {
        if let Some(index) = self.entities.iter().position(|e| e.id == id) {
            Some(self.entities.swap_remove(index))
        } else {
            None
        }
    }
}

pub struct GameOptions {
    pub fps_target: Option<u32>
}

#[allow(dead_code)]
impl GameOptions {
    pub fn new() -> Self {
        Self {
            fps_target: Some(60),
        }
    }
    pub fn new_uncapped_fps() -> Self {
        Self {
            fps_target: None
        }
    }
    pub fn new_capped_fps(fps: u32) -> Self {
        Self {
            fps_target: Some(fps)
        }
    }
}

pub struct Game {
    pub game_options: GameOptions,
    pub grid: Grid,
    pub is_running: bool
}

impl Game {
    #[allow(dead_code)]
    pub fn initialize(&mut self) {
        self.game_options = GameOptions::new();
        self.grid.initialize_data();
        self.is_running = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_centering() {
        let res = center_string_with_width_given_filler("0".to_string(), 5, '-');
        assert_eq!(res, "--0--");
    }
    #[test]
    fn test_grid_data_getting() {
        let mut grid = Grid { width: 5, height: 5, data: Vec::new() };
        grid.initialize_data_with_value(' ');

        assert!(grid.get(0, 0).is_ok());
        assert!(grid.get(4, 0).is_ok());
        assert!(grid.get(0, 4).is_ok());
        assert!(grid.get(4, 4).is_ok());

        assert!(grid.get(-1, 0).is_err());
        assert!(grid.get(0, -1).is_err());
        assert!(grid.get(-1, -1).is_err());
        assert!(grid.get(5, 0).is_err());
        assert!(grid.get(0, 5).is_err());
        assert!(grid.get(5, 5).is_err());
    }
    #[test]
    fn test_grid_row_to_string() {
        let mut grid = Grid { width: 5, height: 5, data: Vec::new() };
        grid.initialize_data_with_value('v');

        grid.data[(2 + 2*grid.width) as usize] = 'u';

        let rows = grid.get_rows_as_string();

        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0], "vvvvv");
        assert_eq!(rows[2], "vvuvv");
        assert_eq!(rows[4], "vvvvv");
    }
}
