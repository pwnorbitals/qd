// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use crate::common::basic::renorm4;

// #region Miscellaneous mathematical operations

impl Quad {
    #[inline]
    pub fn abs(self) -> Quad {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    #[inline]
    pub fn floor(self) -> Quad {
        let a = self.0.floor();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;

        if a == self.0 {
            b = self.1.floor();
            if b == self.1 {
                c = self.2.floor();
                if c == self.2 {
                    d = self.3.floor();
                }
            }
            Quad::from(renorm4(a, b, c, d))
        } else {
            Quad(a, b, c, d)
        }
    }

    #[inline]
    pub fn ceil(self) -> Quad {
        let a = self.0.ceil();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;

        if a == self.0 {
            b = self.1.ceil();
            if b == self.1 {
                c = self.2.ceil();
                if c == self.2 {
                    d = self.3.ceil();
                }
            }
            Quad::from(renorm4(a, b, c, d))
        } else {
            Quad(a, b, c, d)
        }
    }
}

// #endregion

// #region Number properties

impl Quad {
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    #[inline]
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    #[inline]
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }

    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite() || self.1.is_infinite() || self.2.is_infinite() || self.3.is_infinite()
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite() && self.1.is_finite() && self.2.is_finite() && self.3.is_finite()
    }
}

// #endregion
