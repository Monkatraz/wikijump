/*
 * render/html/word.rs
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

use self::Word::*;
use super::module;
use super::prelude::*;
use crate::enums::LinkText;

impl<'a, 'w> ComponentRender for &'a [Word<'w>] {
    fn render(&self, ctx: &mut HtmlContext) -> Result<()> {
        for word in *self {
            word.render(ctx)?;
        }

        Ok(())
    }
}

impl<'a, 'w> ComponentRender for &'a Vec<Word<'w>> {
    #[inline]
    fn render(&self, ctx: &mut HtmlContext) -> Result<()> {
        self.as_slice().render(ctx)
    }
}

// TODO remove this stub
pub fn render_words(ctx: &mut HtmlContext, words: &[Word]) -> Result<()> {
    words.render(ctx)
}

// TODO remove this stub
#[inline]
pub fn render_word(ctx: &mut HtmlContext, word: &Word) -> Result<()> {
    word.render(ctx)
}

// TODO remove this lint
#[allow(unused_variables)]
impl<'w> ComponentRender for Word<'w> {
    fn render(&self, ctx: &mut HtmlContext) -> Result<()> {
        match self {
            &Anchor {
                ref href,
                ref name,
                ref id,
                ref class,
                ref style,
                ref target,
                ref words,
            } => {
                ctx.push_str("<a");

                if let Some(href) = href {
                    write!(ctx, " href=\"{}\"", percent_encode(href))?;
                }

                if let Some(name) = name {
                    write!(ctx, " name=\"{}\"", name)?;
                }

                if let Some(id) = id {
                    write!(ctx, " id=\"{}\"", id)?;
                }

                if let Some(class) = class {
                    write!(ctx, " class=\"{}\"", class)?;
                }

                if let Some(style) = style {
                    write!(ctx, " style=\"{}\"", style)?;
                }

                if let Some(target) = target {
                    write!(ctx, " target=\"{}\"", target)?;
                }

                ctx.push('>');
                render_words(ctx, words)?;
                ctx.push_str("</a>");
            }
            &Link { href, target, text } => {
                write!(ctx, "<a href=\"{}\"", percent_encode(href))?;

                if let Some(target) = target {
                    write!(ctx, " target=\"{}\"", target)?;
                }

                let text = match text {
                    LinkText::Article => &ctx.info().title,
                    LinkText::Text(text) => text,
                    LinkText::Url => href,
                };

                ctx.push('>');
                escape_html(ctx, text)?;
                ctx.push_str("</a>");
            }
            &Bold { ref words } => {
                ctx.html().b().contents(&words)?.end();
            }
            &Color { color, ref words } => {
                ctx.push_str("<span style=\"color: ");
                escape_attr(ctx, color)?;
                ctx.push_str("\">");
                render_words(ctx, words)?;
                ctx.push_str("</span>");
            }
            &Css { style } => ctx.add_style(style),
            &Date { timestamp, format } => {
                use chrono::prelude::*;

                // For now timestamp can only be seconds since the epoch.
                // In the future we may want to extend this to also include ISO timestamps and such.

                let date = NaiveDateTime::from_timestamp(timestamp, 0);
                let format = format.unwrap_or("%e");
                let (format, hover) = if format.ends_with("|agohover") {
                    let new_len = format.len() - "|agohover".len();

                    (&format[..new_len], true)
                } else {
                    (format, false)
                };

                // TODO actually add the hover thing
                let _ = hover;

                write!(ctx, "{}", date.format(format))?;
            }
            &Email { address, text } => {
                write!(ctx, "<a href=\"mailto:{}\">", address)?;
                escape_html(ctx, text.unwrap_or(address))?;
                ctx.push_str("</a>");
            }
            &EquationReference { name } => {
                // TODO
                return Err(Error::StaticMsg(
                    "Rendering for equation references are not implemented yet",
                ));
            }
            &File {
                filename,
                text,
                target,
            } => {
                write!(ctx, "<a href=\"{}\"", percent_encode(filename))?;

                if let Some(target) = target {
                    write!(ctx, " target=\"{}\"", target)?;
                }

                let text = match text {
                    Some("") | None => filename,
                    Some(text) => text,
                };

                ctx.push('>');
                escape_html(ctx, text)?;
                ctx.push_str("</a>");
            }
            &Footnote { ref paragraphs } => {
                // TODO add javascript
                let number = ctx.footnotes_mut().incr();
                ctx.push_str("<sup class=\"footnoteref\">");
                write!(
                    ctx,
                    stringify!(
                        "<a id=\"footnote-{0}\" class=\"footnoteref\" ",
                        "onclick=\"scrollToFootnote('footnote-{0}')\">",
                        "{0}",
                        "</a>",
                    ),
                    number
                )?;
                ctx.push_str("</sup>");

                ctx.write_footnote_block(|ctx| {
                    ctx.push_str("<li>");
                    render_paragraphs(ctx, paragraphs)?;
                    ctx.push_str("</li>");

                    Ok(())
                })?;
            }
            &FootnoteBlock => {
                ctx.footnotes_mut().set_block(true);
                ctx.push_str("\0footnote-block\0");
            }
            &Form { contents } => {
                // TODO
                return Err(Error::StaticMsg(
                    "Rendering for forms is not implemented yet",
                ));
            }
            &Gallery => {
                // TODO
                return Err(Error::StaticMsg(
                    "Rendering for galleries is not implemented yet",
                ));
            }
            &Image {
                filename,
                float,
                direction,
                link,
                ref alt,
                ref title,
                ref width,
                ref height,
                ref style,
                ref class,
                ref size,
            } => {
                ctx.push_str("<div class=\"image-container\"");
                if let Some(align) = direction {
                    write!(ctx, " style=\"text-align: {};\"", align)?;
                }
                ctx.push_str("><img");

                // TODO adjust for other sources
                write_tag_arg(ctx, "src", filename)?;

                // TODO float

                if let Some(alt) = alt {
                    write!(ctx, " alt={}", alt)?;
                }

                // TODO title

                if let Some(width) = width {
                    write!(ctx, " width={}", width)?;
                }

                if let Some(height) = height {
                    write!(ctx, " height={}", height)?;
                }

                if let Some(style) = style {
                    write!(ctx, " style={}", style)?;
                }

                if let Some(class) = class {
                    write!(ctx, " class={}", class)?;
                }

                if let Some(size) = size {
                    write!(ctx, " size={}", size)?;
                }

                ctx.push_str("></img></div>");
            }
            &Italics { ref words } => {
                ctx.push_str("<i>");
                render_words(ctx, words)?;
                ctx.push_str("</i>");
            }
            &Math { expr } => {
                // TODO
                return Err(Error::StaticMsg(
                    "Rendering for inline mathematical expressions is not implemented yet",
                ));
            }
            &Module {
                name,
                ref arguments,
                contents,
            } => module::render(name, ctx, arguments, contents)?,
            &Monospace { ref words } => {
                ctx.html().tt().contents(&words)?.end();
            }
            &Note { ref paragraphs } => {
                ctx.html()
                    .div()
                    .attr("class", &["wiki-note"])?
                    .contents(&paragraphs)?
                    .end();
            }
            &Raw { contents } => ctx.html().text(contents),
            &Size {
                size,
                ref paragraphs,
            } => {
                ctx.html()
                    .span()
                    .attr("style", &["size: ", size])?
                    .contents(&paragraphs)?
                    .end();
            }
            &Span {
                ref id,
                ref class,
                ref style,
                ref paragraphs,
            } => {
                ctx.push_str("<span");

                if let Some(id) = id {
                    write!(ctx, " id={}", id)?;
                }

                if let Some(class) = class {
                    write!(ctx, " class={}", class)?;
                }

                if let Some(style) = style {
                    write!(ctx, " style={}", style)?;
                }

                ctx.push('>');
                render_paragraphs(ctx, paragraphs)?;
                ctx.push_str("</span>");
            }
            &Strikethrough { ref words } => {
                ctx.push_str("<strike>");
                render_words(ctx, words)?;
                ctx.push_str("</strike>");
            }
            &Subscript { ref words } => {
                ctx.push_str("<sub>");
                render_words(ctx, words)?;
                ctx.push_str("</sub>");
            }
            &Superscript { ref words } => {
                ctx.push_str("<sup>");
                render_words(ctx, words)?;
                ctx.push_str("</sup>");
            }
            &TabList { ref tabs } => {
                // TODO
                return Err(Error::StaticMsg(
                    "Rendering for tab lists is not implemented",
                ));
            }
            &Text { contents } => escape_html(ctx, contents)?,
            &Underline { ref words } => {
                ctx.push_str("<u>");
                render_words(ctx, words)?;
                ctx.push_str("</u>");
            }
            &User {
                username,
                show_picture,
            } => {
                let handle = ctx.handle();
                let user = handle.get_user_by_name(username)?;

                match user {
                    Some(user) => {
                        write!(
                            ctx,
                            "<a href=\"http://www.wikidot.com/user:info/{}\">",
                            &user.name
                        )?;

                        if show_picture {
                            write!(
                                ctx,
                                "<img class=\"small\" src=\"https://example.com/avatars/{}\" alt=\"{}\">",
                                &user.id, &user.name,
                            )?;
                        }

                        write!(ctx, "{}</a>", &user.name)?;
                    }
                    None => write!(ctx, "invalid username: {}", username)?,
                }
            }
        }

        Ok(())
    }
}
