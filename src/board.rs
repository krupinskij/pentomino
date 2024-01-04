use crate::block::Block;
use crate::helpers::Variant;
use colored::{Colorize, CustomColor};
use std::collections::HashSet;
use std::fmt::{self, Display};

pub struct Board {
    pub height: u16,
    pub width: u16,
    fields: Vec<Vec<Field>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Block(CustomColor),
    Empty,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.fields.iter() {
            for field in row {
                if let Field::Block(color) = field {
                    write!(f, "{}", "  ".on_custom_color(*color))?;
                } else {
                    write!(f, "{}", "  ".on_white())?;
                }
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

impl Board {
    pub fn new(height: u16, width: u16) -> Board {
        Board {
            height,
            width,
            fields: vec![vec![Field::Empty; usize::from(width)]; usize::from(height)],
        }
    }

    pub fn try_add_block(&mut self, b_x: u16, b_y: u16, variant: &Variant, block: &Block) -> bool {
        for (x, y) in variant.iter() {
            let pos_x = usize::from(b_x) + usize::from(*x);
            let pos_y = usize::from(b_y) + usize::from(*y);

            if let Some(row) = self.fields.get(pos_y) {
                if let Some(Field::Empty) = row.get(pos_x) {
                    continue;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        for (x, y) in variant.iter() {
            let pos_x = usize::from(b_x) + usize::from(*x);
            let pos_y = usize::from(b_y) + usize::from(*y);

            self.fields[pos_y][pos_x] = Field::Block(block.color);
        }
        return true;
    }

    pub fn clear_block(&mut self, b_x: u16, b_y: u16, variant: &Variant) {
        for (x, y) in variant.iter() {
            let pos_x = usize::from(b_x) + usize::from(*x);
            let pos_y = usize::from(b_y) + usize::from(*y);

            self.fields[pos_y][pos_x] = Field::Empty;
        }
    }

    pub fn check_empty_fields(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut set: HashSet<(u16, u16)> = HashSet::new();
                self.check_empty_field(x, y, &mut set);
                if set.len() % 5 != 0 {
                    return false;
                }
            }
        }

        true
    }

    fn check_empty_field(&self, x: u16, y: u16, set: &mut HashSet<(u16, u16)>) {
        if let Field::Empty = self.fields[usize::from(y)][usize::from(x)] {
            if set.contains(&(x, y)) {
                return;
            }
            set.insert((x, y));
            if x > 0 {
                self.check_empty_field(x - 1, y, set)
            }
            if x < (self.width - 1) {
                self.check_empty_field(x + 1, y, set)
            }
            if y > 0 {
                self.check_empty_field(x, y - 1, set)
            }
            if y < (self.height - 1) {
                self.check_empty_field(x, y + 1, set)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board_and_try_add_block() {
        let height = 5;
        let width = 1;

        let mut board = Board::new(height, width);
        let block = Block::build('I').unwrap();

        let is_added = board.try_add_block(0, 0, &block.variants[0], &block);

        assert!(is_added);
        assert_eq!(board.fields[0][0], Field::Block(block.color));

        let mut board = Board::new(height, width);
        let block = Block::build('F').unwrap();

        let is_added = board.try_add_block(0, 0, &block.variants[0], &block);

        assert!(!is_added);
        assert_eq!(board.fields[0][0], Field::Empty);

        let mut board = Board::new(1, 1);
        let block = Block::build('I').unwrap();

        let is_added = board.try_add_block(0, 0, &block.variants[0], &block);

        assert!(!is_added);
        assert_eq!(board.fields[0][0], Field::Empty);
    }

    #[test]
    fn create_board_and_clear() {
        let height = 5;
        let width = 2;

        let mut board = Board::new(height, width);
        let block = Block::build('I').unwrap();

        let is_added = board.try_add_block(0, 0, &block.variants[0], &block);
        assert!(is_added);

        let is_added = board.try_add_block(1, 0, &block.variants[0], &block);
        assert!(is_added);

        assert_eq!(board.fields[0][0], Field::Block(block.color));
        assert_eq!(board.fields[0][1], Field::Block(block.color));

        board.clear_block(0, 0, &block.variants[0]);
        assert_eq!(board.fields[0][0], Field::Empty);
        assert_eq!(board.fields[0][1], Field::Block(block.color));
    }

    #[test]
    fn create_board_and_check_empty_fields() {
        let board = Board::new(1, 1);
        let is_correct = board.check_empty_fields();
        assert!(!is_correct);

        let board = Board::new(5, 3);
        let is_correct = board.check_empty_fields();
        assert!(is_correct);

        let mut board = Board::new(5, 3);
        let block = Block::build('I').unwrap();
        let is_added = board.try_add_block(0, 0, &block.variants[0], &block);
        assert!(is_added);
        let is_correct = board.check_empty_fields();
        assert!(is_correct);

        let mut board = Board::new(5, 3);
        let block = Block::build('I').unwrap();
        let is_added = board.try_add_block(1, 0, &block.variants[0], &block);
        assert!(is_added);
        let is_correct = board.check_empty_fields();
        assert!(is_correct);

        let mut board = Board::new(5, 3);
        let block = Block::build('F').unwrap();
        let is_added = board.try_add_block(0, 0, &block.variants[0], &block);
        assert!(is_added);
        let is_correct = board.check_empty_fields();
        assert!(!is_correct);
    }
}
