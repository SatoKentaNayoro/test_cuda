use clap::{Arg, ArgAction};
use rand::Rng;

fn main() {
    println!("{}", "0649c729808ff7cb3622d0c99603d4fb2c9e2cd28fab1d163ba37ad45f020401cb16da456c889b43c4c517aa03d49a1a61db95727d9b6288c25ff3d0938120f6cfda601e3ef794a24c4d0405c5fcadb2b72cf26236d0b180c0fa7fe82999b96463fb476ca51e948b702b59469126a20ef88ee3022fae8b7dc7db848ba6a94c819a6baaa78275400100".len());
    println!("{}", hex::encode("0649c729808ff7cb3622d0c99603d4fb2c9e2cd28fab1d163ba37ad45f020401cb16da456c889b43c4c517aa03d49a1a61db95727d9b6288c25ff3d0938120f6cfda601e3ef794a24c4d0405c5fcadb2b72cf26236d0b180c0fa7fe82999b96463fb476ca51e948b702b59469126a20ef88ee3022fae8b7dc7db848ba6a94c819a6baaa78275400100").len());
    let vec = hex::decode("439b886c45da16cb").unwrap();
    let result = u32::from_le_bytes(vec.a);
    println!("{}", result)
}
