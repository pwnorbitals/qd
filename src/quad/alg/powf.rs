// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the number raised to a quad-Quad power.
    ///
    /// This function is implemented using the logarithm of the number being raised, which
    /// means it will not work for negatives even though raising a negative number to a
    /// non-integer power is defined. It has been extended to handle zero in accordance with
    /// the IEEE 754 specification.
    ///
    /// It is possible that a new algorithm will eventually remove this restriction, though
    /// this is a surprisingly hard problem (see [this libm implementation][1], for
    /// example).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3).powf(qd!(3.3));
    /// let expected = qd!("37.54050759852955219310186595463382927684873090166843452920390518");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [1]: http://www.netlib.org/fdlibm/e_pow.c
    #[inline]
    pub fn powf(self, n: Quad) -> Quad {
        if self.is_zero() {
            if n.is_zero() {
                Quad::NAN
            } else if n.is_sign_positive() {
                Quad::ZERO
            } else {
                Quad::INFINITY
            }
        } else if n.is_infinite() {
            if self == Quad::ONE {
                Quad::NAN
            } else if n.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::ZERO
            }
        } else {
            (n * self.ln()).exp()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powf() {
        assert_close!(
            qd!("24567.24805421478199532529771567617705237167216222778116359595012"),
            qd!(11.1).powf(qd!(4.2))
        );
        assert_close!(
            qd!("1.409759279075053716836003243441716711042960485535248677014414790"),
            Quad::PI.powf(qd!(0.3))
        );
        assert_close!(
            qd!("0.006810719380166276826846127381721218763394637801309025289387144601"),
            qd!(0.2).powf(qd!(3.1))
        );
        assert_close!(
            qd!("146.8273678860023757393079582114873627092153773446718337101982774"),
            qd!(0.2).powf(qd!(-3.1))
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.powf(qd!(3)));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.powf(qd!(3)));
        assert_exact!(Quad::ZERO, Quad::ZERO.powf(Quad::INFINITY));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.powf(Quad::INFINITY));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powf(qd!(-2)));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.powf(qd!(-2)));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powf(Quad::NEG_INFINITY));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.powf(Quad::NEG_INFINITY));
    }

    #[test]
    fn zero_exponent() {
        assert_exact!(Quad::ONE, qd!(2).powf(Quad::ZERO));
        assert_exact!(Quad::ONE, qd!(2).powf(Quad::NEG_ZERO));
        assert_exact!(Quad::NAN, Quad::ZERO.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::ZERO.powf(Quad::NEG_ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.powf(Quad::NEG_ZERO));
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::NAN, Quad::INFINITY.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::INFINITY.powf(Quad::NEG_ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.powf(Quad::NEG_ZERO));
    }

    #[test]
    fn infinite_exponent() {
        assert_exact!(Quad::INFINITY, qd!(2).powf(Quad::INFINITY));
        assert_exact!(Quad::ZERO, qd!(2).powf(Quad::NEG_INFINITY));
        assert_exact!(Quad::NAN, qd!(1).powf(Quad::INFINITY));
        assert_exact!(Quad::NAN, qd!(1).powf(Quad::NEG_INFINITY));
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.powf(qd!(3)));
        assert_exact!(Quad::NAN, qd!(3).powf(Quad::NAN));
    }

    #[test]
    fn negative() {
        assert_exact!(Quad::NAN, qd!(-1).powf(qd!(1)));
    }
}
