pub type Variant = [(u8, u8); 5];

pub fn rotate(arr: &Variant, height: u8) -> Variant {
    arr.map(|(x, y)| (height - y - 1, x))
}

pub fn mirror(arr: &Variant, width: u8) -> Variant {
    arr.map(|(x, y)| (width - x - 1, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_variant() {
        let variant: Variant = [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)];
        let rotated_variant = rotate(&variant, 5);
        assert_eq!([(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)], rotated_variant);

        let variant: Variant = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
        let rotated_variant = rotate(&variant, 3);
        assert_eq!([(2, 1), (2, 2), (1, 0), (1, 1), (0, 1)], rotated_variant);

        let variant: Variant = [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)];
        let rotated_variant = rotate(&variant, 3);
        assert_eq!([(2, 0), (2, 1), (1, 1), (0, 1), (0, 2)], rotated_variant);
    }

    #[test]
    fn mirror_variant() {
        let variant: Variant = [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)];
        let rotated_variant = mirror(&variant, 1);
        assert_eq!([(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)], rotated_variant);

        let variant: Variant = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
        let rotated_variant = mirror(&variant, 3);
        assert_eq!([(1, 0), (0, 0), (2, 1), (1, 1), (1, 2)], rotated_variant);

        let variant: Variant = [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)];
        let rotated_variant = mirror(&variant, 3);
        assert_eq!([(2, 0), (1, 0), (1, 1), (1, 2), (0, 2)], rotated_variant);
    }
}
