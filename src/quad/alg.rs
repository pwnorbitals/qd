// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use crate::basic::*;

impl Quad {
    // A considerable simplification over simply multiplying the number by itself, with the
    // simplifications possible because the two numbers being multiplied are in fact equal.
    //
    // The result is a simpler calculation:
    //
    //      a0² + 2a0a1 + 2a0a2 + a1² + 2a0a3 + 2a1a2
    //
    // where any further terms, including the low words of the final two terms, are unnecessary to
    // achieve the desired accuracy.
    #[inline]
    pub fn sqr(self) -> Quad {
        let (h0, l0) = two_sqr(self.0);
        let (h1, l1) = two_prod(2.0 * self.0, self.1);
        let (h2, l2) = two_prod(2.0 * self.0, self.2);
        let (h3, l3) = two_sqr(self.1);
        let h4 = 2.0 * self.0 * self.3;
        let h5 = 2.0 * self.1 * self.2;

        // Less primitive functions are not used here because there are steps in them that can be
        // skipped.

        let r0 = h0;

        let (r1, a1) = two_sum(h1, l0);

        let (b0, b1) = two_sum(a1, l1);
        let (c0, c1) = two_sum(h2, h3);
        let (d0, d1) = two_sum(b0, c0);
        let (e0, e1) = two_sum(b1, c1);
        let (f0, f1) = two_sum(d1, e0);
        let (i0, i1) = quick_two_sum(f0, e1 + f1);
        let (r2, j1) = quick_two_sum(d0, i0);

        let (k0, k1) = quick_two_sum(i1, j1);
        let (m0, m1) = two_sum(h4, h5);
        let (n0, n1) = two_sum(l2, l3);
        let (o0, o1) = two_sum(m0, n0);
        let (r3, q1) = two_sum(k0, o0);

        let r4 = m1 + n1 + o1 + k1 + q1;

        Quad::from(renorm5(r0, r1, r2, r3, r4))
    }

    pub fn powi(self, n: i32) -> Quad {
        if n == 0 {
            Quad::ONE
        } else {
            let mut r = self;
            let mut s = Quad::ONE;
            let mut k = n.abs();

            if k > 1 {
                while k > 0 {
                    if k % 2 == 1 {
                        s *= r;
                    }
                    k /= 2;
                    if k > 0 {
                        r = r.sqr();
                    }
                }
            } else {
                s = r;
            }

            if n < 0 {
                s.recip()
            } else {
                s
            }
        }
    }
}
