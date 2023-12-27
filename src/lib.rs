use block::Block;
use board::Board;

mod block;
mod board;

pub fn run() {
    let board = Board::new(2, 5);
    let mut blocks: Vec<Block> = Vec::new();

    if let Some(block) = Block::build('L', '1') {
        blocks.push(block);
    }

    if let Some(block) = Block::build('L', '2') {
        blocks.push(block);
    }

    match_blocks(board, blocks)
}

fn match_blocks(mut board: Board, blocks: Vec<Block>) {
    match_blocks_rec(&mut board, &blocks, 0)
}

fn match_blocks_rec(board: &mut Board, blocks: &Vec<Block>, i: usize) {
    if let Some(block) = blocks.get(i) {
        for variant in block.variants.iter() {
            for x in 0..board.width {
                for y in 0..board.height {
                    let is_added = board.try_add_block(x, y, &variant, &block);

                    if is_added {
                        match_blocks_rec(board, &blocks, i + 1);

                        board.clear_block(x, y, &variant)
                    }
                }
            }
        }
    }
}
