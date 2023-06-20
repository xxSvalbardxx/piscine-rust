#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            None
        } else if self.v % 2 == 0 {
            self.v /= 2;
            Some(self.v)
        } else {
            self.v = self.v * 3 + 1;
            Some(self.v)
        }
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 || n == 1{
        return 0;
    }
    
    
    let mut steps = 0;
    let mut collatz = Collatz::new(n);

    while let Some(_) = collatz.next() {
        steps += 1;
    }

    steps
}