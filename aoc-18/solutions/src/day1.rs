mod day1 {
   pub struct Freq {
        current: i32
    }

    impl Freq {
        pub fn new() -> Freq {
            Freq {current: 0}
        }

        pub fn get_current(&self) -> i32 { self.current}

        pub fn calibrate(&mut self, change: i32) {
            self.current += change;
        }

        pub fn calibrate_str(&mut self, change: &str) {
            self.calibrate(change.parse().unwrap());
        }
    }
}

#[cfg(test)]
mod tests{
use super::day1::Freq;

    #[test]
    fn init(){
        let f = Freq::new();
        assert_eq!(f.get_current(), 0);
    }

    #[test]
    fn add(){
        let mut f = Freq::new();
        f.calibrate(2);
        assert_eq!(f.get_current(), 2);
    }

    #[test]
    fn sub(){
        let mut f = Freq::new();
        f.calibrate(-2);
        assert_eq!(f.get_current(), -2);
    }

    #[test]
    fn add_sub(){
        let mut f = Freq::new();        
        f.calibrate(5);
        f.calibrate(-2);
        assert_eq!(f.get_current(), 3);
    }

    #[test]
    fn add_str(){
        let mut f = Freq::new();
        f.calibrate_str("+4");
        assert_eq!(f.get_current(), 4);
    }

    #[test]
    fn sub_str(){
        let mut f = Freq::new();
        f.calibrate_str("-2");
        assert_eq!(f.get_current(), -2);
    }

    #[test]
    fn add_sub_str(){
        let mut f = Freq::new();        
        f.calibrate_str("+5");
        f.calibrate_str("-2");
        assert_eq!(f.get_current(), 3);
    }

    #[test]
    fn complex_case_1(){
        let mut f = Freq::new();
        
        f.calibrate(5);
        f.calibrate(-7);
        f.calibrate(12);

        f.calibrate_str("+10");
        f.calibrate_str("-2");        
        f.calibrate_str("-10");
        assert_eq!(f.get_current(), 10);
    }
}