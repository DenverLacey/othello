use crate::board::*;

use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue};
use std::io::{stdout, Stdout, Write};

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

pub struct Othello {
    board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    current: Player,
    cursor: Position,
}

impl Othello {
    pub fn new() -> Self {
        let mut board: Board<BOARD_WIDTH, BOARD_HEIGHT> = Board::new();

        board.set(
            (BOARD_WIDTH / 2 - 1) as isize,
            (BOARD_HEIGHT / 2 - 1) as isize,
            Tile::Black,
        );
        board.set(
            (BOARD_WIDTH / 2 - 0) as isize,
            (BOARD_HEIGHT / 2 - 1) as isize,
            Tile::White,
        );
        board.set(
            (BOARD_WIDTH / 2 - 0) as isize,
            (BOARD_HEIGHT / 2 - 0) as isize,
            Tile::Black,
        );
        board.set(
            (BOARD_WIDTH / 2 - 1) as isize,
            (BOARD_HEIGHT / 2 - 0) as isize,
            Tile::White,
        );

        Self {
            board,
            current: Player::Black,
            cursor: Position { x: 0, y: 0 },
        }
    }

    pub fn play(&mut self) -> std::io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        let result = self.cursor_setup(&mut stdout);
        crossterm::terminal::disable_raw_mode()?;
        result
    }

    fn cursor_setup(&mut self, stdout: &mut Stdout) -> std::io::Result<()> {
        execute!(stdout, crossterm::cursor::Hide)?;
        let result = self.game_loop(stdout);
        execute!(stdout, crossterm::cursor::Show)?;
        result
    }

    fn game_loop(&mut self, stdout: &mut Stdout) -> std::io::Result<()> {
        let mut error = None;
        self.draw(stdout, &error)?;

        loop {
            let event = crossterm::event::read()?;
            error = None;
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => {
                        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                        break;
                    }
                    KeyCode::Enter => {
                        let result = self.board.place(
                            self.cursor.x as isize,
                            self.cursor.y as isize,
                            self.current.into(),
                        );

                        match result {
                            Ok(_) => {
                                self.current = match self.current {
                                    Player::Black => Player::White,
                                    Player::White => Player::Black,
                                };
                            }
                            Err(err) => error = Some(err),
                        }
                    }
                    KeyCode::Up => {
                        if self.cursor.y > 0 {
                            self.cursor.y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.cursor.y < BOARD_HEIGHT - 1 {
                            self.cursor.y += 1;
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor.x > 0 {
                            self.cursor.x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor.x < BOARD_WIDTH - 1 {
                            self.cursor.x += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            self.draw(stdout, &error)?;
        }

        Ok(())
    }

    fn draw(&self, stdout: &mut Stdout, error: &Option<String>) -> std::io::Result<()> {
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let tile = self.board.get(x, y);
                let c: char = tile.into();
                if x == self.cursor.x && y == self.cursor.y {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::White),
                        Print(format!("{}", c)),
                        ResetColor,
                        Print(" "),
                    )?;
                } else {
                    queue!(stdout, Print(format!("{} ", c)))?;
                }
            }
            queue!(stdout, Print("\r\n"))?;
        }

        queue!(stdout, Print(format!("{:?}'s Turn.\r\n", self.current)))?;

        if let Some(err) = error {
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                Print(format!("{}\r\n", err)),
                ResetColor,
            )?;
        }

        stdout.flush()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Player {
    Black,
    White,
}

impl From<Player> for Tile {
    fn from(p: Player) -> Self {
        match p {
            Player::Black => Tile::Black,
            Player::White => Tile::White,
        }
    }
}
