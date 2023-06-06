pub use chrono::{DateTime, Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
    pub form_values : (String, String),
    pub date : String,
    pub err : String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut v: Vec<&str> = Vec::new();
        if self.first_name.len() == 0 {
            return Err(FormError::new(String::from("first_name"), self.first_name.clone(), String::from("No user name")));
        } else {
            v.push("Valid first name");
        }
        if self.password.len() < 8 {
            return Err(FormError::new(String::from("password"), self.password.clone(), String::from("At least 8 characters")));
            // else if self.password contains at least one number, one letter and one none alphanumeric character
        } else if self.password.chars().all(char::is_numeric) {
            return Err(FormError::new(String::from("password"), self.password.clone(), String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)")));
        } else if self.password.chars().all(char::is_alphabetic) {
            return Err(FormError::new(String::from("password"), self.password.clone(), String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)")));
        } else if self.password.chars().all(char::is_ascii_punctuation) {
            return Err(FormError::new(String::from("password"), self.password.clone(), String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)")));
        } else {
            v.push("Valid password");
        }
        Ok(v)
    }
}

pub fn create_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("invalid date")
}