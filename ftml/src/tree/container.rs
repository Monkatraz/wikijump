/*
 * tree/container.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
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

//! Representation of generic syntax elements which wrap other elements.

use super::{AttributeMap, Element, HeadingLevel, HtmlTag};
use strum_macros::IntoStaticStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Container<'t> {
    #[serde(rename = "type")]
    ctype: ContainerType,
    elements: Vec<Element<'t>>,
    attributes: AttributeMap<'t>,
}

impl<'t> Container<'t> {
    #[inline]
    pub fn new(
        ctype: ContainerType,
        elements: Vec<Element<'t>>,
        attributes: AttributeMap<'t>,
    ) -> Self {
        Container {
            ctype,
            elements,
            attributes,
        }
    }

    #[inline]
    pub fn ctype(&self) -> ContainerType {
        self.ctype
    }

    #[inline]
    pub fn elements(&self) -> &[Element<'t>] {
        &self.elements
    }

    #[inline]
    pub fn attributes(&self) -> &AttributeMap<'t> {
        &self.attributes
    }
}

impl<'t> From<Container<'t>> for Vec<Element<'t>> {
    #[inline]
    fn from(container: Container<'t>) -> Vec<Element<'t>> {
        let Container { elements, .. } = container;

        elements
    }
}

#[derive(
    Serialize, Deserialize, IntoStaticStr, Debug, Copy, Clone, Hash, PartialEq, Eq,
)]
#[serde(rename_all = "kebab-case")]
pub enum ContainerType {
    Bold,
    Italics,
    Underline,
    Superscript,
    Subscript,
    Strikethrough,
    Monospace,
    Span,
    Div,
    Mark,
    Blockquote,
    Insertion,
    Deletion,
    Hidden,
    Invisible,
    Size,
    Paragraph,
    Header(HeadingLevel),
}

impl ContainerType {
    #[inline]
    pub fn name(self) -> &'static str {
        self.into()
    }

    #[inline]
    pub fn html_tag(self) -> HtmlTag {
        match self {
            ContainerType::Bold => HtmlTag::new("strong"),
            ContainerType::Italics => HtmlTag::new("italics"),
            ContainerType::Underline => HtmlTag::new("u"),
            ContainerType::Superscript => HtmlTag::new("sup"),
            ContainerType::Subscript => HtmlTag::new("sub"),
            ContainerType::Strikethrough => HtmlTag::new("s"),
            ContainerType::Monospace => HtmlTag::new("tt"),
            ContainerType::Span => HtmlTag::new("span"),
            ContainerType::Div => HtmlTag::new("div"),
            ContainerType::Mark => HtmlTag::new("mark"),
            ContainerType::Blockquote => HtmlTag::new("blockquote"),
            ContainerType::Insertion => HtmlTag::new("ins"),
            ContainerType::Deletion => HtmlTag::new("del"),
            ContainerType::Hidden => HtmlTag::with_class("span", "hidden"),
            ContainerType::Invisible => HtmlTag::with_class("span", "invisible"),
            ContainerType::Size => HtmlTag::new("span"),
            ContainerType::Paragraph => HtmlTag::new("p"),
            ContainerType::Header(level) => HtmlTag::new(level.html_tag()),
        }
    }
}

impl slog::Value for ContainerType {
    fn serialize(
        &self,
        _: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_str(key, self.name())
    }
}
