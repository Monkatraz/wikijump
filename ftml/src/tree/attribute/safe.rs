/*
 * tree/attribute/safe.rs
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

use std::collections::HashSet;
use unicase::UniCase;

macro_rules! hashset_unicase {
    () => {
        hashset![]
    };

    ($($x:expr),+ $(,)?) => {
        hashset![
            $(
                UniCase::ascii($x)
            ),+
        ]
    };
}

lazy_static! {
    /// List of safe attributes. All others will be filtered out.
    ///
    /// See https://scuttle.atlassian.net/wiki/spaces/WD/pages/1030782977/Allowed+Attributes+in+Wikitext
    pub static ref SAFE_ATTRIBUTES: HashSet<UniCase<&'static str>> = {
        hashset_unicase![
            "accept",
            "align",
            "alt",
            "autocapitalize",
            "autoplay",
            "background",
            "bgcolor",
            "border",
            "buffered",
            "checked",
            "cite",
            "class",
            "cols",
            "colspan",
            "contenteditable",
            "controls",
            "coords",
            "datetime",
            "decoding",
            "default",
            "dir",
            "dirname",
            "disabled",
            "download",
            "draggable",
            "for",
            "form",
            "headers",
            "height",
            "hidden",
            "high",
            "href",
            "hreflang",
            "id",
            "inputmode",
            "ismap",
            "itemprop",
            "kind",
            "label",
            "lang",
            "list",
            "loop",
            "low",
            "max",
            "maxlength",
            "min",
            "minlength",
            "multiple",
            "muted",
            "name",
            "optimum",
            "pattern",
            "placeholder",
            "poster",
            "preload",
            "readonly",
            "rel",
            "required",
            "reversed",
            "rows",
            "rowspan",
            "sandbox",
            "scope",
            "selected",
            "shape",
            "size",
            "sizes",
            "span",
            "spellcheck",
            "src",
            "srclang",
            "srcset",
            "start",
            "step",
            "style",
            "tabindex",
            "target",
            "title",
            "translate",
            "type",
            "usemap",
            "value",
            "width",
            "wrap",
        ]
    };
}

pub const SAFE_ATTRIBUTE_PREFIXES: [&str; 1] = ["data-"];

pub fn is_safe_attribute(attribute: UniCase<&str>) -> bool {
    if SAFE_ATTRIBUTES.contains(&attribute) {
        return true;
    }

    for prefix in &SAFE_ATTRIBUTE_PREFIXES {
        if attribute.starts_with(prefix) {
            return true;
        }
    }

    false
}
