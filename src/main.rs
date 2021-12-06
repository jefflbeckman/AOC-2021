use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum SubOrder {
    forward(i32),
    down(i32),
    up(i32),
}

#[derive(Debug)]
struct Sub {
    depth: i32,
    distance: i32,
    aim: i32,
}

impl From<&str> for SubOrder {
    fn from(i: &str) -> Self {
        let mut iter = i.split_whitespace();
        //println!("{}", i);
        match iter.next().unwrap() {
            "forward" => SubOrder::forward(iter.next().expect("missing value").parse().unwrap()),
            "down" => SubOrder::down(iter.next().expect("missing value").parse().unwrap()),
            "up" => SubOrder::up(iter.next().expect("missing value").parse().unwrap()),
            _ => SubOrder::forward(0),
        }
    }
}

fn main() -> Result<()>{
    let file_contents: String = std::fs::read_to_string("inputs/day2.txt")?;
    let orders : Vec<SubOrder> = file_contents.lines().map(|x| SubOrder::from(x)).collect();
    let mut sub = Sub { depth: 0, distance: 0, aim: 0 };
    orders.iter().for_each(|x| -> _ {
            match x {
                SubOrder::forward(d) => {
                    sub.distance = sub.distance + d;
                    sub.depth = sub.depth + (sub.aim * d)
                }
                SubOrder::down(d) => sub.aim = sub.aim + d,
                SubOrder::up(d) => sub.aim = sub.aim - d,
            };
            println!("{:?}, {:?}", x, sub);
        }
    );
    println!("{:?} {}",sub, sub.depth * sub.distance);

    Ok(())
}
