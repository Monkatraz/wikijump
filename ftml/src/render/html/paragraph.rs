/*
 * render/html/paragraph.rs
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

use self::Paragraph::*;
use super::prelude::*;
use crate::enums::Alignment;

pub fn render_paragraphs<'a, I, L>(ctx: &mut HtmlContext, paragraphs: I) -> Result<()>
where
    L: AsRef<Paragraph<'a>>,
    I: IntoIterator<Item = L>,
    I::IntoIter: ExactSizeIterator,
{
    for paragraph in paragraphs.into_iter() {
        render_paragraph(ctx, paragraph.as_ref())?;
    }

    Ok(())
}

// TODO remove this
#[allow(unused_variables)]
pub fn render_paragraph(ctx: &mut HtmlContext, paragraph: &Paragraph) -> Result<()> {
    match paragraph {
        &Align {
            alignment,
            ref paragraphs,
        } => {
            write!(ctx, "<div style=\"text-align: {};\">\n", alignment)?;
            render_paragraphs(ctx, paragraphs)?;
            ctx.push_str("</div>");
        }
        &Center { ref words } => {
            ctx.push_str("<div style=\"text-align: center;\">\n");
            render_words(ctx, words)?;
            ctx.push_str("</div>");
        }
        &ClearFloat { direction } => {
            let style = match direction {
                Some(Alignment::Left) => "left",
                Some(Alignment::Right) => "right",
                Some(direction) => panic!("Invalid case for ClearFloat: {:?}", direction),
                None => "both",
            };

            write!(ctx, r#"<div style="clear: {}; height: 0;"></div>"#, style)?;
        }
        &CodeBlock {
            ref language,
            ref contents,
        } => {
            // TODO add language highlighting
            let _ = language;

            ctx.push_str("<code>\n");
            escape_html(ctx, contents)?;
            ctx.push_str("</code>\n");
        }
        &Collapsible {
            ref show_text,
            ref hide_text,
            ref id,
            ref class,
            ref style,
            show_top,
            show_bottom,
            ref paragraphs,
        } => unimplemented!(),
        &Div {
            id,
            class,
            style,
            ref paragraphs,
        } => {
            ctx.push_str("<div");

            if let Some(id) = id {
                write!(ctx, " id={}", id)?;
            }

            if let Some(class) = class {
                write!(ctx, " class={}", class)?;
            }

            if let Some(style) = style {
                write!(ctx, " style={}", style)?;
            }

            ctx.push_str(">\n");
            render_paragraphs(ctx, paragraphs)?;
            ctx.push_str("\n</div>");
        }
        &Heading { level, ref words } => {
            write!(ctx, "<{}>", level)?;
            render_words(ctx, words)?;
            write!(ctx, "</{}>\n", level)?;
        }
        &HorizontalLine => ctx.push_str("<hr>\n"),
        &Html { contents } => ctx.push_str(contents),
        &Iframe { url, ref arguments } => {
            write!(ctx, "<iframe src=\"{}\"", url)?;

            for (key, value) in arguments {
                write_tag_arg(ctx, key, value)?;
            }

            ctx.push_str("></iframe>");
        }
        &IfTags {
            ref required,
            ref prohibited,
            ref paragraphs,
        } => {
            // Not sure what the approach on this should be
            let tags = &ctx.info().tags;
            let should_display = move || {
                for tag in required {
                    if !tags.contains(tag) {
                        return false;
                    }
                }

                for tag in prohibited {
                    if tags.contains(tag) {
                        return false;
                    }
                }

                true
            };

            if should_display() {
                render_paragraphs(ctx, paragraphs)?;
            }
        }
        &Javascript { contents } => {
            write!(ctx, "<script>\n{}\n</script>", contents)?;
        }
        &List {
            style,
            depth,
            ref items,
        } => {
            // TODO will need to collect nearby entries for depth
            let _ = depth;

            write!(ctx, "<{}>\n", style)?;
            for item in items {
                ctx.push_str("<li> ");
                render_paragraph(ctx, item)?;
                ctx.push_str(" </li>\n");
            }
            write!(ctx, "</{}>", style)?;
        }
        &Math {
            label,
            id,
            latex_env,
            expr,
        } => {
            // TODO do LaTeX rendering
            // use mathjax library
            unimplemented!()
        }
        &Newlines { count } => {
            for _ in 0..count {
                ctx.push_str("<br>");
            }

            ctx.push('\n');
        }
        &Table { ref rows } => {
            ctx.push_str("<table>\n");
            for row in rows {
                let (start_tag, end_tag) = match row.title {
                    true => ("<th>", "</th>\n"),
                    false => ("<td>", "</td>\n"),
                };

                ctx.push_str("<tr>\n");
                for column in &row.columns {
                    ctx.push_str(start_tag);
                    render_words(ctx, column)?;
                    ctx.push_str(end_tag);
                }
                ctx.push_str("</tr>\n");
            }
            ctx.push_str("</table>\n");
        }
        &TableOfContents {} => {
            // TODO
            unimplemented!()
        }
        QuoteBlock {
            id,
            class,
            style,
            ref paragraphs,
        } => {
            ctx.push_str("<blockquote");

            if let Some(id) = id {
                write!(ctx, " id={}", id)?;
            }

            if let Some(class) = class {
                write!(ctx, " class={}", class)?;
            }

            if let Some(style) = style {
                write!(ctx, " style={}", style)?;
            }

            ctx.push_str(">\n");
            render_paragraphs(ctx, paragraphs)?;
            ctx.push_str("\n</blockquote>");
        }
        Words {
            centered,
            ref words,
        } => {
            ctx.push_str("<p");
            if *centered {
                ctx.push_str(" style=\"text-align: center;\"");
            }
            ctx.push('>');
            render_words(ctx, words)?;
            ctx.push_str("</p>\n");
        }
    }

    Ok(())
}