use anyhow::Result;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() -> Result<()>{
    let file_contents: String = std::fs::read_to_string("inputs/day1.txt")?;
    let mut depths_cache: VecDeque<i32> = VecDeque::new();
    depths_cache.push_back(0);
    depths_cache.push_back(0);
    depths_cache.push_back(0);
    let mut depth_increase_count : i32 = 0;
    let parser = |s: &str| -> i32 {
        let cur_depth : i32 = s.parse().unwrap();
        let last_depth_window : i32 = depths_cache.iter().sum::<i32>();
        let ret = depths_cache.pop_front().unwrap();
        depths_cache.push_back(cur_depth);
        println!("{:?}, {}, {}", depths_cache, depths_cache.iter().sum::<i32>(), last_depth_window);
        if depths_cache.iter().sum::<i32>() > last_depth_window {
            //println!("{} > {}", prev_depths.iter().sum::<i32>(), last_depth_window);
            depth_increase_count = depth_increase_count + 1;
            };
        ret
    };
    let depths : Vec<i32> = file_contents.split_whitespace().map(parser).collect();
    println!("{}", depth_increase_count);
    Ok(())
}
