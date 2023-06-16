use std::fmt;

pub struct Fahrenheit(pub f32);
pub struct Celsius(pub f32);

impl From<Fahrenheit> for Celsius {
    fn from(value: Fahrenheit) -> Self {
        Celsius((value.0 - 32.0) * 5.0 / 9.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°F", self.0)
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Fahrenheit((value.0 * 9.0 / 5.0) + 32.0)
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}
