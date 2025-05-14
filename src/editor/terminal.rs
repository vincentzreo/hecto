use std::{
    fmt::Display,
    io::{Write, stdout},
};

use crossterm::{
    Command,
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clean_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn clean_screen() -> Result<(), std::io::Error> {
        Self::query_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clean_line() -> Result<(), std::io::Error> {
        Self::query_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), std::io::Error> {
        Self::query_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), std::io::Error> {
        Self::query_command(Show)?;
        Ok(())
    }

    pub fn print<T: Display>(string: T) -> Result<(), std::io::Error> {
        Self::query_command(Print(string))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), std::io::Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::query_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    fn query_command<T: Command>(command: T) -> Result<(), std::io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
