extern crate rustbox;
extern crate scribe;

pub mod modes;
pub mod presenters;
mod scrollable_region;

use models::terminal::Terminal;
use scribe::buffer::{Category, LineRange, Position, Range, Token};
use pad::PadStr;
use rustbox::Color;

pub struct Data {
    pub tokens: Vec<Token>,
    pub cursor: Position,
    pub highlight: Option<Range>,
    pub status_line: StatusLine
}

pub struct StatusLine {
    pub content: String,
    pub color: Color
}

pub fn map_color(category: &Category) -> Color {
    match category {
        &Category::Keyword    => Color::Yellow,
        &Category::Identifier => Color::Magenta,
        &Category::String     => Color::Red,
        &Category::Key        => Color::Red,
        &Category::Comment    => Color::Blue,
        &Category::Method     => Color::Cyan,
        &Category::Function   => Color::Cyan,
        &Category::Call       => Color::Cyan,
        _                     => Color::Default,
    }
}

pub fn draw_tokens(terminal: &Terminal, data: &Data) {
    let mut line = 0;
    let mut offset = 0;
    for token in data.tokens.iter() {
        let color = map_color(&token.category);

        for character in token.lexeme.chars() {
            let current_position = Position{ line: line, offset: offset };
            let background_color =
                match data.highlight {
                    Some(ref h) => {
                        if current_position >= h.start() && current_position < h.end() {
                            Color::White
                        } else {
                            Color::Default
                        }
                    },
                    None => {
                        Color::Default
                    }
                };

            if character == '\n' {
                line += 1;
                offset = 0;
            } else {
                offset += 1;

                terminal.print_char(
                    offset,
                    line,
                    rustbox::RB_NORMAL,
                    color,
                    background_color,
                    character
                );
            }
        }
    }
}

pub fn draw_status_line(terminal: &Terminal, content: &str, color: Color) {
    let line = terminal.height()-1;
    terminal.print(
        0,
        line,
        rustbox::RB_BOLD,
        Color::White,
        color,
        &content.pad_to_width(terminal.width())
    );
}
