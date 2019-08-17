// Module for generating big primes (only up to 64 bits long)
// Includes Miller-Rabin primality test (simplified version)
// TODO: get rid of casting

use rand::Rng;

fn main() {
 print_primes_range(9500, 10000);
 println!("{} is a big prime", get_big_prime(61, 100));

}

fn print_primes_range(start: u64, end: u64) {
 for i in start..end {
  if is_prime(i, 200) { println!("{} is prime", i); }
 }
}

fn is_prime(m:u64, n_runs:usize) -> bool {
 // m is prime -> return true
 // m is composite -> return false with probability >= 1 - 2^(-n_runs)
 std::iter::repeat(m).take(n_runs).map(probably_prime).all(|x| x)
}

fn probably_prime(m: u64) -> bool {
 // m is prime -> return true
 // m is composite -> return false with probability >= 0.5
 // m is composite -> return true with probability < 0.5
 let mut i = 0;
 let mut l: u64 = m - 1;
 while l % 2 == 0 {
  i = i + 1;
  l = l / 2;
 }
 let mut x = rand::thread_rng().gen_range(1, m);
 x = square_multiply(x as u128, l as u128, m as u128);
 i = i - 1;
 let mut y = x;
 x = square_multiply(x as u128, 2u128, m as u128);
 while x != 1 && i >= 0 {
  i = i - 1;
  y = x;
  x = square_multiply(x as u128, 2u128,  m as u128);
 }
 i >= 0 && 1 <= y && y <= m - 1
}

fn square_multiply(base: u128, pow: u128, modulo: u128) -> u64 {
 if pow == 0 {1}
 else if pow == 1 {(base % modulo) as u64}
 else if pow % 2 == 0 {square_multiply((base * base) % modulo, pow / 2, modulo)}
 else {((base * square_multiply((base * base) % modulo, (pow - 1) / 2, modulo) as u128) % modulo) as u64}
}

fn get_big_number(n_bits: u32) -> u64 {
 let rand_number : u64 = rand::random::<u64>();
 let bitmask : u64 = 2u64.pow(n_bits) - 1;
 (rand_number & bitmask) | 2u64.pow(n_bits - 1)
}

fn make_uneven(number: u64) -> u64 {
 number | 1
}

fn get_big_prime(n_bits: u32, n_runs: usize) -> u64 {
 let possible_primes = std::iter::repeat(n_bits).map(get_big_number).map(make_uneven);
 let index = possible_primes.clone().map(|p| is_prime(p, n_runs)).position(|x| x).unwrap();
 possible_primes.clone().nth(index).unwrap()
}
