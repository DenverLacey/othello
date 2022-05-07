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

    pub fn get(&self, x: isize, y: isize) -> Tile {
        self.tiles[y as usize][x as usize]
    }

    pub fn set(&mut self, x: isize, y: isize, tile: Tile) {
        self.tiles[y as usize][x as usize] = tile;
    }

    pub fn place(&mut self, x: isize, y: isize, tile: Tile) -> Result<usize, String> {
        if self.get(x, y) != Tile::Empty {
            return Err("Tile is already occupied!".to_string());
        }

        let enemies = self.find_trapped_enemies(tile, x, y);
        let num_enemies_trapped = enemies.len();

        if num_enemies_trapped == 0 {
            return Err("Cannot place a tile here!".to_string());
        }

        for e in enemies {
            self.set(e.x, e.y, tile);
        }

        self.set(x, y, tile);

        Ok(num_enemies_trapped)
    }

    fn find_trapped_enemies(&self, friend: Tile, x: isize, y: isize) -> Vec<Position> {
        let mut enemies = vec![];

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || nx >= W as isize || ny < 0 || ny >= H as isize {
                    continue;
                }

                let neighbour = self.get(nx, ny);
                if neighbour == friend || neighbour == Tile::Empty {
                    continue;
                }

                self.find_trapped_enemies_in_direction(&mut enemies, friend, nx, ny, dx, dy);
            }
        }

        enemies
    }

    fn find_trapped_enemies_in_direction(
        &self,
        enemies: &mut Vec<Position>,
        friend: Tile,
        mut x: isize,
        mut y: isize,
        dx: isize,
        dy: isize,
    ) {
        let before_len = enemies.len();
        let mut found_friend = false;

        while (x >= 0 && (x as usize) < W) && (y >= 0 && (y as usize) < H) {
            let tile = self.get(x, y);
            if tile == friend {
                found_friend = true;
                break;
            } else if tile == Tile::Empty {
                break;
            } else {
                enemies.push(Position { x, y });
            }

            x += dx;
            y += dy;
        }

        if !found_friend {
            enemies.truncate(before_len);
        }
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
    pub x: isize,
    pub y: isize,
}
