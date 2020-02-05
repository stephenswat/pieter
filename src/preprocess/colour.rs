use image::Rgb;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Colour {
    Colour { hue: u8, lightness: u8 },
    Black,
    White,
    Other
}

impl Colour {
    pub fn from_rgb(Rgb(c): &Rgb<u8>) -> Colour {
        match c {
            [255, 192, 192] => Colour::Colour { hue: 0, lightness: 0 },
            [255,   0,   0] => Colour::Colour { hue: 0, lightness: 1 },
            [192,   0,   0] => Colour::Colour { hue: 0, lightness: 2 },

            [255, 255, 192] => Colour::Colour { hue: 1, lightness: 0 },
            [255, 255,   0] => Colour::Colour { hue: 1, lightness: 1 },
            [192, 192,   0] => Colour::Colour { hue: 1, lightness: 2 },

            [192, 255, 192] => Colour::Colour { hue: 2, lightness: 0 },
            [  0, 255,   0] => Colour::Colour { hue: 2, lightness: 1 },
            [  0, 192,   0] => Colour::Colour { hue: 2, lightness: 2 },

            [192, 255, 255] => Colour::Colour { hue: 3, lightness: 0 },
            [  0, 255, 255] => Colour::Colour { hue: 3, lightness: 1 },
            [  0, 192, 192] => Colour::Colour { hue: 3, lightness: 2 },

            [192, 192, 255] => Colour::Colour { hue: 4, lightness: 0 },
            [  0,   0, 255] => Colour::Colour { hue: 4, lightness: 1 },
            [  0,   0, 192] => Colour::Colour { hue: 4, lightness: 2 },

            [255, 192, 255] => Colour::Colour { hue: 5, lightness: 0 },
            [255,   0, 255] => Colour::Colour { hue: 5, lightness: 1 },
            [192,   0, 192] => Colour::Colour { hue: 5, lightness: 2 },

            [  0,   0,   0] => Colour::Black,
            [255, 255, 255] => Colour::White,
            _               => Colour::Other
        }
    }
}