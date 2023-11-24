/*use lambdaworks_math::unsigned_integer::u64_word::element::UnsignedInteger as Uint64;
use lambdaworks_math::unsigned_integer::u32_word::element::UnsignedInteger as Uint32;*/

fn convert_to_32bit_limb(input: Vec<u64>) -> Vec<u32> {
    let mut result = Vec::new();

    for &limb in input.iter() {
        let lower_limb = (limb & 0xFFFFFFFF) as u32;
        let upper_limb = ((limb >> 32) & 0xFFFFFFFF) as u32;

        result.push(upper_limb);
        result.push(lower_limb);
    }

    result
}

fn main() {

    let args = std::env::args().skip(1).collect::<Vec<String>>();

    let limbs = args.into_iter().map(|word| word.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    println!("{:?}", convert_to_32bit_limb(limbs));

    /*let limbs = [
        1445463580056702870,
        13122285128622708909,
        3107671372009581347,
        11396525602857743462,
        921361708038744867,
        6872850209053821716,
    ];

    let uint_64 = Uint64::from_limbs(limbs);

    let limb32 = convert_to_32bit_limb(limbs.to_vec());
    let uint_32 = Uint32::<12>::from_limbs(limb32[..12].try_into().unwrap());

    println!("{:?}", uint_64);

    println!("{:?}", uint_32);*/
}