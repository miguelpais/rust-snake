pub fn even_ceiling(num: u16) -> u16 {
    num +  if num % 2 == 0 { 0 } else { 1 }
}
