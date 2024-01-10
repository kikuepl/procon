struct SegTree {
    size: usize,
    ha: Vec<isize>,
}

impl SegTree {
    fn init(n: usize) -> SegTree {
        let mut size = 1;
        while size <n {
            size *= 2;
        }
        SegTree {
            size, 
            ha: vec![0; size*2]
        }
    }

    fn update(&mut self, mut pos: usize, x:isize) {
        pos += self.size;
        self.ha[pos]=x;
        while pos > 1{
            pos/=2;
            self.ha[pos]=max(self.ha[pos*2], self.ha[pos*2+1])
        }
    }

    fn query(&self, l: usize, r: usize, a: usize, b: usize, pos: usize) -> isize {
        if r<=a || b<=l {
            return -100_000_0007;
        } 
        if l<=a && b<=r {
            return self.ha[pos];
        }
        let mid = (a+b)/2;
        let ansl = self.query(l, r, a, mid, pos*2);
        let ansr = self.query(l, r, mid, b, pos*2+1);
        return max(ansl, ansr);
    }
}