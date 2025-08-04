pub struct DirectionMeasurements {
    count: u64,
    sum_sin_rad: f64,
    sum_cos_rad: f64,
}

impl DirectionMeasurements {
    pub fn new() -> Self {
        DirectionMeasurements {
            count: 0,
            sum_sin_rad: 0.0,
            sum_cos_rad: 0.0,
        }
    }

    pub fn from_values(values: &Vec<f64>) -> Self {
        let mut measurements = DirectionMeasurements::new();
        for &value in values {
            measurements.add_measurement(value);
        }
        measurements
    }

    pub fn add_measurement(&mut self, angle_degrees: f64) {
        self.count += 1;
        self.sum_sin_rad += angle_degrees.to_radians().sin();
        self.sum_cos_rad += angle_degrees.to_radians().cos();
    }

    pub fn average_direction(&self) -> f64 {
        if self.count == 0 {
            return f64::NAN;
        }
        let avg_sin_rad = self.sum_sin_rad / self.count as f64;
        let avg_cos_rad = self.sum_cos_rad / self.count as f64;
        let arctan = f64::atan2(avg_sin_rad, avg_cos_rad);
        let arctan_degrees = arctan.to_degrees();
        // atan2 returns values in the range [-180, 180], so we need to normalize it to [0, 360]
        if arctan_degrees < 0.0 {
            arctan_degrees + 360.0
        } else {
            arctan_degrees
        }
    }

    pub fn standard_deviation(&self) -> f64 {
        if self.count == 0 {
            return f64::NAN;
        }
        let avg_sin_rad = self.sum_sin_rad / self.count as f64;
        let avg_cos_rad = self.sum_cos_rad / self.count as f64;
        let epsilon = f64::sqrt(1.0 - (avg_sin_rad.powi(2) + avg_cos_rad.powi(2)));
        let arcsin = f64::asin(epsilon);
        let b = 2.0 / f64::sqrt(3.0) - 1.0; // constant from Yamartino paper
        let sigma = arcsin * (1.0 + b * epsilon.powi(3));
        sigma.to_degrees()
    }
}

impl Default for DirectionMeasurements {
    fn default() -> Self {
        DirectionMeasurements::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_measurements() {
        let measurements = DirectionMeasurements::new();
        assert!(measurements.average_direction().is_nan());
        assert!(measurements.standard_deviation().is_nan());
    }

    #[test]
    fn single_measurement_0() {
        let mut measurements = DirectionMeasurements::new();
        measurements.add_measurement(0.0);
        assert_eq!(measurements.average_direction(), 0.0);
        assert_eq!(measurements.standard_deviation(), 0.0);
    }

    #[test]
    fn single_measurement_90() {
        let mut measurements = DirectionMeasurements::new();
        measurements.add_measurement(90.0);
        assert_eq!(measurements.average_direction(), 90.0);
        assert_eq!(measurements.standard_deviation(), 0.0);
    }

    #[test]
    fn single_measurement_180() {
        let mut measurements = DirectionMeasurements::new();
        measurements.add_measurement(180.0);
        assert_eq!(measurements.average_direction(), 180.0);
        assert_eq!(measurements.standard_deviation(), 0.0);
    }

    #[test]
    fn single_measurement_270() {
        let mut measurements = DirectionMeasurements::new();
        measurements.add_measurement(270.0);
        assert_eq!(measurements.average_direction(), 270.0);
        assert_eq!(measurements.standard_deviation(), 0.0);
    }

    #[test]
    fn single_measurement_360() {
        let mut measurements = DirectionMeasurements::new();
        measurements.add_measurement(360.0);
        assert_eq!(measurements.average_direction(), 360.0);
        assert_eq!(measurements.standard_deviation(), 0.0);
    }

    #[test]
    fn multiple_measurements() {
        let mut measurements = DirectionMeasurements::new();
        measurements.add_measurement(0.0);
        measurements.add_measurement(90.0);
        assert_eq!(measurements.average_direction(), 45.0);
        assert!((measurements.standard_deviation() - 45.0).abs() < 3.0);
    }

    #[test]
    fn full_circle_measurements() {
        let mut measurements = DirectionMeasurements::new();
        for angle in 0..360 {
            measurements.add_measurement(angle as f64);
        }
        assert!(
            measurements.standard_deviation() < 104.0 && measurements.standard_deviation() > 103.0
        ); // Case explored in Yamartino paper
    }
}
