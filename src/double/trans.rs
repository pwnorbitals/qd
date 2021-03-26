// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::{tables, Double};
use std::f64;

const TWO: Double = Double(2.0, 0.0);

const INV_K: Double = Double(0.001953125, 0.0);  //   1/512, used for exp

impl Double {
    /// Computes the exponential function, *e*<sup>x</sup>, where *x* is this `Double`.
    ///
    /// The result of this function grows rapidly. Once *x* exceeds 708, the result is too
    /// large to represent with a `Double`; at that point the function begins to return
    /// [`INFINITY`]. The limit on the low end is less due to the fact that the second
    /// component needs to fit in an `f64` rather than the first, along with extra bits used
    /// in argument reduction; this function begins to return 0 at -600.
    ///
    /// As *x* grows this function does lose a bit of precision. It's precise to at least 30
    /// digits up to values of -140 <= x <= 150, and from then until the limits, it's
    /// precise to at least 29 digits.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).exp();
    /// let expected = dd!("7.3890560989306502272304274605750057");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// # }
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    #[allow(clippy::many_single_char_names)]
    pub fn exp(self) -> Double {
        match self.pre_exp() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // We first reduce the range of the argument to a convenient size to perform
                // the calculation efficiently. This reduction takes advantage of the
                // following identity.
                //
                //      exp(kx) = exp(x)^k
                //
                // We in fact go a little further because it makes the reduction easier. 
                //
                //      exp(kx + m * ln(2)) = 2^m * exp(x)^k
                //
                // where m and k are arbitary integers. By choosing m appropriately we can
                // make |kx| <= ln(2) / 2 = 0.347. Then exp(x) is evaluated using a Taylor
                // series, which for exp(x) is pleasantly easy:
                //
                //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
                //
                // Reducing x substantially speeds up the convergence, so we have to use
                // fewer terms to reach the required precision.
                //
                // Once we have executed the Taylor series to produce an intermediate
                // answer, we expand it to compensate for the earlier reduction.

                // k = 512 is chosen; INV_K is defined above as that reciprocal
                let eps = INV_K * Double::EPSILON;
                // m doesn't need to be *that* accurate, so we calculate it with f64
                // arithmetic instead of the more expensive Double arithmetic
                let m = (self.0 / Double::LN_2.0 + 0.5).floor();

                // solving for x in exp(kx + m * ln(2)). INV_K is a power of 2 so we could
                // use mul_exp2, but on larger numbers that causes a loss of precision when
                // used with negative powers of two because bits are being shifted to the
                // right without accounting for the ones that are lost off the right.
                let x = (self - Double::LN_2 * Double(m, 0.0)) * INV_K;

                // This is the "x + x^2/2! + x^3/3!" part of the Taylor series.
                let mut p = x.sqr();
                let mut r = x + p.mul_pwr2(0.5);
                p *= x;
                let mut t = p * tables::INV_FACTS[0];
                let mut i = 0;

                // This is the rest of the Taylor series. We perform it as many times as
                // we need to reach our desired precision.
                loop {
                    r += t;
                    p *= x;
                    i += 1;
                    t = p * tables::INV_FACTS[i];
                    if i >= 5 || t.abs() <= eps {
                        break;
                    }
                }

                // Add the Taylor series parts together, then expand by the same number of
                // times that we reduced earlier.
                r += t;

                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                r = r * TWO + r.sqr();
                // Finally, add the "1 +" part of the Taylor series.
                r += Double::ONE;

                // Final step of expansion, this is the "* 2^m" part
                r.ldexp(m as i32)
            }
        }
    }

    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(7).ln();
    /// let expected = dd!("1.9459101490553133051053527434432");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// # }
    /// ```
    pub fn ln(self) -> Double {
        match self.pre_ln() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // The Taylor series for logarithms converges much more slowly than that of
                // exp because of the lack of a factorial term in the denominator. Hence
                // this routine instead tries to determine the root of the function
                //
                //      f(x) = exp(x) - a
                //
                // using Newton's iteration. This iteration is given by
                //
                //      x' = x - f(x)/f'(x)              (general Newton's iteration)
                //         = x - (exp(x) - a) / exp(x)
                //         = x - (1 - a / exp(x))
                //         = x - (1 - a * exp(-x))
                //         = x + a * exp(-x) - 1
                //
                // Because the derivative of exp(x) is exp(x), this is perhaps the simplest
                // of all Newton iterations.
                let mut x = Double(self.0.ln(), 0.0); // initial approximation

                let k = x.0.abs().log2().floor() as i32;
                let eps = Double::EPSILON.mul_pwr2(2f64.powi(k + 2));

                loop {
                    let r = x + self * (-x).exp() - Double::ONE;
                    if (x - r).abs() < eps {
                        return r;
                    }
                    x = r;
                }
            }
        }
    }

    /// Calculates log<sub>10</sub> of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E.log10();
    /// let expected = dd!("0.434294481903251827651128918916605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log10(self) -> Double {
        self.ln() / Double::LN_10
    }

    /// Calculates log<sub>2</sub> of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log2();
    /// let expected = dd!("3.32192809488736234787031942948939");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// # }
    /// ```
    #[inline]
    pub fn log2(self) -> Double {
        self.ln() / Double::LN_2
    }

    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the `Double`.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`,
    /// the specialized functions for those purposes([`ln`], [`log2`], and [`log10`]
    /// respectively) will be more efficient.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log(7.0);
    /// let expected = dd!("1.18329466245493832681792856164686");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// # }
    /// ```
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    #[inline]
    pub fn log(self, b: f64) -> Double {
        self.ln() / Double::from(b).ln()
    }

    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_exp(&self) -> Option<Double> {
        if self.0 < -600.0 {
            Some(Double::ZERO)
        } else if self.0 > 708.0 {
            Some(Double::INFINITY)
        } else if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(Double::ONE)
        } else if *self == Double::ONE {
            Some(Double::E)
        } else {
            None
        }
    }

    #[inline]
    fn pre_ln(&self) -> Option<Double> {
        if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_sign_negative() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(Double::NEG_INFINITY)
        } else if self.is_infinite() {
            Some(Double::INFINITY)
        } else if *self == Double::ONE {
            Some(Double::ZERO)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp() {
        assert_all_close!(
            dd!("1.0644944589178594295633905946428894"), dd!(0.0625).exp();
            dd!("1.1331484530668263168290072278117932"), dd!(0.125).exp();
            dd!("1.2062302494209807106555860104464342"), dd!(0.1875).exp();
            dd!("1.2840254166877414840734205680624368"), dd!(0.25).exp();

            dd!("0.93941306281347578611971082462230501"), dd!(-0.0625).exp();
            dd!("0.88249690258459540286489214322905049"), dd!(-0.125).exp();
            dd!("0.82902911818040034301464550934308218"), dd!(-0.1875).exp();
            dd!("0.77880078307140486824517026697832046"), dd!(-0.25).exp();

            dd!("23.140692632779269005729086367948552"), Double::PI.exp();
            dd!("15.154262241479264189760430272629902"), Double::E.exp();
            dd!("0.043213918263772249774417737171728016"), (-Double::PI).exp();
            dd!("0.065988035845312537076790187596846535"), (-Double::E).exp();
            dd!("535.49165552476473650304932958904745"), Double::MUL_2_PI.exp();
            dd!("4.8104773809653516554730356667038329"), Double::FRAC_PI_2.exp();
            dd!("4.113250378782927517173581815140309"), Double::SQRT_2.exp();
            dd!("2.0281149816474724511081261127463503"), Double::FRAC_1_SQRT_2.exp();
            dd!("1.3937095806663796973183419371414568e+65"), dd!(150).exp();
            dd!("1.5804200602736129648293184125529729e-61"), dd!(-140).exp();

            Double::E, Double::E.ln().exp();
            Double::ONE, Double::ONE.ln().exp();
            Double::E, Double::ONE.exp();
            dd!("2.7182818284590452353630057531811221"), dd!("1.000000000000000000001").exp();
            dd!("2.7182818284590452353575691895242041"), dd!("0.999999999999999999999").exp();
            dd!("22026.465794806716516957900645284255"), dd!(10).exp();
            dd!("0.00012340980408667954949763669073003385"), dd!(-9).exp();
        );
        assert_precision_all!(
            dd!("2.6503965530043108163386794472695841e-261"), dd!(-600).exp(), 29;
            dd!("1.0142320547350045094553295952312673e+304"), dd!(700).exp(), 29;
            dd!("3.0233831442760550147756219850967309e+307"), dd!(708).exp(), 29;
        );
        assert_all_exact!(
            Double::ZERO, dd!(-710).exp();
            Double::INFINITY, dd!(710).exp();
            Double::ONE, Double::ZERO.exp();
            Double::ONE, Double::NEG_ZERO.exp();
            Double::INFINITY, Double::INFINITY.exp();
            Double::ZERO, Double::NEG_INFINITY.exp();
            Double::NAN, Double::NAN.exp();
        );
    }

    #[test]
    fn ln() {
        assert_all_close!(
            dd!("1.1447298858494001741434273513531"), Double::PI.ln();
            Double::ONE, Double::E.ln();
            dd!("1.8378770664093454835606594728112"), Double::MUL_2_PI.ln();
            dd!("0.45158270528945486472619522989488"), Double::FRAC_PI_2.ln();
            dd!("0.34657359027997265470861606072909"), Double::SQRT_2.ln();
            dd!("-0.34657359027997265470861606072909"), Double::FRAC_1_SQRT_2.ln();
            dd!("46.051701859880913680359829093687287"), dd!("1e20").ln();
            dd!("69.077552789821370520539743640530926"), dd!("1e30").ln();
            dd!("-667.74967696827324836521752185847"), dd!("1e-290").ln();
        );
        assert_all_exact!(
            Double::NAN, (-Double::PI).ln();
            Double::NAN, (-Double::E).ln();
            Double::ZERO, Double::ONE.ln();
            Double::NEG_INFINITY, Double::ZERO.ln();
            Double::NAN, Double::NEG_ZERO.ln();
            Double::INFINITY, Double::INFINITY.ln();
            Double::NAN, Double::NEG_INFINITY.ln();
            Double::NAN, Double::NAN.ln();
        );
    }

    #[test]
    fn log10() {
        assert_close!(dd!("1.62324929039790046322098305657224"), dd!(42).log10());
        assert_close!(dd!("2.38560627359831218647513951627558"), dd!(243).log10());
        assert_exact!(Double::ZERO, dd!(1).log10());
        assert_close!(Double::ONE, dd!(10).log10());
    }

    #[test]
    fn log10_zero() {
        assert_exact!(Double::NAN, Double::ZERO.log10());
        assert_exact!(Double::NAN, Double::NEG_ZERO.log10());
    }

    #[test]
    fn log10_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log10());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log10());
    }

    #[test]
    fn log10_nan() {
        assert_exact!(Double::NAN, Double::NAN.log10());
    }

    #[test]
    fn log10_neg() {
        assert_exact!(Double::NAN, dd!(-1).log10());
    }

    #[test]
    fn log2() {
        assert_close!(dd!("3.32192809488736234787031942948939"), dd!(10).log2());
        assert_close!(dd!("7.92481250360578090726869471973908"), dd!(243).log2());
        assert_exact!(Double::ZERO, dd!(1).log2());
        assert_close!(Double::ONE, dd!(2).log2());
    }

    #[test]
    fn log2_zero() {
        assert_exact!(Double::NAN, Double::ZERO.log2());
        assert_exact!(Double::NAN, Double::NEG_ZERO.log2());
    }

    #[test]
    fn log2_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log2());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log2());
    }

    #[test]
    fn log2_nan() {
        assert_exact!(Double::NAN, Double::NAN.log2());
    }

    #[test]
    fn log2_neg() {
        assert_exact!(Double::NAN, dd!(-1).log2());
    }

    #[test]
    fn log() {
        assert_close!(dd!("1.17473150366718002267187494833236"), dd!(10).log(7.1));
        assert_close!(
            dd!("4.22480900593537861528922880434435"),
            dd!(243).log(3.67)
        );
        assert_exact!(Double::ZERO, dd!(1).log(6.3));
        assert_close!(Double::ONE, dd!(3.3).log(3.3));
    }

    #[test]
    fn log_zero() {
        assert_exact!(Double::NAN, Double::ZERO.log(9.2));
        assert_exact!(Double::NAN, Double::NEG_ZERO.log(1.8));
    }

    #[test]
    fn log_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log(7.3));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log(7.3));
    }

    #[test]
    fn log_nan() {
        assert_exact!(Double::NAN, Double::NAN.log(3.4));
    }

    #[test]
    fn log_neg() {
        assert_exact!(Double::NAN, dd!(-1).log(1.8));
    }
}
