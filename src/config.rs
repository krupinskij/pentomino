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

                        let is_forbidden_letter = blocks.chars().any(|c| !BLOCKS.contains(c));

                        if is_forbidden_letter {
                            return Err("Found forbidden letter in \"blocks\" argument.");
                        }

                        config.blocks = blocks;
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
}
