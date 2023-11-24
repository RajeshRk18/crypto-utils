fn wnaf_form(num: u128, width: u32) -> Vec<u32>{
    let mut l = 0;
    let mut num = num;
    let mut wnaf = vec![0u32; 128];

    while num > 0 {
        if num % 2_u128 == 1_u128 {
            wnaf[l as usize] = (num % 2_u128.pow(width+1)) as u32;

            if wnaf[l as usize] >= 2_u32.pow(width) {
                wnaf[l as usize] -= 2_u32.pow(width);
            }

            num -= wnaf[l as usize] as u128;
        } else {
            wnaf[l as usize] = 0;
        }

        num /= 2;
        l += 1;
    }
    wnaf
}

fn main() {
    println!("{:?}", wnaf_form(564672435, 4));
}