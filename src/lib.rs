use block::Block;
use board::Board;
use config::Config;

mod block;
mod board;
pub mod config;

pub fn run(config: Config) {
    let board = Board::new(config.height, config.width);
    let mut blocks: Vec<Block> = Vec::new();
    let mut solutions: u32 = 0;

    config.blocks.chars().for_each(|c| {
        if let Some(block) = Block::build(c) {
            blocks.push(block);
        };
    });

    match_blocks(board, blocks, &mut solutions);

    println!("{} solution(s) found", solutions)
}

fn match_blocks(mut board: Board, blocks: Vec<Block>, solutions: &mut u32) {
    match_blocks_rec(&mut board, &blocks, 0, solutions)
}

fn match_blocks_rec(board: &mut Board, blocks: &Vec<Block>, i: usize, solutions: &mut u32) {
    if let Some(block) = blocks.get(i) {
        for variant in block.variants.iter() {
            for x in 0..board.width {
                for y in 0..board.height {
                    let is_added = board.try_add_block(x, y, &variant, block);

                    if is_added {
                        let are_empty_correct = board.check_empty_fields();
                        if are_empty_correct {
                            match_blocks_rec(board, &blocks, i + 1, solutions);
                        }

                        board.clear_block(x, y, &variant)
                    }
                }
            }
        }
    } else {
        println!("{}", board);
        *solutions += 1;
    }
}
