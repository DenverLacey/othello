use crate::board::*;

use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode};
use crossterm::style::{Attribute, Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue};
use std::io::{stdout, Stdout, Write};

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

pub struct Othello {
    board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    current: Player,
    cursor: Position,
    scores: [usize; 2],
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
            scores: [2, 2],
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
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Char('q') => {
                        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        let result = self.board.place(
                            self.cursor.x as isize,
                            self.cursor.y as isize,
                            self.current.into(),
                        );

                        match result {
                            Ok(num_enemies_trapped) => {
                                match self.current {
                                    Player::Black => {
                                        self.scores[0] += num_enemies_trapped + 1;
                                        self.scores[1] -= num_enemies_trapped;
                                        self.current = Player::White;
                                    }
                                    Player::White => {
                                        self.scores[1] += num_enemies_trapped + 1;
                                        self.scores[0] -= num_enemies_trapped;
                                        self.current = Player::Black;
                                    }
                                }

                                if !self.board.player_has_legal_move(self.current.into()) {
                                    break;
                                }
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
                        if self.cursor.y < (BOARD_HEIGHT as isize) - 1 {
                            self.cursor.y += 1;
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor.x > 0 {
                            self.cursor.x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor.x < (BOARD_WIDTH as isize) - 1 {
                            self.cursor.x += 1;
                        }
                    }
                    _ => {}
                }
            }
            self.draw(stdout, &error)?;
        }

        self.draw(stdout, &error)?;

        let winner = if self.scores[0] > self.scores[1] {
            Some(Player::Black)
        } else if self.scores[0] < self.scores[1] {
            Some(Player::White)
        } else {
            None
        };

        self.draw_winner_message(stdout, winner)?;

        Ok(())
    }

    fn draw(&self, stdout: &mut Stdout, error: &Option<String>) -> std::io::Result<()> {
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        for y in 0..BOARD_HEIGHT as isize {
            for x in 0..BOARD_WIDTH as isize {
                let tile = self.board.get(x, y);
                let c: char = tile.into();
                if x == self.cursor.x && y == self.cursor.y {
                    queue!(
                        stdout,
                        Print(format!(
                            "{}{}{}",
                            Attribute::Reverse,
                            c,
                            Attribute::NoReverse
                        )),
                        Print(" "),
                    )?;
                } else {
                    queue!(stdout, Print(format!("{} ", c)))?;
                }
            }
            queue!(stdout, Print("\r\n"))?;
        }

        queue!(
            stdout,
            Print(format!(
                "{}: {}, {}: {}\r\n",
                char::from(Tile::Black),
                self.scores[0],
                char::from(Tile::White),
                self.scores[1]
            ))
        )?;

        let c: char = Tile::from(self.current).into();
        queue!(stdout, Print(format!("{}'s Turn.\r\n", c)))?;

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

    fn draw_winner_message(
        &mut self,
        stdout: &mut Stdout,
        winner: Option<Player>,
    ) -> std::io::Result<()> {
        let msg = if let Some(winner) = winner {
            let c: char = Tile::from(winner).into();
            format!("{} is the winner!\r\n", c)
        } else {
            "It's a draw!\r\n".into()
        };
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(msg),
            ResetColor,
        )
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
