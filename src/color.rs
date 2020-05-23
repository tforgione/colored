use std::borrow::Cow;
use std::convert::From;
use std::str::FromStr;

/// The 8 standard colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Palette(u8),
    True(u8, u8, u8),
}

#[allow(missing_docs)]
impl Color {
    pub fn to_fg_str(&self) -> Cow<str> {
        match *self {
            Color::Black => Cow::from("30"),
            Color::Red => Cow::from("31"),
            Color::Green => Cow::from("32"),
            Color::Yellow => Cow::from("33"),
            Color::Blue => Cow::from("34"),
            Color::Magenta => Cow::from("35"),
            Color::Cyan => Cow::from("36"),
            Color::White => Cow::from("37"),
            Color::BrightBlack => Cow::from("90"),
            Color::BrightRed => Cow::from("91"),
            Color::BrightGreen => Cow::from("92"),
            Color::BrightYellow => Cow::from("93"),
            Color::BrightBlue => Cow::from("94"),
            Color::BrightMagenta => Cow::from("95"),
            Color::BrightCyan => Cow::from("96"),
            Color::BrightWhite => Cow::from("97"),
            Color::Palette(code) => Cow::from(format!("38;5;{}", code)),
            Color::True(r, g, b) => Cow::from(format!("38;2;{};{};{}", r, g, b)),
        }
    }

    pub fn to_bg_str(&self) -> Cow<str> {
        match *self {
            Color::Black => Cow::from("40"),
            Color::Red => Cow::from("41"),
            Color::Green => Cow::from("42"),
            Color::Yellow => Cow::from("43"),
            Color::Blue => Cow::from("44"),
            Color::Magenta => Cow::from("45"),
            Color::Cyan => Cow::from("46"),
            Color::White => Cow::from("47"),
            Color::BrightBlack => Cow::from("100"),
            Color::BrightRed => Cow::from("101"),
            Color::BrightGreen => Cow::from("102"),
            Color::BrightYellow => Cow::from("103"),
            Color::BrightBlue => Cow::from("104"),
            Color::BrightMagenta => Cow::from("105"),
            Color::BrightCyan => Cow::from("106"),
            Color::BrightWhite => Cow::from("107"),
            Color::Palette(code) => Cow::from(format!("48;5;{}", code)),
            Color::True(r, g, b) => Cow::from(format!("48;2;{};{};{}", r, g, b)),
        }
    }
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "purple" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "bright black" => Ok(Color::BrightBlack),
            "bright red" => Ok(Color::BrightRed),
            "bright green" => Ok(Color::BrightGreen),
            "bright yellow" => Ok(Color::BrightYellow),
            "bright blue" => Ok(Color::BrightBlue),
            "bright magenta" => Ok(Color::BrightMagenta),
            "bright cyan" => Ok(Color::BrightCyan),
            "bright white" => Ok(Color::BrightWhite),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod from_str {
        pub use super::*;

        macro_rules! make_test {
            ( $( $name:ident: $src:expr => $dst:expr),* ) => {

                $(
                    #[test]
                    fn $name() {
                        let color : Color = $src.into();
                        assert_eq!($dst, color)
                    }
                )*
            }
        }

        make_test!(
            black: "black" => Color::Black,
            red: "red" => Color::Red,
            green: "green" => Color::Green,
            yellow: "yellow" => Color::Yellow,
            blue: "blue" => Color::Blue,
            magenta: "magenta" => Color::Magenta,
            purple: "purple" => Color::Magenta,
            cyan: "cyan" => Color::Cyan,
            white: "white" => Color::White,
            brightblack: "bright black" => Color::BrightBlack,
            brightred: "bright red" => Color::BrightRed,
            brightgreen: "bright green" => Color::BrightGreen,
            brightyellow: "bright yellow" => Color::BrightYellow,
            brightblue: "bright blue" => Color::BrightBlue,
            brightmagenta: "bright magenta" => Color::BrightMagenta,
            brightcyan: "bright cyan" => Color::BrightCyan,
            brightwhite: "bright white" => Color::BrightWhite,

            invalid: "invalid" => Color::White,
            capitalized: "BLUE" => Color::Blue,
            mixed_case: "bLuE" => Color::Blue
        );
    }

    mod from_string {
        pub use super::*;

        macro_rules! make_test {
            ( $( $name:ident: $src:expr => $dst:expr),* ) => {

                $(
                    #[test]
                    fn $name() {
                        let src = String::from($src);
                        let color : Color = src.into();
                        assert_eq!($dst, color)
                    }
                )*
            }
        }

        make_test!(
            black: "black" => Color::Black,
            red: "red" => Color::Red,
            green: "green" => Color::Green,
            yellow: "yellow" => Color::Yellow,
            blue: "blue" => Color::Blue,
            magenta: "magenta" => Color::Magenta,
            cyan: "cyan" => Color::Cyan,
            white: "white" => Color::White,
            brightblack: "bright black" => Color::BrightBlack,
            brightred: "bright red" => Color::BrightRed,
            brightgreen: "bright green" => Color::BrightGreen,
            brightyellow: "bright yellow" => Color::BrightYellow,
            brightblue: "bright blue" => Color::BrightBlue,
            brightmagenta: "bright magenta" => Color::BrightMagenta,
            brightcyan: "bright cyan" => Color::BrightCyan,
            brightwhite: "bright white" => Color::BrightWhite,

            invalid: "invalid" => Color::White,
            capitalized: "BLUE" => Color::Blue,
            mixed_case: "bLuE" => Color::Blue
        );
    }

    mod fromstr {
        pub use super::*;

        #[test]
        fn parse() {
            let color: Result<Color, _> = "blue".parse();
            assert_eq!(Ok(Color::Blue), color)
        }

        #[test]
        fn error() {
            let color: Result<Color, ()> = "bloublou".parse();
            assert_eq!(Err(()), color)
        }
    }
}
