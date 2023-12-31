use std::num::ParseIntError;

pub struct Config {
    pub blocks: String,
    pub height: u16,
    pub width: u16,
}

const BLOCKS: &str = "WVXZFUTNYLIP";

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut config = Config {
            blocks: String::from(BLOCKS),
            height: 6,
            width: 10,
        };

        while let Some(arg) = args.next() {
            match &arg[..] {
                "--blocks" | "-b" => {
                    if let Some(blocks) = args.next() {
                        let blocks = blocks.to_uppercase();
                        let chars = blocks.chars();

                        let is_forbidden_letter = chars.clone().any(|c| !BLOCKS.contains(c));

                        if is_forbidden_letter {
                            return Err("Found forbidden letter in \"blocks\" argument.");
                        }

                        let mut chars_vector: Vec<char> = chars.collect();
                        chars_vector.sort_by(|c1, c2| {
                            let c1_pos = BLOCKS.find(*c1);
                            let c2_pos = BLOCKS.find(*c2);

                            c1_pos.cmp(&c2_pos)
                        });

                        let str = String::from_iter(chars_vector);

                        config.blocks = str;
                    } else {
                        return Err("Didn't get value for \"blocks\" argument.");
                    }
                }
                "--height" | "-h" => {
                    if let Some(height) = args.next() {
                        match height.trim().parse() as Result<u16, ParseIntError> {
                            Ok(height) => config.height = height,
                            Err(_) => {
                                return Err(
                                    "Error parsing \"height\" argument. Pass integer value.",
                                );
                            }
                        }
                    } else {
                        return Err("Didn't get value for \"height\" argument.");
                    }
                }
                "--width" | "-w" => {
                    if let Some(width) = args.next() {
                        match width.trim().parse() as Result<u16, ParseIntError> {
                            Ok(width) => config.width = width,
                            Err(_) => {
                                return Err(
                                    "Error parsing \"width\" argument. Pass integer value.",
                                );
                            }
                        }
                    } else {
                        return Err("Didn't get value for \"width\" argument.");
                    }
                }
                _ => {
                    return Err("Unknown argument.");
                }
            }
        }

        Ok(config)
    }

    pub fn validate(config: &Config) -> Result<(), &'static str> {
        if config.blocks.len() != usize::from(config.height * config.width) {
            return Err("Board size mismatches number of blocks.");
        }

        Ok(())
    }
}
