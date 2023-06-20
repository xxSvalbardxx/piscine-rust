
#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers {
    pub fn new(numbers: &'a [u32]) -> Self<'a> {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            return None;
        }
        Some(self.numbers.last().copied());
    }

    pub fn highest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            return None;
        }
        Some(self.numbers.iter().max().copied());
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