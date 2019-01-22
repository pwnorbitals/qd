// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Calculates the square root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).sqrt();
    /// let diff = (x - Double::SQRT_2).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn sqrt(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.is_sign_negative() {
            Double::NAN
        } else {
            // Strategy: use a method developed by Alan Karp and Peter Markstein at HP
            // https://cr.yp.to/bib/1997/karp.pdf
            //
            // If x is an approximation of sqrt(a), then
            //
            //      sqrt(a) ≈ ax + (a - (ax)^2)x / 2
            //
            // The approximation is accurate to twice the accuracy of x. This can be repeated an
            // arbitrary number of times, but this method when used on double-doubles only requires
            // one iteration.
            let x = Double::from_div(1.0, self.0.sqrt());
            let ax = self * x;
            ax + (self - ax.sqr()) * mul_pwr2(x, 0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_close!(dd!("1.7724538509055160272981674833411"), Double::PI.sqrt());
        assert_close!(dd!("48.135226186234961951944911890074"), dd!(2317).sqrt());
    }

    #[test]
    fn edge() {
        assert_exact!(Double::ZERO, dd!(0).sqrt());
        assert_exact!(Double::NAN, dd!(-3).sqrt());
    }
}