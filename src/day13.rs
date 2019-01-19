use std::cell::RefCell;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum CartTurn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Cart {
    c: Direction,
    turn: CartTurn,
    tick: bool, // To check if cart has already moved that turn.
}

impl Cart {
    fn intersection_turn(&mut self) {
        match self.turn {
            CartTurn::Left => {
                self.turn_left();
                self.turn = CartTurn::Straight;
            }
            CartTurn::Straight => {
                self.turn = CartTurn::Right;
            }
            CartTurn::Right => {
                self.turn_right();
                self.turn = CartTurn::Left;
            }
        }
    }

    fn turn_left(&mut self) {
        self.c = match self.c {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&mut self) {
        self.c = match self.c {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug)]
struct Track {
    track: char,
    cart: Option<Cart>,
}

#[derive(Debug)]
pub struct Tracks {
    tracks: Vec<Vec<Option<RefCell<Track>>>>,
    cur_tick: bool,
    remove_on_collision: bool,
}

// Cart on intersection is not parsed correctly, but input seems not to contain them.
fn parse_track(c: char) -> Option<RefCell<Track>> {
    let (cart, track) = match c {
        '/' => (None, c),
        '\\' => (None, c),
        '+' => (None, c),
        '-' => (None, c),
        '|' => (None, c),
        '>' => (Some(Direction::Right), '-'),
        '<' => (Some(Direction::Left), '-'),
        '^' => (Some(Direction::Up), '|'),
        'v' => (Some(Direction::Down), '|'),
        ' ' => return None,
        _ => panic!("unexpected char: {}", c),
    };

    Some(RefCell::new(Track {
        track,
        cart: cart.map(|c| Cart {
            c: c,
            turn: CartTurn::Left,
            tick: true,
        }),
    }))
}

impl FromStr for Tracks {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let initial: Vec<Vec<Option<RefCell<Track>>>> = s
            .lines()
            .map(|l| l.trim_end_matches('\n').chars().map(parse_track).collect())
            .collect();

        Ok(Tracks {
            tracks: initial,
            cur_tick: false,
            remove_on_collision: false,
        })
    }
}

impl Tracks {
    fn move_cart(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        let mut track = self.tracks[y][x].as_ref().unwrap().borrow_mut();
        let mut cart: Cart = track.cart.take().unwrap();

        let (next_y, next_x) = match cart.c {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        };
        let mut next_track = self.tracks[next_y][next_x].as_ref().unwrap().borrow_mut();
        if next_track.cart.is_some() {
            // Collision
            if self.remove_on_collision {
                next_track.cart = None;
            }
            return Some((next_x, next_y));
        }

        match (track.track, next_track.track) {
            ('-', '-') => (),
            ('-', '/') => cart.turn_left(),
            ('-', '\\') => cart.turn_right(),
            ('-', '+') => cart.intersection_turn(),
            ('|', '|') => (),
            ('|', '/') => cart.turn_right(),
            ('|', '\\') => cart.turn_left(),
            ('|', '+') => cart.intersection_turn(),
            ('/', '|') => (),
            ('/', '-') => (),
            ('/', '+') => cart.intersection_turn(),
            ('\\', '|') => (),
            ('\\', '-') => (),
            ('\\', '+') => cart.intersection_turn(),
            ('+', '-') => (),
            ('+', '|') => (),
            ('+', '\\') => match cart.c {
                Direction::Down | Direction::Up => cart.turn_left(),
                Direction::Left | Direction::Right => cart.turn_right(),
            },
            ('+', '/') => match cart.c {
                Direction::Down | Direction::Up => cart.turn_right(),
                Direction::Left | Direction::Right => cart.turn_left(),
            },
            ('+', '+') => cart.intersection_turn(),
            _ => panic!("unexpected track: {} {}", track.track, next_track.track),
        }

        cart.tick = self.cur_tick;
        next_track.cart = Some(cart);
        None
    }

    fn has_cart(&self, y: usize, x: usize) -> bool {
        let track = self.tracks[y][x].as_ref().unwrap().borrow();
        if let Some(cart) = &track.cart {
            // Cart has already moved in this tick, so it doesn't count.
            if cart.tick == self.cur_tick {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    // Return crashing cart position or position of last cart
    // depending or remove_on_collision.
    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut n_carts = 0;
        let mut last_cart = None;
        for (y, row) in self.tracks.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                if pos.is_some() && self.has_cart(y, x) {
                    n_carts += 1;
                    last_cart = Some((x, y));
                    if let Some(crash) = self.move_cart(y, x) {
                        if self.remove_on_collision == false {
                            return Some(crash);
                        }
                    }
                }
            }
        }
        self.cur_tick = !self.cur_tick;
        if self.remove_on_collision && n_carts <= 1 {
            return last_cart;
        }
        None
    }

    #[allow(dead_code)]
    fn print_track(&self) -> String {
        let out = self
            .tracks
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pos| match pos {
                        Some(t) => {
                            let track = t.borrow();
                            match &track.cart {
                                Some(cart) => match cart.c {
                                    Direction::Up => '^',
                                    Direction::Down => 'v',
                                    Direction::Left => '<',
                                    Direction::Right => '>',
                                },
                                None => track.track,
                            }
                        }
                        None => ' ',
                    })
                    .chain("\n".chars())
            })
            .flatten()
            .collect();

        out
    }
}

pub fn solve1(s: String) -> (usize, usize) {
    let mut tracks: Tracks = s.parse().unwrap();
    loop {
        // println!("{}", tracks.print_track());
        if let Some(crash) = tracks.tick() {
            // println!("{}", tracks.print_track());
            return crash;
        }
    }
}

pub fn solve2(s: String) -> (usize, usize) {
    let mut tracks: Tracks = s.parse().unwrap();
    tracks.remove_on_collision = true;
    loop {
        if let Some(last_cart) = tracks.tick() {
            return last_cart;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

        assert_eq!(solve1(input.to_string()), (7, 3));
    }

    #[test]
    fn test_solve2() {
        let input = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

        assert_eq!(solve2(input.to_string()), (6, 4));
    }
}
