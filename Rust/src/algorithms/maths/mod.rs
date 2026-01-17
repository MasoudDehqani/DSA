mod armstrong;
mod count_digits;
mod digital_root;
mod factorial;
mod fibonacci;
mod gcd;
pub mod pow;
mod prime_numbers;
mod reverse_number;

pub use factorial::factorial;
pub use fibonacci::{fibonacci, fibonacci_memoization};
pub mod palindrome;
pub use armstrong::is_armstrong_number;
pub use count_digits::count_digits;
pub use digital_root::digital_root;
pub use gcd::gcd;
pub use prime_numbers::is_prime;
pub use reverse_number::reverse_number;
