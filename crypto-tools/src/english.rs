use phf::phf_map;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::collections::HashMap;

pub fn is_english(input: &[u8]) -> f64 {
    let mut text_distribution = HashMap::new();
    for c in input {
        let c = if c.is_ascii_alphabetic() || FREQUENCY_TABLE_ENG.contains_key(&c) {
            c.to_ascii_lowercase()
        } else {
            NULL_CHAR
        };

        text_distribution
            .entry(c)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let g = g_test(&text_distribution, &FREQUENCY_TABLE_ENG);
    //let g = chi_square(&text_distribution, &frequency_table_eng);

    p_value(g, FREQUENCY_TABLE_ENG.len() - 1)
}

static NULL_CHAR: u8 = 0;

/// https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
static FREQUENCY_TABLE_ENG: phf::Map<u8, f64> = phf_map! {
    b'a' => 0.082,
    b'b' => 0.015,
    b'c' => 0.028,
    b'd' => 0.043,
    b'e' => 0.127,
    b'f' => 0.022,
    b'g' => 0.02,
    b'h' => 0.061,
    b'i' => 0.07,
    b'j' => 0.0015,
    b'k' => 0.0077,
    b'l' => 0.04,
    b'm' => 0.024,
    b'n' => 0.067,
    b'o' => 0.075,
    b'p' => 0.019,
    b'q' => 0.0095,
    b'r' => 0.06,
    b's' => 0.063,
    b't' => 0.091,
    b'u' => 0.028,
    b'v' => 0.0098,
    b'w' => 0.024,
    b'x' => 0.0015,
    b'y' => 0.02,
    b'z' => 0.0074,
    // Added but not rebalanced, so the total will not loger add up to 1
    b' ' => 0.12,
    b'\0' => 0.001,
    //b'#' => 0.074,
    b'\'' => 0.074,
    b'\n' => 0.0074,
    b'-' => 0.0074,
};

/// https://en.wikipedia.org/wiki/G-test
fn g_test(observed: &HashMap<u8, u32>, expected: &phf::Map<u8, f64>) -> f64 {
    let count = observed.values().map(|v| *v as u32).sum::<u32>() as f64;

    let g_test_sum = |(i, e): (_, &f64)| {
        let o = observed.get(i).map(|v| *v as f64).unwrap_or(0.00001);
        o * (o / (e * count)).ln()
    };
    2.0 * expected.into_iter().map(g_test_sum).sum::<f64>()
}

/// chi square test
fn _chi_square(observed: &HashMap<u8, u32>, expected: &phf::Map<u8, f64>) -> f64 {
    let count = observed.values().map(|v| *v as u32).sum::<u32>() as f64;

    let chi_square_sum = |(i, e): (_, &f64)| {
        let o = observed.get(i).map(|v| *v as f64).unwrap_or(0.00001);
        //let e = expected.get(i).unwrap_or(&0.0);
        let e = e * count;
        (o - e).powi(2) / e
    };
    expected.into_iter().map(chi_square_sum).sum::<f64>()
}

fn p_value(chi_square: f64, degrees_freedom: usize) -> f64 {
    let distribution = ChiSquared::new(degrees_freedom as f64).unwrap();

    1.0 - distribution.cdf(chi_square)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use phf::phf_map;

    use crate::english::{chi_square, g_test, p_value};

    #[test]
    fn test_g_test() {
        let frequency_table_eng = phf_map! {
            b'a' => 1.0/3.0,
            b'b' => 1.0/3.0,
            b'c' => 1.0/3.0,
        };

        let mut count = HashMap::new();
        count.insert(b'a', 80);
        count.insert(b'b', 125);
        count.insert(b'c', 95);

        let g = g_test(&count, &frequency_table_eng);
        assert!((g - 10.337).abs() < 0.001);
        let p_value = p_value(g, frequency_table_eng.len() - 1);
        assert!((p_value - 0.005693).abs() < 0.000001);
    }

    #[test]
    fn test_chi_square() {
        let frequency_table_dice = phf_map! {
            b'1' => 1.0/6.0,
            b'2' => 1.0/6.0,
            b'3' => 1.0/6.0,
            b'4' => 1.0/6.0,
            b'5' => 1.0/6.0,
            b'6' => 1.0/6.0,
        };

        let mut count = HashMap::new();
        count.insert(b'1', 5);
        count.insert(b'2', 8);
        count.insert(b'3', 9);
        count.insert(b'4', 8);
        count.insert(b'5', 10);
        count.insert(b'6', 20);
        let g = chi_square(&count, &frequency_table_dice);
        assert!((g - 13.4).abs() < 0.0001, "The g value was: {g}");

        let p_value = p_value(g, frequency_table_dice.len() - 1);
        assert!((p_value - 0.019905).abs() < 0.000001);
    }
}
