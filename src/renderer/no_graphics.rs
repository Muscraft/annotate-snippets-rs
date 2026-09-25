use alloc::borrow::{Cow, ToOwned};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Reverse;
use core::fmt::{self, Write};

use super::graphics::{Hyperlink, MessageOrTitle, TitleStyle, str_width};
use super::preprocess::{Preprocessed, PreprocessedElement, PreprocessedGroup};
use super::{ElementStyle, Stylesheet, normalize_whitespace};
use crate::renderer::graphics::ANONYMIZED_LINE_NUM;
use crate::{AnnotationKind, Id, Renderer, Report};

/// Print out a file position optimized for the data available.
///
/// When the `path` is available, we print `at PATH:LL:CC`, which is detected by terminals and
/// editors as a path to a specific place in a file. Otherwise, we print`on line LL, column CC`,
/// for the elements that are available.
fn render_path(
    output: &mut String,
    path: Option<&str>,
    line: Option<usize>,
    col: Option<usize>,
    anonymized_origin_line_numbers: bool,
) -> Result<(), fmt::Error> {
    let line = line.map(|l| {
        if anonymized_origin_line_numbers {
            ANONYMIZED_LINE_NUM.to_owned()
        } else {
            l.to_string()
        }
    });

    match (path, line, col) {
        (Some(path), Some(line), Some(col)) => {
            // `at $DIR/file.txt:LL:CC`
            write!(output, "at {path}:{line}:{col}")?;
        }
        (Some(path), Some(line), None) => {
            // `at $DIR/file.txt:LL`
            write!(output, "at {path}:{line}")?;
        }
        (Some(path), None, None) => {
            // `at $DIR/file.txt`
            write!(output, "at {path}")?;
        }
        (None, Some(line), Some(col)) => {
            // We are not printing the path, so instead we show
            // ` on line LL, column CC`.
            write!(output, "on line {line}, column {col}")?;
        }
        (None, Some(line), None) => {
            // ` on line LL`
            write!(output, "on line {line}")?;
        }
        (None, None, Some(col)) => {
            // ` on column CC`
            write!(output, "on column {col}")?;
        }
        (Some(path), None, Some(col)) => {
            // This condition should never have been called.
            write!(output, "at {path}, column {col}")?;
        }
        (None, None, None) => {}
    }
    Ok(())
}

