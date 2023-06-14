use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid Roman digit: {}", num),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let stat = num;
        let mut result = Vec::new();

        while num >= 1000 {
            result.push(M);
            num -= 1000;
        }

        if num >= 900 {
            result.push(C);
            result.push(M);
            num -= 900;
        } else if num >= 500 {
            result.push(D);
            num -= 500;
        } else if num >= 400 {
            result.push(C);
            result.push(D);
            num -= 400;
        }

        while num >= 100 {
            result.push(C);
            num -= 100;
        }

        if num >= 90 {
            result.push(X);
            result.push(C);
            num -= 90;
        } else if num >= 50 {
            result.push(L);
            num -= 50;
        } else if num >= 40 {
            result.push(X);
            result.push(L);
            num -= 40;
        }

        while num >= 10 {
            result.push(X);
            num -= 10;
        }

        if num >= 9 {
            result.push(I);
            result.push(X);
            num -= 9;
        } else if num >= 5 {
            result.push(V);
            num -= 5;
        } else if num >= 4 {
            result.push(I);
            result.push(V);
            num -= 4;
        }

        while num > 0 {
            result.push(I);
            num -= 1;
        }
        if stat == 0 {
            result.push(Nulla)
        }

        RomanNumber(result)
    }
    
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option <Self::Item> {
        if self.0.len() == 0 {
            return None;
        }
        let mut result :u32 = 0;
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i] == M {
                result += 1000;
            } else if self.0[i] == D {
                result += 500;
            } else if self.0[i] == C {
                if i + 1 < self.0.len() && self.0[i + 1] == M {
                    result += 900;
                    i += 1;
                } else if i + 1 < self.0.len() && self.0[i + 1] == D {
                    result += 400;
                    i += 1;
                } else {
                    result += 100;
                }
            } else if self.0[i] == L {
                result += 50;
            } else if self.0[i] == X {
                if i + 1 < self.0.len() && self.0[i + 1] == C {
                    result += 90;
                    i += 1;
                } else if i + 1 < self.0.len() && self.0[i + 1] == L {
                    result += 40;
                    i += 1;
                } else {
                    result += 10;
                }
            } else if self.0[i] == V {
                result += 5;
            } else if self.0[i] == I {
                if i + 1 < self.0.len() && self.0[i + 1] == X {
                    result += 9;
                    i += 1;
                } else if i + 1 < self.0.len() && self.0[i + 1] == V {
                    result += 4;
                    i += 1;
                } else {
                    result += 1;
                }
            }
            i += 1;
        }
        result += 1;
        Some(RomanNumber::from(result))
    }
}