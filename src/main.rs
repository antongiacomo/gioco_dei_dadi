use rand::{thread_rng, Rng};


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dado(u32);
impl Dado {
    pub fn new(&self,size: u32) -> Dado {
        assert!(size != 0, "`size is zero");
        return Dado(self.0);
    }
    pub fn lancia(&self) -> u32 {
        let mut rng = thread_rng();
        return rng.gen_range(1..=self.0);
    }

}

pub struct Lanciatore(Vec<u32>);
impl Lanciatore {
    pub fn lancia(&self) -> (u32, Vec<u32>) {
        let mut x: Vec<u32> = [].to_vec();
        for size in self.0.iter() {
            x.push(Dado(*size).lancia());
        }

        let punteggio = self.calcola_punteggio(x.clone());

        return (punteggio, x);
    }

    pub fn calcola_punteggio(&self, mut x: Vec<u32>) -> u32 {
        x.sort();

        if x == [1, 2, 3].to_vec() {
            return 0;
        }
        if x == [4, 5, 6].to_vec() {
            return 7;
        }

        let last = match x.last() {
            Some(x) => *x as usize,
            None => 0 as usize,
        };

        let mut bucket: Vec<u32> = vec![0; last];

        for result in x.iter() {
            let slot = *result - 1;
            bucket[slot as usize] += 1
        }
        let mut max: u32 = 0;
        let mut index: usize = 0;
        for (i, n) in bucket.iter().enumerate() {
            if *n > max {
                max = *n;
                index = i + 1;
            }
        }


        if max == 1 {
            return 0;
        }

        return index as u32;
    }
}

fn lanciamo() -> u32{
    let dadi = vec![6, 6, 6] ;
    let (punteggio, _risultati) = Lanciatore(dadi).lancia();
    return punteggio
}

fn main() {
    let mut pti_g1 = 0;
    let mut pti_g2 = 0 ;

    for _i in 1..=10000{


        pti_g1 += lanciamo();
        pti_g2 += lanciamo();

        // thread::sleep(Duration::from_millis(100));

    }

    println!("Punteggio Antongiacomo: {}", pti_g1);
    println!("Punteggio Giorgio: {}", pti_g2);
}
