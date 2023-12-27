use colored::Color;

pub type Variant = [(u8, u8); 5];

#[derive(Debug)]
pub struct Block {
    pub variants: Vec<Variant>,
    pub color: Color,
}

struct Config {
    height: u8,
    width: u8,
    s_mirror: bool,
    s_point: bool,
    s_rotation: bool,
}

impl Block {
    fn new(variant: Variant, config: Config) -> Block {
        let mut variants = vec![variant];

        if !config.s_rotation {
            let variant_2 = rotate(&variant, config.height);
            variants.push(variant_2);

            if !config.s_point {
                let variant_3 = rotate(&variant_2, config.width);
                let variant_4 = rotate(&variant_3, config.height);

                variants.push(variant_3);
                variants.push(variant_4);
            }
        }

        let mut mirrored_variants = variants.clone();

        if !config.s_mirror {
            for (i, variant) in variants.iter().enumerate() {
                mirrored_variants.push(mirror(
                    &variant,
                    if i % 2 == 0 {
                        config.width
                    } else {
                        config.height
                    },
                ))
            }
        }

        Block {
            variants: mirrored_variants,
            color: Color::TrueColor {
                r: rand::random::<u8>(),
                g: rand::random::<u8>(),
                b: rand::random::<u8>(),
            },
        }
    }

    pub fn build(block_type: char) -> Option<Block> {
        match block_type {
            // 'F' => Some(Block::new(
            //     [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
            //     Config {
            //         height: 3,
            //         width: 3,
            //         s_mirror: false,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'I' => Some(Block::new(
            //     [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            //     Config {
            //         height: 5,
            //         width: 1,
            //         s_mirror: true,
            //         s_point: true,
            //         s_rotation: false,
            //     },
            // )),
            'L' => Some(Block::new(
                [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
                Config {
                    height: 4,
                    width: 2,
                    s_mirror: false,
                    s_point: false,
                    s_rotation: false,
                },
            )),
            // 'N' => Some(Block::new(
            //     [(1, 0), (1, 1), (0, 2), (1, 2), (0, 3)],
            //     Config {
            //         height: 4,
            //         width: 2,
            //         s_mirror: false,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'P' => Some(Block::new(
            //     [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
            //     Config {
            //         height: 3,
            //         width: 2,
            //         s_mirror: false,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'T' => Some(Block::new(
            //     [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],
            //     Config {
            //         height: 3,
            //         width: 3,
            //         s_mirror: true,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'U' => Some(Block::new(
            //     [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
            //     Config {
            //         height: 2,
            //         width: 3,
            //         s_mirror: true,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'V' => Some(Block::new(
            //     [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            //     Config {
            //         height: 3,
            //         width: 3,
            //         s_mirror: true,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'W' => Some(Block::new(
            //     [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            //     Config {
            //         height: 3,
            //         width: 3,
            //         s_mirror: true,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'X' => Some(Block::new(
            //     [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            //     Config {
            //         height: 3,
            //         width: 3,
            //         s_mirror: true,
            //         s_point: true,
            //         s_rotation: true,
            //     },
            // )),
            // 'Y' => Some(Block::new(
            //     [(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
            //     Config {
            //         height: 4,
            //         width: 2,
            //         s_mirror: false,
            //         s_point: false,
            //         s_rotation: false,
            //     },
            // )),
            // 'Z' => Some(Block::new(
            //     [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
            //     Config {
            //         height: 3,
            //         width: 3,
            //         s_mirror: false,
            //         s_point: true,
            //         s_rotation: false,
            //     },
            // )),
            _ => None,
        }
    }
}

fn rotate(arr: &Variant, height: u8) -> Variant {
    arr.map(|(x, y)| (height - y - 1, x))
}

fn mirror(arr: &Variant, width: u8) -> Variant {
    arr.map(|(x, y)| (width - x - 1, y))
}
