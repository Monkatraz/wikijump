/*
 * render/html/html.rs
 *
 * ftml - Convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith
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

use super::{ComponentRender, HtmlContext};
use crate::Result;
use std::fmt::Write;

#[derive(Debug)]
pub struct HtmlBuilder<'c, 'i, 'h> {
    ctx: &'c mut HtmlContext<'i, 'h>,
}

impl<'c, 'i, 'h> HtmlBuilder<'c, 'i, 'h> {
    #[inline]
    pub fn new(ctx: &'c mut HtmlContext<'i, 'h>) -> Self {
        HtmlBuilder { ctx }
    }

    #[inline]
    pub fn tag<'t>(self, tag: &'t str) -> HtmlBuilderTag<'c, 'i, 'h, 't> {
        debug_assert!(is_alphanumeric(tag));

        let HtmlBuilder { ctx } = self;
        HtmlBuilderTag::new(ctx, tag)
    }
}

#[derive(Debug)]
pub struct HtmlBuilderTag<'c, 'i, 'h, 't> {
    ctx: &'c mut HtmlContext<'i, 'h>,
    tag: &'t str,
    in_tag: bool,
}

impl<'c, 'i, 'h, 't> HtmlBuilderTag<'c, 'i, 'h, 't> {
    pub fn new(ctx: &'c mut HtmlContext<'i, 'h>, tag: &'t str) -> Self {
        ctx.push('<');
        ctx.push_str(tag);

        HtmlBuilderTag {
            ctx,
            tag,
            in_tag: true,
        }
    }

    #[inline]
    pub fn attr(&mut self, key: &str, value: &str) -> Result<()> {
        debug_assert!(is_alphanumeric(key));
        debug_assert!(self.in_tag);

        // TODO add html escaping
        write!(self.ctx, " {}=\"{}\"", key, value)?;
        Ok(())
    }

    #[inline]
    pub fn contents(&mut self, component: &dyn ComponentRender) -> Result<()> {
        self.in_tag = true;
        component.render(self.ctx)
    }

    pub fn end(self) {
        if !self.in_tag {
            self.ctx.push_str("</");
            self.ctx.push_str(self.tag);
        }

        self.ctx.push('>');
    }
}

fn is_alphanumeric(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '-')
}