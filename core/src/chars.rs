//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! Special characters used within Mnemosyne

/// Unicode code point for the lambda character
pub const LAMBDA: &'static str      = "\u{03bb}";
/// Unicode code point for the arrow character
pub const ARROW: &'static str       = "\u{2192}";
/// Unicode code point for the fat arrow (typeclass) character.
pub const FAT_ARROW: &'static str   = "\u{21D2}";

pub const ALPHA_EXT: &'static str   = "+-*/<=>!:%^";
pub const ALPHA_EXT_SUBSEQUENT: &'static str = "+-*/<=>!:%^'";
pub const OPS: &'static str         = "+-*/|=<>";