/// Render all errors from a `Report` as text only.
///
/// We will render output as follows:
///
/// ```text
/// error EXXXX: main message
///  at $DIR/file.ext:LL:CC: span label
///   on line LL, column KK: span label
/// note: note message
///  at $DIR/file.ext:LL:CC
/// help: suggestion message
///   on line LL, add `suggestion`
/// ```
pub(crate) fn render_no_graphics(
    renderer: &Renderer,
    groups: Report<'_>,
) -> Result<String, fmt::Error> {
    let mut output = String::new();
    let Preprocessed {
        max_line_num: _,
        report_primary_path,
        groups,
    } = Preprocessed::preprocess(groups);

    let mut iter = groups.into_iter().peekable();
    while let Some(PreprocessedGroup {
        group,
        elements,
        primary_path,
        max_depth: _,
    }) = iter.next()
    {
        if let Some(title) = &group.title {
            let title_style = if title.allows_styling {
                TitleStyle::Secondary
            } else {
                TitleStyle::Primary
            };
            render_title(
                title,
                &mut output,
                title_style,
                &renderer.stylesheet,
                renderer.hyperlink,
            )?;
            if iter.peek().is_some()
                || elements
                    .iter()
                    .find(|section| !matches!(section, PreprocessedElement::Padding(_)))
                    .is_some()
            {
                // This diagnostic has content, add a newline after the title.
                writeln!(output)?;
            }
        }

        let mut message_iter = elements.into_iter().enumerate().peekable();
        let mut last_suggestion_path = None;
        while let Some((_i, section)) = message_iter.next() {
            let peek = message_iter.peek().map(|(_, s)| s);
            match section {
                PreprocessedElement::Message(message) => {
                    last_suggestion_path = None;
                    render_title(
                        message,
                        &mut output,
                        TitleStyle::Message,
                        &renderer.stylesheet,
                        renderer.hyperlink,
                    )?;
                    if peek.is_some() {
                        writeln!(output)?;
                    }
                }
                PreprocessedElement::Cause((snippet, sm, _)) => {
                    last_suggestion_path = None;

                    let mut annotations = snippet
                        .markers
                        .iter()
                        .filter(|ann| !matches!(ann.kind, AnnotationKind::Visible))
                        .collect::<Vec<_>>();
                    annotations.sort_by_key(|a| (Reverse(a.kind.is_primary()), a.span.start));
                    if annotations.is_empty() {
                        // We have a diagnostic with no span labels, but we should show *some*
                        // position. We use the snippet start to provide that minimal context.
                        write!(output, " ")?;
                        render_path(
                            &mut output,
                            snippet.path.as_deref(),
                            Some(snippet.line_start),
                            None,
                            renderer.anonymized_origin_line_numbers,
                        )?;
                        if peek.is_some() {
                            writeln!(output)?;
                        }
                    }

                    for (i, annotation) in annotations.iter().enumerate() {
                        let label = annotation.label.as_ref().filter(|s| !s.is_empty());
                        if i > 0 {
                            if label.is_none() {
                                continue;
                            } else if peek.is_none() {
                                writeln!(output)?;
                            }
                        }
                        let (lo, hi) =
                            sm.span_to_locations(annotation.span.start..annotation.span.end);

                        // Indent subsequent labels under the first label's file path.
                        // Without a path, keep all labels at the same indentation.
                        let padding = if i > 0 && snippet.path.is_some() {
                            "  "
                        } else {
                            " "
                        };
                        write!(output, "{padding}")?;

                        if i == 0
                            && let Some(path) = &snippet.path
                        {
                            // `at $DIR/file.txt:LL:CC: label`
                            //  ^^^^^^^^^^^^^^^^^^^^^^
                            render_path(
                                &mut output,
                                Some(path),
                                Some(lo.line),
                                Some(lo.char + 1),
                                renderer.anonymized_origin_line_numbers,
                            )?;
                            if lo.line != hi.line {
                                // This is a multiline highlight, so we mention both the start and the
                                // end. `LL:CC to MM:DD`
                                //            ^^^^^^^^^
                                write!(output, " to {}:{}", hi.line, hi.char + 1)?;
                            }
                        } else {
                            let col = if let Some(line) = sm.get_line(lo.line)
                                && line.chars().take(lo.char).all(|c| c.is_whitespace())
                            {
                                // Everything before the span is whitespace, mentioning the column
                                // doesn't add information.
                                None
                            } else {
                                Some(lo.char + 1)
                            };
                            render_path(
                                &mut output,
                                None,
                                Some(lo.line),
                                col,
                                renderer.anonymized_origin_line_numbers,
                            )?;
                            if lo.line != hi.line {
                                // This is a multiline highlight, so we mention both the start and the
                                // end. `line LL, column CC to line MM, column DD`
                                //                         ^^^^^^^^^^^^^^^^^^^^^^
                                write!(output, " to line {}", hi.line)?;
                                if let Some(line) = sm.get_line(hi.line)
                                    && let Some(post) = line.get(hi.char..)
                                    && post.chars().all(|c| c.is_whitespace())
                                {
                                    // Everything after the span is whitespace, mentioning the
                                    // column doesn't add information.
                                } else {
                                    write!(output, ", column {}", hi.char + 1)?;
                                }
                            }
                        }

                        if let Some(label) = label {
                            // If the span has a label, we render it to the right of the position
                            // information.
                            // `at $DIR/file.txt:LL:CC: this is the label`
                            //                        ^^^^^^^^^^^^^^^^^^^
                            write!(output, ": {label}")?;
                        }
                        if peek.is_some() {
                            writeln!(output)?;
                        }
                    }
                }
                PreprocessedElement::Suggestion((
                    suggestion,
                    sm,
                    spliced_lines,
                    _display_suggestion,
                )) => {
                    if spliced_lines.patches.is_empty() {
                        // We have a suggestion with no patches, we don't render anything.
                        continue;
                    }

                    let next_is_suggestion =
                        matches!(peek, Some(PreprocessedElement::Suggestion(_)));

                    // We only include the file path when it is different to the
                    // primary file.
                    //
                    // `at $DIR/file.txt:LL:CC: label`
                    //  ^^^^^^^^^^^^^^^^^
                    let path = if suggestion.path.as_ref() != primary_path.or(report_primary_path)
                        && let Some(path) = suggestion.path.as_ref()
                        && last_suggestion_path.map(|(p, _)| p) != Some(suggestion.path.as_ref())
                    {
                        Some(path.as_ref())
                    } else {
                        None
                    };

                    let padding = if let Some(count) = last_suggestion_path.map(|(_, c)| c) {
                        writeln!(output, " option {}", count + 1)?;
                        "  "
                    } else if next_is_suggestion {
                        writeln!(output, " option 1")?;
                        "  "
                    } else {
                        " "
                    };

                    for (i, patch) in spliced_lines.patches.iter().enumerate() {
                        let (lo, hi) = sm.span_to_locations(patch.span.start..patch.span.end);

                        let col = if lo.line == hi.line
                            && let Some(line) = sm.get_line(lo.line)
                            && let Some(pre) = line.get(..lo.byte)
                            && pre.chars().all(|c| c.is_whitespace())
                            && let Some(post) = line.get(hi.byte..)
                            && (patch.replacement.lines().count() > 1
                                || post.chars().all(|c| c.is_whitespace()))
                        {
                            // We are changing the whole text in the line, no need to mention the
                            // column.
                            None
                        } else {
                            Some(lo.char + 1)
                        };

                        write!(output, "{padding}")?;

                        render_path(
                            &mut output,
                            path,
                            Some(lo.line),
                            col,
                            renderer.anonymized_origin_line_numbers,
                        )?;

                        let add = if let Some(snippet) =
                            sm.span_to_snippet(patch.span.start..patch.span.end)
                            && snippet.chars().all(|c| c.is_whitespace())
                        {
                            "add"
                        } else {
                            "replace with"
                        };

                        if !patch.replacement.trim().is_empty() {
                            write!(output, " {add}: ")?;
                            let st = ElementStyle::Addition
                                .color_spec(&crate::Level::NOTE, &renderer.stylesheet);

                            write!(
                                output,
                                "{st}{}{st:#}",
                                normalize_whitespace(patch.replacement.trim_end_matches('\n'))
                            )?;
                        }
                        if i + 1 != spliced_lines.patches.len() {
                            writeln!(output)?;
                        }
                    }
                    if peek.is_some() {
                        writeln!(output)?;
                    }
                    last_suggestion_path = Some((
                        suggestion.path.as_ref(),
                        last_suggestion_path.map_or(1, |(_, count)| count + 1),
                    ));
                }
                PreprocessedElement::Origin(origin) => {
                    last_suggestion_path = None;
                    write!(output, " ")?;
                    render_path(
                        &mut output,
                        Some(&origin.path),
                        origin.line,
                        origin.char_column,
                        renderer.anonymized_origin_line_numbers,
                    )?;
                    if peek.is_some() {
                        writeln!(output)?;
                    }
                }
                PreprocessedElement::Padding(_) => {
                    last_suggestion_path = None;
                }
            }
        }

        if iter.peek().is_some()
            && let Some(c) = output.chars().last()
            && c != '\n'
        {
            writeln!(output)?;
        }
    }

    Ok(output)
}

