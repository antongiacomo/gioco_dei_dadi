use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dado(u32);

impl Dado {
    pub fn new(size: u32) -> Dado {
        assert!(size != 0, "`size is zero");
        return Dado(size);
    }

    pub fn lancia(&self) -> u32 {
        let mut rng = thread_rng();
        return rng.gen_range(1..=self.0);
    }
}
