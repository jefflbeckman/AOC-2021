use anyhow::Result;

#[derive(Debug)]
struct FreqCounter {
    counts : Vec<u32>,
    total : u32,
}
impl FreqCounter {
    fn new(size : usize) -> Self {
        let mut empty_counts = Vec::with_capacity(size);
        for i in 0..size {
            empty_counts.push(1);
        }
        FreqCounter { counts: empty_counts, total: 0}
    }
    fn add_report(&mut self, report : Vec<u8>) -> &mut Self {
        let mut report_iter = report.iter();
        let mut freq_iter = self.counts.iter();
        self.counts = report_iter.map(|bit| -> u32 {
            match bit
            {
                48 => *freq_iter.next().unwrap(),
                49 => *freq_iter.next().unwrap() + 1,
                _ => {println!("err"); 0}
            }
        }).collect();
        self.total += 1;
        self
    }
    fn get_freqs(&self) -> Vec<bool> {
        let mut gamma : u32 = 0;
        let mut epsilon : u32 = 0;
        let counts : Vec<bool> = self.counts.iter()
            .map(|&x| x > self.total / 2).collect();
        counts.iter().for_each(|&bit| {
            gamma <<= 1;
            epsilon <<= 1;
            gamma ^= bit as u32;
            epsilon ^= !bit as u32;
            println!("{:?} {:?} {:?}", bit, gamma, epsilon);

        });
        println!("{:?} {:?}",  gamma.to_be_bytes(), epsilon.to_be_bytes());
        gamma * epsilon
    }
}

fn main() -> Result<()>{
    let file_contents: String = std::fs::read_to_string("inputs/day3.txt")?;
    let reports : Vec<Vec<u8>> = file_contents.lines().map(|x| -> Vec<u8> {
        x.as_bytes().to_vec()
    }).collect();
    let mut freq_counter = FreqCounter::new(reports.iter().nth(0).clone().unwrap().len());
   for report in &reports {
       freq_counter.add_report(report.to_owned());
   }
    println!("{:?}",freq_counter);
    println!("{:?}",freq_counter.get_soln());

    Ok(())
}
