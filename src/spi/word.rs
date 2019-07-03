pub trait Word {
    type ArchType;
    const LEN: u8;
    const MASK: Self::ArchType;
}

macro_rules! word_type {
    ($name:ident, $underlying:ident, $len:expr, $mask:expr) => {
        pub struct $name($underlying);

        impl $name {
            /// Create a new value.
            ///
            /// This will panic if the given value is out of range for the
            /// target word type.
            pub fn new(v: $underlying) -> Self {
                if v & (!$mask) != 0 {
                    panic!("value out of range for word type");
                }
                Self(v)
            }
        }

        impl Word for $name {
            type ArchType = $underlying;
            const LEN: u8 = $len;
            const MASK: $underlying = $mask;
        }
    };
}

word_type!(U1, u8, 1, 0b00000001);
word_type!(U2, u8, 2, 0b00000011);
word_type!(U3, u8, 3, 0b00000111);
word_type!(U4, u8, 4, 0b00001111);
word_type!(U5, u8, 5, 0b00011111);
word_type!(U6, u8, 6, 0b00111111);
word_type!(U7, u8, 7, 0b01111111);
word_type!(U8, u8, 8, 0b11111111);
word_type!(U9, u16, 9, 0b0000000111111111);
word_type!(U10, u16, 10, 0b0000001111111111);
word_type!(U11, u16, 11, 0b0000011111111111);
word_type!(U12, u16, 12, 0b0000111111111111);
word_type!(U13, u16, 13, 0b0001111111111111);
word_type!(U14, u16, 14, 0b0011111111111111);
word_type!(U15, u16, 15, 0b0111111111111111);
word_type!(U16, u16, 16, 0b1111111111111111);

impl Word for u8 {
    type ArchType = u8;
    const LEN: u8 = 8;
    const MASK: u8 = 0b11111111;
}

impl Word for u16 {
    type ArchType = u16;
    const LEN: u8 = 8;
    const MASK: u16 = 0xffff;
}
