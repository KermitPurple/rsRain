use std::time::Duration;
use std::thread::sleep;
use rand::{
    seq::SliceRandom,
    Rng,
};
use termion::{
    self,
    cursor,
    terminal_size,
    color,
    clear,
};

type Coord = (u16, u16);

fn random_char_vec(len: usize) -> Vec<char> {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=[]{}\\|\"'?/.,<>;:".chars().collect();
    chars.choose_multiple(&mut rand::thread_rng(), len).cloned().collect()
}

#[derive(Debug)]
struct Trail{
    window_size: Coord,
    position: Coord,
    char_vec: Vec<char>,
    length: usize,
}

impl Trail{
    fn new(window_size: Coord, position: Coord, length: usize) -> Self {
        Self {
            window_size,
            position,
            char_vec: random_char_vec(length),
            length,
        }
    }

    fn make_trails(window_size: Coord) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut result = vec![];
        let length = window_size.1 / 5;
        for x in 1..window_size.0 {
            result.push(Self::new(
                window_size,
                (x, rng.gen_range(1..(window_size.1 + length))),
                length as usize
            ));
        }
        result
    }

    fn update(&mut self){
        // self.position = (self.position.0, self.position.1 + 1);
        self.position.1 += 1;
        if self.position.1 > self.window_size.1 {
            *self = Self::new(
                self.window_size,
                (self.position.0, 1), 
                self.length,
            );
        }
    }

    fn draw(&mut self){
        for i in 0..self.char_vec.len() {
            let y = self.position.1 + self.length as u16 - i as u16;
            if y < 1 {
                continue
            } else if y > self.window_size.1 {
                break;
            }
            print!(
                "{}{}",
                cursor::Goto(self.position.0, y),
                self.char_vec[i]
            );
        }
    }
}

fn main() {
    let mut trails = Trail::make_trails(terminal_size().unwrap());
    print!(
        "{}{}{}",
        cursor::Hide,
        color::Fg(color::LightGreen),
        cursor::Goto(1, 1),
    );
    loop {
        print!("{}", clear::All);
        for trail in &mut trails{
            trail.draw();
            trail.update();
        }
        sleep(Duration::from_millis(100));
    }
}