/// Render the main message line for a diagnostic
///
/// This will print out:
///
/// ```text
/// LEVEL CODE: message
/// ```
/// like the following:
/// ```text
/// error E0308: mismatched types
/// ```
fn render_title(
    title: &dyn MessageOrTitle,
    buffer: &mut String,
    title_style: TitleStyle,
    stylesheet: &Stylesheet,
    hyperlink: bool,
) -> Result<(), fmt::Error> {
    let (label_style, title_element_style) = match title_style {
        TitleStyle::Primary => (
            ElementStyle::Level(title.level().level),
            ElementStyle::MainHeaderMsg,
        ),
        TitleStyle::Secondary => (
            ElementStyle::Level(title.level().level),
            ElementStyle::HeaderMsg,
        ),
        TitleStyle::Message => (ElementStyle::MainHeaderMsg, ElementStyle::NoStyle),
    };
    let label_style = label_style.color_spec(title.level(), stylesheet);
    let title_element_style = title_element_style.color_spec(title.level(), stylesheet);

    let mut label_width = 0;
    let level_is_visible = title.level().name != Some(None);
    if level_is_visible || title.id().is_some() {
        if title_style == TitleStyle::Message {
            write!(buffer, " ")?;
            label_width += 1;
        }

        if level_is_visible {
            // error EXXXX: message
            // ^^^^^
            write!(buffer, "{}{}{0:#}", label_style, title.level().as_str(),)?;
            label_width += str_width(title.level().as_str());
        }

        if let Some(Id { id: Some(id), url }) = &title.id() {
            let url = url
                .as_deref()
                .filter(|_| hyperlink)
                .map(Hyperlink::with_url)
                .unwrap_or_default();

            if level_is_visible {
                // error EXXXX: message
                //      ^
                write!(buffer, " ")?;
                label_width += 1;
            }
            // error EXXXX: message
            //       ^^^^^
            write!(buffer, "{label_style}{url}{id}{url:#}{label_style:#}")?;
            label_width += str_width(id);
        }
        // error EXXXX: message
        //            ^
        write!(buffer, "{title_element_style}: {title_element_style:#}")?;
        label_width += 2;
    }
    let padding = " ".repeat(label_width);

    // error EXXXX: message
    //              ^^^^^^^
    let (title_str, st) = if title.allows_styling() {
        (
            Cow::Borrowed(title.text()),
            ElementStyle::NoStyle.color_spec(title.level(), stylesheet),
        )
    } else {
        (normalize_whitespace(title.text()), title_element_style)
    };
    for (i, text) in title_str.split('\n').enumerate() {
        if i != 0 {
            write!(buffer, "\n{padding}")?;
        }
        write!(buffer, "{st}{text}{st:#}")?;
    }
    Ok(())
}
