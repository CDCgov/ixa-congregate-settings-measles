use ixa::IxaError;

// use crate::utils::{cumulative_trapezoid_integral, linear_interpolation, trapezoid_integral};

// use super::InterpolatedRateFn;
use super::InfectiousnessRateFn;

/// Gamma rate function
/// maybe we don't need a separate struct or function
/// for this part because the rate can still be represented as 1 / `infection_duration`
/// but each agent's infectious duration can be drawn from a gamma distribution instead of constant
pub struct GammaRate {
    // a rate of infection in terms of people per unit time
    r: f64,
    // the time after which the rate of infection becomes 0
    infection_duration: f64,
}

impl GammaRate {
    /// # Errors
    /// - The rate of infection must be non-negative.
    /// - The duration of infection must be non-negative.
    pub fn new(r: f64, infection_duration: f64) -> Result<Self, IxaError> {
        if r < 0.0 {
            return Err(IxaError::IxaError(
                "The rate of infection must be non-negative.".to_string(),
            ));
        }
        if infection_duration < 0.0 {
            return Err(IxaError::IxaError(
                "The duration of infection must be non-negative.".to_string(),
            ));
        }
        Ok(Self {
            r,
            infection_duration,
        })
    }
}

impl InfectiousnessRateFn for GammaRate {
    fn rate(&self, t: f64) -> f64 {
        if t > self.infection_duration {
            return 0.0;
        }
        self.r
    }
    fn cum_rate(&self, t: f64) -> f64 {
        self.r * t.min(self.infection_duration)
    }
    fn inverse_cum_rate(&self, events: f64) -> Option<f64> {
        let t: f64 = events / self.r;
        if t > self.infection_duration {
            None
        } else {
            Some(t)
        }
    }
    fn infection_duration(&self) -> f64 {
        self.infection_duration
    }
}

#[cfg(test)]
mod test {
    // use ixa::assert_almost_eq;
    // use ixa::IxaError;
}

#[test]
/// Test that a negative rate of infection for `GammaRate` returns an `IxaError`
fn test_gamma_rate_errors_r_negative() {
    let e: Option<IxaError> = GammaRate::new(-5., 1.).err();
    match e {
        Some(IxaError::IxaError(msg)) => {
            assert_eq!(
                msg,
                "The rate of infection must be non-negative.".to_string()
            );
        }
        // what is ue or where is it defined?
        Some(ue) => panic!(
            "Expected an error that the rate of infection must be non-negative. Instead got {:?}",
            ue.to_string()
        ),
        None => panic!("Expected an error. Instead, created a gamma rate struct with no errors."),
    }
}
