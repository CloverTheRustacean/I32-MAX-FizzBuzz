use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    overcomplicated_fizz_buzz();
    let duration = start.elapsed();
    println!("Time elapsed for our 32bit integer limit FizzBuzz: {:?}", duration);
}
 fn overcomplicated_fizz_buzz() {
     let (fizz, buzz, fizz_buzz) = (1..=i32::MAX)
         .into_par_iter()
         .fold(
             || (0u64, 0u64, 0u64),
             |mut counts, i| {
                 if i % 15 == 0 {
                     counts.2 += 1;
                 } else if i % 3 == 0 {
                     counts.0 += 1;
                 } else if i % 5 == 0 {
                     counts.1 += 1;
                 }
                 counts
             },
         )
         .reduce(
             || (0u64, 0u64, 0u64),
             |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2),
         );
     println!("fizz is {}", fizz);
     println!("buzz is {}", buzz);
     println!("fizz_buzz is {}", fizz_buzz);
 }