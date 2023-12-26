pub struct Block {
    height: u8,
    width: u8,
    s_mirror: bool,
    s_point: bool,
    s_rotation: bool,
    content: [(u8, u8); 5],
}

impl Block {
    pub fn build(id: char) -> Option<Block> {
        match id {
            'F' => Some(Block {
                height: 3,
                width: 2,
                s_mirror: false,
                s_point: false,
                s_rotation: false,
                content: [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
            }),
            'I' => Some(Block {
                height: 5,
                width: 1,
                s_mirror: true,
                s_point: true,
                s_rotation: false,
                content: [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            }),
            'L' => Some(Block {
                height: 4,
                width: 2,
                s_mirror: false,
                s_point: false,
                s_rotation: false,
                content: [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
            }),
            'N' => Some(Block {
                height: 4,
                width: 2,
                s_mirror: false,
                s_point: false,
                s_rotation: false,
                content: [(1, 0), (1, 1), (0, 2), (1, 2), (0, 3)],
            }),
            'P' => Some(Block {
                height: 3,
                width: 2,
                s_mirror: false,
                s_point: false,
                s_rotation: false,
                content: [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
            }),
            'T' => Some(Block {
                height: 3,
                width: 3,
                s_mirror: true,
                s_point: false,
                s_rotation: false,
                content: [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],
            }),
            'U' => Some(Block {
                height: 2,
                width: 3,
                s_mirror: true,
                s_point: false,
                s_rotation: false,
                content: [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
            }),
            'V' => Some(Block {
                height: 3,
                width: 3,
                s_mirror: true,
                s_point: false,
                s_rotation: false,
                content: [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            }),
            'W' => Some(Block {
                height: 3,
                width: 3,
                s_mirror: true,
                s_point: false,
                s_rotation: false,
                content: [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            }),
            'X' => Some(Block {
                height: 3,
                width: 3,
                s_mirror: true,
                s_point: true,
                s_rotation: true,
                content: [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            }),
            'Y' => Some(Block {
                height: 4,
                width: 2,
                s_mirror: false,
                s_point: false,
                s_rotation: false,
                content: [(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
            }),
            'Z' => Some(Block {
                height: 3,
                width: 3,
                s_mirror: false,
                s_point: true,
                s_rotation: false,
                content: [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
            }),
            _ => None,
        }
    }

    pub fn rotate(&self) -> Block {
        Block {
            height: self.width,
            width: self.height,
            content: self.content.map(|(x, y)| (self.height - y - 1, x)),
            ..*self
        }
    }

    pub fn mirror(&self) -> Block {
        Block {
            content: self.content.map(|(x, y)| (self.width - x - 1, y)),
            ..*self
        }
    }
}
