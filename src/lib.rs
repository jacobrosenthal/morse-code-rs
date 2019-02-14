#![no_std]

#[derive(Clone)]
pub enum MorseChar {
    Empty,
    Dot,
    Dash,
    Space,
}

type MorseSymbol = [MorseChar; 4];

pub struct MorseIter {
    string: &'static str,
    index: usize,
}

impl Iterator for MorseIter {
    type Item = MorseSymbol;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.string.len() {
            let index = self.index;

            self.index = self.index + 1;

            let chr = self.string.chars().nth(index).unwrap();

            Some(chr.into_morse_symbol())
        } else {
            None
        }
    }
}

pub trait IntoMorse {
    fn into_morse(self) -> MorseIter;
}

impl IntoMorse for &'static str {
    fn into_morse(self) -> MorseIter {
        MorseIter {
            string: self,
            index: 0,
        }
    }
}

pub trait IntoMorseSymbol {
    fn into_morse_symbol(self) -> MorseSymbol;
}

impl IntoMorseSymbol for char {
    fn into_morse_symbol(self) -> MorseSymbol {
        match self {
            'A' | 'a' => [
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            'B' | 'b' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dot,
            ],
            'C' | 'c' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dot,
            ],
            'D' | 'd' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Empty,
            ],
            'E' | 'e' => [
                MorseChar::Dot,
                MorseChar::Empty,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            'F' | 'f' => [
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dot,
            ],
            'G' | 'g' => [
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Empty,
            ],
            'H' | 'h' => [
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dot,
            ],
            'I' | 'i' => [
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            'J' | 'j' => [
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Dash,
            ],
            'K' | 'k' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Empty,
            ],
            'L' | 'l' => [
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dot,
            ],
            'M' | 'm' => [
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            'N' | 'n' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            'O' | 'o' => [
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Empty,
            ],
            'P' | 'p' => [
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Dot,
            ],
            'Q' | 'q' => [
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dash,
            ],
            'R' | 'r' => [
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Empty,
            ],
            'S' | 's' => [
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Empty,
            ],
            'T' | 't' => [
                MorseChar::Dash,
                MorseChar::Empty,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            'U' | 'u' => [
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Empty,
            ],
            'V' | 'v' => [
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dash,
            ],
            'W' | 'w' => [
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Empty,
            ],
            'X' | 'x' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dot,
                MorseChar::Dash,
            ],
            'Y' | 'y' => [
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dash,
                MorseChar::Dash,
            ],
            'Z' | 'z' => [
                MorseChar::Dash,
                MorseChar::Dash,
                MorseChar::Dot,
                MorseChar::Dot,
            ],
            ' ' => [
                MorseChar::Space,
                MorseChar::Empty,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
            _ => [
                MorseChar::Empty,
                MorseChar::Empty,
                MorseChar::Empty,
                MorseChar::Empty,
            ],
        }
    }
}
