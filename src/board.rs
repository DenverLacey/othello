use std::vec;

pub struct Board<const W: usize, const H: usize> {
    tiles: [[Tile; W]; H],
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; W]; H],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    pub fn set(&mut self, x: isize, y: isize, tile: Tile) {
        self.tiles[y as usize][x as usize] = tile;
    }

    pub fn place(&mut self, x: isize, y: isize, tile: Tile) -> Result<(), String> {
        if self.get(x as usize, y as usize) != Tile::Empty {
            return Err("Cannot place a tile here!".to_string());
        }

        let directions = self.find_directions(tile, x, y);
        if directions.is_empty() {
            return Err("Cannot place a tile here!".to_string());
        }

        let enemies = self.find_enemies_in_directions(tile, x, y, directions);
        for e in enemies {
            self.set(e.x as isize, e.y as isize, tile);
        }

        self.set(x, y, tile);

        Ok(())
    }

    fn find_directions(&self, friend: Tile, x: isize, y: isize) -> Vec<Direction> {
        let mut dirs = vec![];

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x + dx as isize;
                let ny = y + dy as isize;

                if nx < 0 || nx >= W as isize || ny < 0 || ny >= H as isize {
                    continue;
                }

                let neighbour = self.get(nx as usize, ny as usize);
                if neighbour == friend || neighbour == Tile::Empty {
                    continue;
                }

                if self.friendly_exists(friend, x, y, dx, dy) {
                    dirs.push(Direction { dx, dy });
                }
            }
        }

        dirs
    }

    fn find_enemies_in_directions(
        &self,
        friend: Tile,
        x: isize,
        y: isize,
        directions: Vec<Direction>,
    ) -> Vec<Position> {
        let mut enemies = vec![];

        for Direction { dx, dy } in directions {
            let mut x = x + dx as isize;
            let mut y = y + dy as isize;

            while (x >= 0 && (x as usize) < W) && (y >= 0 && (y as usize) < H) {
                let tile = self.get(x as usize, y as usize);
                if tile == friend {
                    break;
                } else if tile != Tile::Empty {
                    enemies.push(Position {
                        x: x as usize,
                        y: y as usize,
                    });
                }

                x += dx as isize;
                y += dy as isize;
            }
        }

        enemies
    }

    fn friendly_exists(&self, friend: Tile, x: isize, y: isize, dx: i8, dy: i8) -> bool {
        let mut x = x + dx as isize * 2;
        let mut y = y + dy as isize * 2;

        while (x >= 0 && (x as usize) < W) && (y >= 0 && (y as usize) < H) {
            if self.get(x as usize, y as usize) == friend {
                return true;
            }

            x += dx as isize;
            y += dy as isize;
        }

        false
    }
}

impl<const W: usize, const H: usize> std::fmt::Debug for Board<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..H {
            for x in 0..W {
                let c: char = self.get(x, y).into();
                write!(f, "{} ", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Black,
    White,
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => '.',
            Tile::Black => '\u{25A1}',
            Tile::White => '\u{25A0}',
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction {
    pub dx: i8,
    pub dy: i8,
}
