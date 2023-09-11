use lambdaworks_math::unsigned_integer::*;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args[0].parse::<u32>() == Ok(32u32) {
        println!("{:?}", u32_word::element::U384::from_hex_unchecked(&args[1]))
    } else if args[0].parse::<u32>() == Ok(64u32) {
        println!("{:?}", u64_word::element::U384::from_hex_unchecked(&args[1]))
    }
}

