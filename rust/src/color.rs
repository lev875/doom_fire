#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn to_bytes(self) -> [u8; 4] {
        let Color { r, g, b, a } = self;
        [r, g, b, a]
    }
}

pub const PALETTE: [Color; 36] = [
    Color::new(7, 7, 7, 255),
    Color::new(31, 7, 7, 255),
    Color::new(47, 15, 7, 255),
    Color::new(71, 15, 7, 255),
    Color::new(87, 23, 7, 255),
    Color::new(103, 31, 7, 255),
    Color::new(119, 31, 7, 255),
    Color::new(143, 39, 7, 255),
    Color::new(159, 47, 7, 255),
    Color::new(175, 63, 7, 255),
    Color::new(191, 71, 7, 255),
    Color::new(199, 71, 7, 255),
    Color::new(223, 79, 7, 255),
    Color::new(223, 87, 7, 255),
    Color::new(223, 87, 7, 255),
    Color::new(215, 95, 7, 255),
    Color::new(215, 103, 15, 255),
    Color::new(207, 111, 15, 255),
    Color::new(207, 119, 15, 255),
    Color::new(207, 127, 15, 255),
    Color::new(207, 135, 23, 255),
    Color::new(199, 135, 23, 255),
    Color::new(199, 143, 23, 255),
    Color::new(199, 151, 31, 255),
    Color::new(191, 159, 31, 255),
    Color::new(191, 159, 31, 255),
    Color::new(191, 167, 39, 255),
    Color::new(191, 167, 39, 255),
    Color::new(191, 175, 47, 255),
    Color::new(183, 175, 47, 255),
    Color::new(183, 183, 47, 255),
    Color::new(183, 183, 55, 255),
    Color::new(207, 207, 111, 255),
    Color::new(223, 223, 159, 255),
    Color::new(239, 239, 199, 255),
    Color::new(255, 255, 255, 255),
];

#[cfg(test)]
mod test {
    use super::Color;

    #[test]
    fn color_new_test() {
        let [r, g, b, a] = [0, 15, 31, 255];
        let color = Color::new(r, g, b, a);
        assert_eq!(color.r, r);
        assert_eq!(color.g, g);
        assert_eq!(color.b, b);
        assert_eq!(color.a, a);
    }

    #[test]
    fn color_to_bytes_test() {
        let values = [0, 15, 31, 255];
        let [r, g, b, a] = values;
        let color = Color::new(r, g, b, a);
        let result = color.to_bytes();
        assert_eq!(result, values);
    }
}
