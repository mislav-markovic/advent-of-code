struct Generator {
    curr_value: u64,
    factor: u64,
    radix: u64,
    cond: u64
}

impl Generator {
    fn new(start: u64, factor: u64, radix: u64, cond: u64) -> Generator {
        let mut val = (start*factor) % radix;

        while val % cond != 0 {
            val = (val*factor) % radix;
        }

        Generator{curr_value: val, factor: factor, radix: radix, cond: cond}
    }

    fn next_val(&mut self) {
        self.curr_value = (self.curr_value * self.factor) % self.radix;
    }

    fn lowest_bits(&self) -> u16 {
        (self.curr_value & 0xffff) as u16
    }

    fn next_with_cond(&mut self) {
        self.next_val();
        while self.curr_value % self.cond != 0 {
            self.next_val();
        }
    }
}

fn main() {
    let mut gen_a = Generator::new(699, 16807, 2147483647, 4);
    let mut gen_b = Generator::new(124, 48271, 2147483647, 8);
    let mut judge = 0;
    println!("Gen A: {}", gen_a.curr_value);
    println!("Gen B: {}", gen_b.curr_value);   

    for _ in 0..5_000_000 {
        if gen_a.lowest_bits() == gen_b.lowest_bits() {
            judge += 1;
        }
        gen_a.next_with_cond();
        gen_b.next_with_cond();
       //gen_a.next_val();
       //gen_b.next_val();
    }
    println!("{}", judge);
}
