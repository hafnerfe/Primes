use rand::Rng;

fn main() {
 for i in 3..500 {
  println!("{}: {}", i, is_prime(i, 100));
 }
}

fn is_prime(m:u64, n_runs:usize) -> bool {
 // m is prime -> return true
 // m is composite -> return false with probability >= 0.5^(-n_runs)
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
 x = square_multiply(x, l, m);
 //x = 1;
 // x = u64::pow(x, l) % m; // alternative
 i = i - 1;
 let mut y = x;
 x = square_multiply(x, 2, m);
 while x != 1 && i >= 0 {
  i = i - 1;
  y = x;
  x = square_multiply(x, 2, m);
 }
 i >= 0 && 1 <= y && y <= m - 1
}

fn square_multiply(base: u64, pow: u64, modulo: u64) -> u64 {
 if pow == 0 {1}
 else if pow == 1 {base % modulo}
 else if pow % 2 == 0 {square_multiply((base * base) % modulo, pow / 2, modulo)}
 else {(base * square_multiply((base * base) % modulo, (pow-1) / 2, modulo)) % modulo}

}
