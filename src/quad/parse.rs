// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use crate::error::{ErrorKind, ParseError};
use std::str::FromStr;

const TEN: Quad = Quad(10.0, 0.0, 0.0, 0.0);

// #region Parsing

impl FromStr for Quad {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Quad, ParseError> {
        let mut result = Quad::ZERO;
        let mut digits = 0;
        let mut point = -1;
        let mut sign = 0;
        let mut exp = 0;

        let s = s.trim();

        if s.is_empty() {
            return Err(ParseError {
                kind: ErrorKind::Empty,
            });
        }

        if s.to_ascii_lowercase() == "nan" {
            return Ok(Quad::NAN);
        }
        if s.to_ascii_lowercase() == "inf" {
            return Ok(Quad::INFINITY);
        }
        if s.to_ascii_lowercase() == "-inf" {
            return Ok(Quad::NEG_INFINITY);
        }

        for (index, ch) in s.chars().enumerate() {
            match ch.to_digit(10) {
                Some(d) => {
                    result *= TEN;
                    result += Quad(d as f64, 0.0, 0.0, 0.0);
                    digits += 1;
                }
                None => match ch {
                    '.' => {
                        if point >= 0 {
                            return Err(ParseError {
                                kind: ErrorKind::Invalid,
                            });
                        }
                        point = digits;
                    }
                    '-' => {
                        if sign != 0 || digits > 0 {
                            return Err(ParseError {
                                kind: ErrorKind::Invalid,
                            });
                        }
                        sign = -1;
                    }
                    '+' => {
                        if sign != 0 || digits > 0 {
                            return Err(ParseError {
                                kind: ErrorKind::Invalid,
                            });
                        }
                        sign = 1;
                    }
                    'e' | 'E' => {
                        let end = &s[(index + 1)..];
                        match end.parse::<i32>() {
                            Ok(e) => {
                                exp = e;
                                break;
                            }
                            Err(_) => {
                                return Err(ParseError {
                                    kind: ErrorKind::Invalid,
                                });
                            }
                        }
                    }
                    '_' => {
                        // just continue; _ is a no-op but not an error
                    }
                    _ => {
                        return Err(ParseError {
                            kind: ErrorKind::Invalid,
                        });
                    }
                },
            }
        }

        if point >= 0 {
            exp -= digits - point;
        }
        if exp != 0 {
            // Do this in two stages if the exponent is too small
            // For exmaple, a number with 30 digits could have an exponent as low as -337 and
            // still not overflow, but doing the -337 all at once WOULD overflow
            if exp < -307 {
                let adjust = exp + 307;
                result *= TEN.powi(adjust);
                exp -= adjust;
            }
            result *= TEN.powi(exp);
        }
        if sign == -1 {
            result = -result;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pi() {
        let pi = "2.302585092994045684017991454684364207601101488628772976033327900967573";
        let pi_q = pi.parse::<Quad>().unwrap();
        println!("Quad({:e}, {:e}, {:e}, {:e});", pi_q.0, pi_q.1, pi_q.2, pi_q.3);
    }
}
