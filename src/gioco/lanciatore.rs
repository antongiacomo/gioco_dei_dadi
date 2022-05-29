#[path = "./dado.rs"]
mod dado;

pub struct Lanciatore(Vec<u32>);

trait Max {
   fn get_index_max(&self) -> (u32, usize);
}

impl Max for Vec<u32> {
    fn get_index_max(&self) -> (u32, usize) {
        let mut max: u32 = 0;
        let mut index: usize = 0;
        for (i, n) in self.iter().enumerate() {
            if *n > max {
                max = *n;
                index = i + 1;
            }
        }
        return (max, index)
    }
}

impl Lanciatore {
    pub fn new(dadi: Vec<u32>) -> Lanciatore {
        return Lanciatore(dadi);
    }

    pub fn lancia(&self) -> (u32, Vec<u32>) {
        let mut x: Vec<u32> = [].to_vec();
        for size in self.0.iter() {
            let dado = dado::Dado::new(*size);
            x.push(dado.lancia());
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

        let (max, index) = bucket.get_index_max();

        if max == 1 {
            return 0;
        }

        return index as u32;
    }
}
