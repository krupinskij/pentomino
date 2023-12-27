use crate::block::{Block, Variant};
use colored::Color;

#[derive(Debug)]
pub struct Board {
    pub height: u16,
    pub width: u16,
    fields: Vec<Vec<Field>>,
}

#[derive(Debug, Clone)]
enum Field {
    Block(Color),
    Empty,
}

impl Board {
    pub fn new(height: u16, width: u16) -> Board {
        Board {
            height,
            width,
            fields: vec![vec![Field::Empty; usize::from(height)]; usize::from(width)],
        }
    }

    pub fn try_add_block(&mut self, b_x: u16, b_y: u16, variant: &Variant, block: &Block) -> bool {
        for (x, y) in variant.iter() {
            let pos_x = usize::from(b_x) + usize::from(*x);
            let pos_y = usize::from(b_y) + usize::from(*y);

            if let Some(col) = self.fields.get(pos_x) {
                if let Some(Field::Empty) = col.get(pos_y) {
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

            self.fields[pos_x][pos_y] = Field::Block(block.color);
        }
        println!("{:?}", self.fields);
        return true;
    }

    pub fn clear_block(&mut self, b_x: u16, b_y: u16, variant: &Variant) {
        for (x, y) in variant.iter() {
            let pos_x = usize::from(b_x) + usize::from(*x);
            let pos_y = usize::from(b_y) + usize::from(*y);

            self.fields[pos_x][pos_y] = Field::Empty;
        }
    }
}
