struct BIT {
    n: usize,
    data: Vec<usize>
}

impl BIT {
    fn init(size: usize) -> BIT {
        BIT {
            n: size,
            data: vec![0; size+1],
        }
    }
    
    fn add(&mut self, mut pos: isize, val: usize) {
        while pos as usize <= self.n {
            self.data[pos as usize] += val;
            pos += pos & -pos;
        }
    }
    
    fn sum(&self, mut pos:isize) -> usize {
        let mut cnt = 0;
        while pos > 0 {
            cnt += self.data[pos as usize];
            pos -= pos &-pos;
        }
        cnt
    }
}