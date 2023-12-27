use crate::block::{Block, Variant};
use colored::{Colorize, CustomColor};
use std::fmt::{self, Display};

pub struct Board {
    pub height: u16,
    pub width: u16,
    fields: Vec<Vec<Field>>,
}

#[derive(Clone)]
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
}
