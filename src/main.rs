use anyhow::Result;

#[derive(Debug)]
struct FreqCounter {
    counts : Vec<u32>,
    total : u32,
    size : usize,
}
impl FreqCounter {
    fn new(nsize : usize) -> Self {
        let mut empty_counts = Vec::with_capacity(nsize);
        for _ in 0..nsize {
            empty_counts.push(0);
        }
        FreqCounter { counts: empty_counts, total: 0, size: nsize }
    }
    fn add_report(&mut self, report : Vec<u8>) -> &mut Self {
        let report_iter = report.iter();
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
        let counts : Vec<bool> = self.counts.iter()
            .map(|&x| x > self.total / 2).collect();
        counts
    }
    fn empty(&mut self) -> &mut Self  {
        self.counts = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            self.counts.push(0);
        }
        self.total = 0;
        self
    }
}

fn main() -> Result<()> {
    let file_contents: String = std::fs::read_to_string("inputs/day3.txt")?;
    let reports: Vec<Vec<u8>> = file_contents.lines().map(|x| -> Vec<u8> {
        x.as_bytes().to_vec()
    }).collect();
    let bin_len = reports.iter().nth(0).clone().unwrap().len();
    let mut freq_counter = FreqCounter::new(bin_len);
    for report in &reports {
        freq_counter.add_report(report.to_owned());
    }


    println!("Calculating Oxygen Rating");
    let mut oxy_ratings: Vec<Vec<u8>> = reports.clone();
    let mut oxy_rating : u32 = 0;
    let mut oxy_freqs : FreqCounter = FreqCounter::new(bin_len);
    let mut bit_position: i32 = 0;
    loop {
        oxy_freqs.empty();
        for rating in &oxy_ratings {
            oxy_freqs.add_report(rating.to_owned());
        }
        let count = oxy_freqs.counts.iter().nth(bit_position as usize).unwrap();
        //println!("freqs: {:?}", oxy_freqs.counts);
        //println!("bitpo: {:?} count: {}  total: {}", bit_position, count, oxy_freqs.total);

        /* Filter down based on the calculated count */
        oxy_ratings = oxy_ratings.iter().filter(|x| {
            let diag_bit = x.iter().nth(bit_position as usize).unwrap();
            let freq : bool = (*count >= ((oxy_freqs.total+1)/2));
            let ret = (*diag_bit == 49 && freq) || (*diag_bit == 48 && !freq);
            //println!("freq {:?}, diag_bit {:?} ret {:?},", freq, diag_bit, ret);
            ret
        }).map(|x| x.clone()).collect::<Vec<Vec<u8>>>();
        if oxy_ratings.len() == 1 && oxy_rating == 0 {
            let x1 = oxy_ratings.iter().nth(0).unwrap();
            for (pow, byte) in x1.iter().enumerate() {
                oxy_rating += ((*byte as u32 - 48) << bin_len-pow-1);
            }
            println!("Oxy rating {:?}", oxy_rating);
            //println!("{:?}", x1);
            break;
        }
        bit_position+=1;
    }

    println!("Calculating CO2 Rating");
    let mut co2_ratings: Vec<Vec<u8>> = reports.clone();
    let mut co2_rating : u32 = 0;
    let mut co2_freqs : FreqCounter = FreqCounter::new(bin_len);
    bit_position = 0;
    loop {
        co2_freqs.empty();
        for rating in &co2_ratings {
            co2_freqs.add_report(rating.to_owned());
        }
        let count = co2_freqs.counts.iter().nth(bit_position as usize).unwrap();
        println!("freqs: {:?}", co2_freqs.counts);
        println!("bitpo: {:?} count: {}  total: {}", bit_position, count, co2_freqs.total);

        /* Filter down based on the calculated count */
        co2_ratings = co2_ratings.iter().filter(|x| {
            let diag_bit = x.iter().nth(bit_position as usize).unwrap();
            let freq : bool = (*count < ((co2_freqs.total+1)/2));
            let ret = (*diag_bit == 49 && freq) || (*diag_bit == 48 && !freq);
            println!("freq {:?}, diag_bit {:?} ret {:?},", freq, diag_bit, ret);
            ret
        }).map(|x| x.clone()).collect::<Vec<Vec<u8>>>();
        if co2_ratings.len() == 1 && co2_rating == 0 {
            let x1 = co2_ratings.iter().nth(0).unwrap();
            for (pow, byte) in x1.iter().enumerate() {
                co2_rating += ((*byte as u32 - 48) << bin_len-pow-1);
            }
            println!("CO2 rating {:?}", co2_rating);
            println!("{:?}", x1);
            break;
        }
        bit_position+=1;
    }

    println!("{:?}",co2_rating * oxy_rating);
    Ok(())
}
