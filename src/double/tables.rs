// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

/// Reciprocals of factorials, rendered as Doubles. These are used in Taylor series
/// calculations.
pub const INV_FACTS: [Double; 15] = [
    Double(1.6666666666666666e-1, 9.25185853854297e-18),
    Double(4.1666666666666664e-2, 2.3129646346357427e-18),
    Double(8.333333333333333e-3, 1.1564823173178714e-19),
    Double(1.388888888888889e-3, -5.300543954373577e-20),
    Double(1.984126984126984e-4, 1.7209558293420705e-22),
    Double(2.48015873015873e-5, 2.1511947866775882e-23),
    Double(2.7557319223985893e-6, -1.858393274046472e-22),
    Double(2.755731922398589e-7, 2.3767714622250297e-23),
    Double(2.505210838544172e-8, -1.448814070935912e-24),
    Double(2.08767569878681e-9, -1.20734505911326e-25),
    Double(1.6059043836821613e-10, 1.2585294588752098e-26),
    Double(1.1470745597729725e-11, 2.0655512752830745e-28),
    Double(7.647163731819816e-13, 7.03872877733453e-30),
    Double(4.779477332387385e-14, 4.399205485834081e-31),
    Double(2.8114572543455206e-15, 1.6508842730861433e-31),
];

/// Coefficients of a Remez-algorithm-generated polynomial that simulates exp(x) where x is
/// in the range [0, 0.25]. The first element is the constant term, so the element at index
/// *i* will be the coefficient for the x^i term.
pub const COEFFS_EXP: [Double; 17] = [
    Double(1e0, 5.700752635386218e-32),
    Double(1e0, -5.546678239835239e-32),
    Double(5e-1, 1.9132958439520556e-29),
    Double(1.6666666666666666e-1, 9.251858535636339e-18),
    Double(4.1666666666666664e-2, 2.3129648669590502e-18),
    Double(8.333333333333333e-3, 1.1563696712831638e-19),
    Double(1.388888888888889e-3, -5.26453518972086e-20),
    Double(1.984126984126984e-4, -7.827423575810297e-21),
    Double(2.480158730158743e-5, -9.088510300188971e-22),
    Double(2.755731922397088e-6, -1.6537254970616428e-22),
    Double(2.7557319225298207e-7, -1.2646289355573392e-23),
    Double(2.5052108299753353e-8, 1.5117167551423136e-24),
    Double(2.087676114728279e-9, 1.491311291313656e-25),
    Double(1.605889602605414e-10, -6.721704574528207e-27),
    Double(1.1474475261065964e-11, -6.833152708496626e-28),
    Double(7.584053348445579e-13, 4.724366315262144e-29),
    Double(5.417171084430837e-14, 2.1022980583719787e-30),
];

// Table of sin(kπ/16), for k in [1, 4]
pub const SINES: [Double; 4] = [
    Double(1.9509032201612828e-1, -7.991079068461734e-18),
    Double(3.826834323650898e-1, -1.005077269646159e-17),
    Double(5.555702330196022e-1, 4.7094109405616756e-17),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];

// Table of cos(kπ/16), for k in [1, 4]
pub const COSINES: [Double; 4] = [
    Double(9.807852804032304e-1, 1.8546939997824996e-17),
    Double(9.238795325112867e-1, 1.764504708433667e-17),
    Double(8.314696123025452e-1, 1.4073856984728008e-18),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];
