pub fn is_armstrong_number(num: u32) -> bool {
  let mut sum = 0_u32;
  let length = count_digits(num);
  for n in 0..length {
    sum += digit_at(num, n).pow(length);
  }
  sum==num
}

fn count_digits(num: u32) -> u32 {
    let b = num as f64;
    (b.log10().floor() + 1_f64) as u32
}

fn digit_at(num: u32, index: u32) -> u32 {
  num / 10_u32.pow(index) % 10
}
