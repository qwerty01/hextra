pub mod custom;
pub mod printable;

pub trait Decoder {
    fn map() -> CharMap;
}

pub struct CharMap([Option<&'static str>; 256]);

macro_rules! charmap_match {
    ($arr:ident, $v:literal: $start:literal .. => None) => {
        #[allow(unused_comparisons)]
        if $v >= $start {
            $arr[$v as usize] = None;
        }
    };
    ($arr:ident, $v:literal: $start:literal ..= $end:literal => None) => {
        #[allow(unused_comparisons)]
        if $v >= $start && $v <= $end {
            $arr[$v as usize] = None;
        }
    };
    ($arr:ident, $v:literal: $start:literal .. => [$($val:literal),+]) => {
        let v: usize = $v as _;
        #[allow(unused_comparisons)]
        if v >= $start {
            let val: [Option<&str>; 256-$start] = [$($val.into()),+];
            $arr[$v as usize] = val[v - $start];
        }
    };
    ($arr:ident, $v:literal: $start:literal ..= $end:literal => [$($val:literal),+]) => {
        let v: usize = $v as _;
        #[allow(unused_comparisons)]
        if v >= $start && v <= $end {
            let val: [Option<&str>; $end-$start+1] = [$(Some($val)),+];
            $arr[$v as usize] = val[v - $start];
        }
    };
    ($arr:ident, $v:literal: $chr:literal => $val:expr) => {
        if $v == $chr {
            $arr[$v as usize] = $val.into();
        }
    }
}
pub(crate) use charmap_match;
macro_rules! charmap {
    ($($pat:pat => $val:expr),+) => {{
        let mut val = [None; 256];
        match 0u8 {
            $($pat => ()),+
        }
        seq_macro::seq!(N in 0u8..256 {
            $(
                $crate::decode::charmap_match!(val, N: $pat => $val);
            )+
        });
        val
    }};
}
pub(crate) use charmap;
