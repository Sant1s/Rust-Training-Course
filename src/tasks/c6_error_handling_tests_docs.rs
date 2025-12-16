// This chapter is dedicated to the error handling, tests and documentation.

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    text.chars().next().ok_or_else(|| "Empty string".to_string())
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    line.split_whitespace()
        .map(|word| word.parse::<i32>().map_err(|_| "Invalid number".to_string()))
        .collect()
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        match &self.email {
            Some(email) => email.split('@').nth(1).map(|s| s.to_string()),
            None => None,
        }
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.

fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    fn test_factorial_of_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_of_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_factorial_of_small_numbers() {
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_of_larger_numbers() {
        assert_eq!(factorial(10), 3_628_800);
        assert_eq!(factorial(12), 479_001_600);
    }

    #[test]
    fn test_factorial_edge_case() {
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    use super::*;

    #[test]
    fn test_numbers_less_than_two() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_two_is_prime() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn test_small_prime_numbers() {
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);
    }

    #[test]
    fn test_small_composite_numbers() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(20), false);
    }

    #[test]
    fn test_large_prime_numbers() {
        assert_eq!(is_prime(97), true);
        assert_eq!(is_prime(101), true);
        assert_eq!(is_prime(1009), true);
        assert_eq!(is_prime(10007), true);
    }

    #[test]
    fn test_large_composite_numbers() {
        assert_eq!(is_prime(100), false);
        assert_eq!(is_prime(1000), false);
        assert_eq!(is_prime(10000), false);
    }

    #[test]
    fn test_perfect_squares() {
        assert_eq!(is_prime(49), false);
        assert_eq!(is_prime(121), false);
        assert_eq!(is_prime(169), false);
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// A log for storing and analyzing temperature readings for a specific city.
///
/// `TemperatureLog` allows you to collect daily temperature measurements
/// and perform basic statistical analysis, such as calculating the average temperature.
///
/// # Examples
///
/// ```
/// let mut log = TemperatureLog::new("Moscow");
///
/// log.add_reading(22.5);
/// log.add_reading(24.0);
/// log.add_reading(23.2);
///
/// if let Some(avg) = log.average() {
///     println!("Average temperature in {}: {:.1}°C", log.city, avg);
/// }
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// The name of the city for which temperatures are being recorded.
    pub city: String,

    /// A collection of temperature readings in degrees Celsius.
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Creates a new `TemperatureLog` for the specified city.
    ///
    /// The log is initialized with an empty list of readings.
    ///
    /// # Arguments
    ///
    /// * `city` - The name of the city for which to track temperatures.
    ///
    /// # Examples
    ///
    /// ```
    /// let log = TemperatureLog::new("London");
    /// assert_eq!(log.city, "London");
    /// assert!(log.readings.is_empty());
    /// ```
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }

    /// Adds a new temperature reading to the log.
    ///
    /// # Arguments
    ///
    /// * `value` - The temperature value in degrees Celsius to add.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut log = TemperatureLog::new("Paris");
    /// log.add_reading(18.5);
    /// log.add_reading(20.3);
    /// assert_eq!(log.readings.len(), 2);
    /// ```
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }

    /// Calculates and returns the average temperature from all readings.
    ///
    /// Returns `None` if there are no readings recorded yet.
    /// Returns `Some(average)` with the mean temperature value otherwise.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The average temperature if readings exist.
    /// * `None` - If the log contains no readings.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut log = TemperatureLog::new("Berlin");
    ///
    /// // No readings yet
    /// assert_eq!(log.average(), None);
    ///
    /// // Add some readings
    /// log.add_reading(20.0);
    /// log.add_reading(22.0);
    /// log.add_reading(24.0);
    ///
    /// // Calculate average: (20.0 + 22.0 + 24.0) / 3 = 22.0
    /// assert_eq!(log.average(), Some(22.0));
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}
