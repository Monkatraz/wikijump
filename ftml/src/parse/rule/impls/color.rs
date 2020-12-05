/*
 * parse/rule/impls/color.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;

pub const RULE_COLOR: Rule = Rule {
    name: "color",
    try_consume_fn,
};

fn try_consume_fn<'t, 'r>(
    log: &slog::Logger,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    _full_text: FullText<'t>,
) -> Consumption<'t, 'r> {
    debug!(log, "Trying to create color container");

    assert_eq!(extracted.token, Token::Color, "Current token isn't '##'");

    /*
    try_container(
        log,
        extracted,
        remaining,
        (RULE_BOLD, ContainerType::Bold),
        (Token::Bold, Token::Bold),
        &[Token::ParagraphBreak, Token::InputEnd],
        &[
            (Token::Bold, Token::Whitespace),
            (Token::Whitespace, Token::Bold),
        ],
    )
    */

    todo!()
}