// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::*;
use crate::double::Double;
use std::f64;

// #region Powers

impl Double {
    /// Calculates `self`<sup>2</sup> and returns it as a new `Double`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available when the
    /// two numbers being multiplied are the same, so it is more efficient than bare multiplication.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from(3);
    /// assert!(dd.square() == dd * dd); // The left side is faster though
    /// ```
    #[inline]
    pub fn square(self) -> Double {
        let (p, e) = two_square(self.0);
        Double::from(quick_two_sum(
            p,
            e + 2.0 * self.0 * self.1 + self.1 * self.1,
        ))
    }

    /// Calculates `self`<sup>`n`</sup> and returns it as a new `Double`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from(3);
    /// assert!(dd.powi(3) == 27.0);
    /// ```
    pub fn powi(self, n: i32) -> Double {
        if n == 0 {
            return Double::from(1.0);
        }

        let mut r = self.clone();
        let mut s = Double::from(1.0);
        let mut i = n.abs();

        if i > 1 {
            while i > 0 {
                if i % 2 == 1 {
                    s *= r;
                }
                i /= 2;
                if i > 0 {
                    r = r.square();
                }
            }
        } else {
            s = r;
        }

        if n < 0 {
            1.0 / s
        } else {
            s
        }
    }

    #[inline]
    pub fn powf(self, n: Double) -> Double {
        // a^b = exp(b ln(a)), but since ln(a) is not defined for negative values, this works
        // ONLY FOR POSITIVE VALUES OF A (self in this case). Other solutions to powf are more
        // general but also much more complex and I am not yet ready to try one.
        (n * self.ln()).exp()
    }

    /// Calculates `self` &times; 2<sup>`n`</sup> and returns it as a new `Double`.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced mathematical
    /// calculations (including several within this library). Therefore an implementation that is
    /// much more efficient than calculating it through multiplication and [`powi`](#method.powi) is
    /// offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from(3);
    /// assert!(dd.ldexp(3) == 24.0); // 3 * 2^3
    /// ```
    #[inline]
    pub fn ldexp(self, n: i32) -> Double {
        Double(self.0 * 2f64.powi(n), self.1 * 2f64.powi(n))
    }
}

// #endregion

// #region Roots

impl Double {
    /// Calculates the square root of `self` and returns it as a new `Double`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from(2);
    /// // floating point error is reduced substantially but can't be eliminated,
    /// // so we check to see that the numbers are very close rather than equal
    /// assert!((dd.sqrt() - Double::SQRT_2).abs() < Double::EPSILON);
    /// ```
    pub fn sqrt(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.is_sign_negative() {
            Double::from(f64::NAN)
        } else {
            // Strategy: use a method developed by Alan Karp and Peter Markstein at HP
            // https://cr.yp.to/bib/1997/karp.pdf
            //
            // If x is an approximation of sqrt(a), then
            //
            //      sqrt(a) ≈ ax + (a - (ax)^2)x / 2
            //
            // The approximation is accurate to twice the accuracy of x. This can be repeated an
            // arbitrary number of times, but this method when used on double-doubles seems to only
            // require one iteration. (It can be performed with f64 mlutiplication for ax and
            // (...)x, but that proved less accurate with a single iteration and probably requires
            // more.)
            let x = Double::from_div(1.0, self.0.sqrt());
            let ax = self * x;
            ax + (self - ax.square()) * x * 0.5
        }
    }

    #[inline]
    pub fn cbrt(self) -> Double {
        self.nroot(3)
    }

    pub fn nroot(self, n: i32) -> Double {
        if n <= 0 {
            return Double::NAN;
        }
        if n % 2 == 0 && self.is_sign_negative() {
            return Double::NAN;
        }
        if n == 1 {
            return self.clone();
        }
        if n == 2 {
            return self.sqrt();  // use the more specialized method in sqrt
        }
        if self.is_zero() {
            return Double::ZERO;
        }

        // Strategy: the square root method is specialized for square roots, but the traditional
        // way of finding roots is using Newton's iteration for the function
        //
        //      f(x) = x^(-n) - a
        //
        // to find its root a^(-1/n). The iteration is therefore
        //
        //      x' = x + x * (1 - a * x^n) / n
        //
        // This converges quadratically, which is pretty fast. We can then find a^(1/n) by taking
        // the reciprocal.

        let r = self.abs();
        let mut x: Double = (-(r.0.ln()) / n as f64).exp().into();  // a^(-1/n) = exp(-ln(a) / n)

        x += x * (1.0 - r * x.powi(n)) / n as f64;
        if self.is_sign_negative() {
            x = -x;
        }

        x.recip()
    }
}

// #endregion
