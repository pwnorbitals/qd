// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::utils as u;
use crate::quad::Quad;
use std::ops::{Add, AddAssign};

// Utility function that returns the quad component with the specified index and then
// increments the index. This is how we do `a[i++]` without the `++` operator.
#[inline]
fn index_and_inc(a: Quad, i: &mut usize) -> f64 {
    let r = a[*i];
    *i += 1;
    r
}

impl Add for Quad {
    type Output = Quad;

    // This function is the real reason indexing was added to quads. Unlike multiplication,
    // where every component has a specific function and appears in a specific place in the
    // algorithm, addition is just a repeated iteration over each successive component.

    /// Adds this `Quad` to another, producing a new `Quad` as a result.
    ///
    /// This implements the `+` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E + Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl, clippy::many_single_char_names)]
    fn add(self, other: Quad) -> Quad {
        match self.pre_add(&other) {
            Some(r) => r,
            None => {
                let mut i = 0;
                let mut j = 0;
                let mut k = 0;

                let mut x = [0.0; 4];

                // These two assignments, along with the reassignments of the same variables
                // in the `accumulate` call below, act as a merge sort. The largest
                // component between the two quads is operated on first, then the second
                // largest, and so on.
                let u = if self[i].abs() > other[j].abs() {
                    index_and_inc(self, &mut i)
                } else {
                    index_and_inc(other, &mut j)
                };
                let v = if self[i].abs() > other[j].abs() {
                    index_and_inc(self, &mut i)
                } else {
                    index_and_inc(other, &mut j)
                };
                let (mut u, mut v) = u::renorm2(u, v);

                while k < 4 {
                    if i >= 4 && j >= 4 {
                        x[k] = u;
                        if k < 3 {
                            k += 1;
                            x[k] = v;
                        }
                        break;
                    }

                    let t = if i >= 4 {
                        index_and_inc(other, &mut j)
                    } else if j >= 4 || self[i].abs() > other[j].abs() {
                        index_and_inc(self, &mut i)
                    } else {
                        index_and_inc(other, &mut j)
                    };

                    let (s, y, z) = u::accumulate(u, v, t);
                    u = y;
                    v = z;

                    if s != 0.0 {
                        x[k] = s;
                        k += 1;
                    }
                }

                for k in i..4 {
                    x[3] += self[k];
                }
                for k in j..4 {
                    x[3] += other[k];
                }
                let (a, b, c, d) = u::renorm4(x[0], x[1], x[2], x[3]);
                Quad(a, b, c, d)
            }
        }
    }
}

impl Add for &Quad {
    type Output = Quad;

    /// Adds a reference to this `Quad` to another, producing a new `Quad` as a result.
    ///
    /// This implements the `+` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E + &Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn add(self, other: &Quad) -> Quad {
        (*self).add(*other)
    }
}

impl Add<&Quad> for Quad {
    type Output = Quad;

    /// Adds this `Quad` to a reference to another, producing a new `Quad` as a result.
    ///
    /// This implements the `+` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E + &Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn add(self, other: &Quad) -> Quad {
        self.add(*other)
    }
}

impl Add<Quad> for &Quad {
    type Output = Quad;

    /// Adds a reference to this `Quad` to another `Quad`, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the `+` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E + Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn add(self, other: Quad) -> Quad {
        (*self).add(other)
    }
}

impl AddAssign for Quad {
    /// Adds another `Quad` to this one, modifying this one to equal the result.
    ///
    /// This implements the `+=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x += Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Quad) {
        let r = self.add(other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl AddAssign<&Quad> for Quad {
    /// Adds a reference to another `Quad` to this one, modifying this one to equal the
    /// result.
    ///
    /// This implements the `+=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x += &Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Quad) {
        let r = self.add(*other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl Quad {
    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_add(&self, other: &Quad) -> Option<Quad> {
        if self.is_nan() || other.is_nan() {
            Some(Quad::NAN)
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Some(Quad::INFINITY)
                    } else {
                        Some(Quad::NAN)
                    }
                } else if other.is_sign_negative() {
                    Some(Quad::NEG_INFINITY)
                } else {
                    Some(Quad::NAN)
                }
            } else if self.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
        assert_close!(expected, Quad::PI + Quad::E);
    }

    #[test]
    fn ref_ref() {
        let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
        assert_close!(expected, &Quad::PI + &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
        assert_close!(expected, Quad::PI + &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
        assert_close!(expected, &Quad::PI + Quad::E);
    }

    #[test]
    fn assign_num() {
        let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");

        let mut a = Quad::PI;
        a += Quad::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");

        let mut b = Quad::PI;
        b += &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY + Quad::ONE);
        assert_exact!(Quad::INFINITY, Quad::ONE + Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY + Quad::ONE);
        assert_exact!(Quad::NEG_INFINITY, Quad::ONE + Quad::NEG_INFINITY);
    }

    #[test]
    fn infinities() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY + Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY + Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY + Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY + Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY + Quad::NAN);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY + Quad::NAN);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN + Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE + Quad::NAN);
    }
}