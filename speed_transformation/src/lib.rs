// Create a function that converts speed from km/h (kilometers per hour) and returns m/s (meters per second).

pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h / 3.6
}


#[cfg(test)]
mod tests {
    use speed_transformation::km_per_hour_to_meters_per_second;

    fn main() {
        let km_h = 100.0;
        let m_s = km_per_hour_to_meters_per_second(km_h);
        println!("{} km/h is equivalent to {} m/s", km_h, m_s);
    }
}
