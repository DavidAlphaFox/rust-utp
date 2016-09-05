use std::time;
use std::ops::Sub;
use num_traits::ToPrimitive;

/// Return current time in microseconds since the UNIX epoch.
pub fn now_microseconds() -> u32 {
    let t = time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap_or_else(|e| e.duration());
    (t.as_secs().wrapping_mul(1_000_000) as u32).wrapping_add(t.subsec_nanos() / 1000)
}

/// Calculate the exponential weighted moving average for a vector of numbers, with a smoothing
/// factor `alpha` between 0 and 1. A higher `alpha` discounts older observations faster.
pub fn ewma<'a, T, I>(mut samples: I, alpha: f64) -> f64
    where T: ToPrimitive + 'a,
          I: Iterator<Item = &'a T>
{
    let first = samples.next().map_or(0.0, |v| v.to_f64().unwrap());
    samples.map(|v| v.to_f64().unwrap())
           .fold(first, |avg, sample| alpha * sample + (1.0 - alpha) * avg)
}

/// Returns the absolute difference between two values.
pub fn abs_diff<T: PartialOrd + Sub<Output = U>, U>(a: T, b: T) -> U {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod test {
    use util::ewma;

    #[test]
    fn test_ewma_empty_vector() {
        let empty: Vec<u32> = vec![];
        let alpha = 1.0 / 3.0;
        assert_eq!(ewma(empty.iter(), alpha), 0.0);
    }

    #[test]
    fn test_ewma_one_element() {
        let input = vec![1u32];
        let alpha = 1.0 / 3.0;
        assert_eq!(ewma(input.iter(), alpha), 1.0);
    }

    #[test]
    fn test_exponential_smoothed_moving_average() {
        let input = (1u32..11).collect::<Vec<u32>>();
        let alpha = 1.0 / 3.0;
        let expected = [1.0,
                        4.0 / 3.0,
                        17.0 / 9.0,
                        70.0 / 27.0,
                        275.0 / 81.0,
                        1036.0 / 243.0,
                        3773.0 / 729.0,
                        13378.0 / 2187.0,
                        46439.0 / 6561.0,
                        158488.0 / 19683.0];
        assert_eq!(ewma(input.iter(), alpha), expected[expected.len() - 1]);
    }
}
