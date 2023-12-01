
pub fn cube_the_bits(bits: &[i32]) -> String{
    bits.iter()
        .fold(0, |acc, &bit| acc ^ bit)
        .pow(3)
        .to_string();
}