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