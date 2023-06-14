use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}
 
impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen == other.antigen {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}/{:?}", self.antigen, self.rh_factor)
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let mut atg = String::new();
        for c in s.chars() {
            if c.is_alphabetic() {
                atg.push(c);
            }
        }
        let a = atg.parse::<Antigen>()?;
        
        let mut rh = String::new();
        for c in s.chars() {
            if !c.is_alphanumeric() {
                rh.push(c);
            }
        }
        let r = rh.parse::<RhFactor>()?;

            return Ok(BloodType { antigen: a, rh_factor: r })
        
        //Err("aa".to_string())
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &BloodType) -> bool {
        self.antigen == other.antigen || other.antigen == Antigen::O
    }

	pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();

        // Rh factor compatibility
        let compatible_rh_factor = match self.rh_factor {
            RhFactor::Positive => vec![RhFactor::Positive, RhFactor::Negative],
            RhFactor::Negative => vec![RhFactor::Negative],
        };

        // Antigen compatibility
        let compatible_antigens = match self.antigen {
            Antigen::O => vec![Antigen::O],
            Antigen::A => vec![Antigen::A, Antigen::O],
            Antigen::B => vec![Antigen::B, Antigen::O],
            Antigen::AB => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
        };

        for antigen in &compatible_antigens {
            for rh_factor in &compatible_rh_factor {
                donors.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                });
            }
        }
        //replace "Positive" with "+" and "Negative" with "-" and remove the slashe
        donors
    }

	pub fn recipients(&self) -> Vec<Self> {
        let mut recipients = Vec::new();

        // Rh factor compatibility
        let compatible_rh_factor = match self.rh_factor {
            RhFactor::Positive => vec![RhFactor::Positive],
            RhFactor::Negative => vec![RhFactor::Positive, RhFactor::Negative],
        };

        // Antigen compatibility
        let compatible_antigens = match self.antigen {
            Antigen::O => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
            Antigen::A => vec![Antigen::A, Antigen::AB],
            Antigen::B => vec![Antigen::B, Antigen::AB],
            Antigen::AB => vec![Antigen::AB],
        };

        for antigen in &compatible_antigens {
            for rh_factor in &compatible_rh_factor {
                recipients.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                });
            }
        }
        recipients
    }
}