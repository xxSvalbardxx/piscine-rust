
#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'_> {
    pub fn new(numbers: &'a [u32]) -> Numbers<'a> {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            return None;
        }
        Some(self.numbers.last().copied().unwrap())
    }

    pub fn highest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            return None;
        }
        Some(self.numbers.iter().max().copied().unwrap())
    }

    pub fn highest_three(&self) -> Vec<u32> {
        if self.numbers.is_empty() {
            return Vec::new();
        }
        let mut numbers = self.numbers.to_vec();
        numbers.sort();
        numbers.reverse();
        numbers.truncate(3);
        numbers
    }
}