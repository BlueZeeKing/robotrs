use paste::paste;
use std::io::Read;

macro_rules! decode_number {
    ($number:ident) => {
        paste! {
            pub fn [<decode_ $number>](mut data: impl Read) -> Result<$number, std::io::Error> {
                let mut buf = [0; size_of::<$number>()];

                data.read_exact(&mut buf)?;

                Ok($number::from_be_bytes(buf))
            }
        }
    };
}

decode_number!(u8);
decode_number!(u16);
decode_number!(u32);
decode_number!(u64);
decode_number!(u128);
decode_number!(i8);
decode_number!(i16);
decode_number!(i32);
decode_number!(i64);
decode_number!(i128);
decode_number!(f32);
decode_number!(f64);
