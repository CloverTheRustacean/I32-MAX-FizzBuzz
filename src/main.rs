use std::thread::sleep;
use core::time::Duration;
pub mod country_road_take_me_home_to_a_lang_where_i_belong;

fn main() {
    println!("Welcome to Rust Lang");
    sleep(Duration::from_secs(10));
    let answer = country_road_take_me_home_to_a_lang_where_i_belong::smth::add(32, 32);
    println!("{}", answer);
}
