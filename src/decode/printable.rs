use super::{charmap, CharMap, Decoder};

#[rustfmt::skip]
const BASIC_MAP: [Option<&str>; 256] = charmap! {
    0..=31 => None,
    32..=126 => [
        " ", "!", "\"","#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?",
        "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\","]", "^", "_",
        "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
        "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~"
    ],
    127.. => None
};
#[derive(Default)]
pub struct BasicDecoder;
impl Decoder for BasicDecoder {
    fn map() -> CharMap {
        CharMap(BASIC_MAP)
    }
}
