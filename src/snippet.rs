//! Structures used as an input for the library.

use crate::renderer::stylesheet::Stylesheet;
use anstyle::Style;
use std::ops::Range;

pub(crate) const ERROR_TXT: &str = "error";
pub(crate) const HELP_TXT: &str = "help";
pub(crate) const INFO_TXT: &str = "info";
pub(crate) const NOTE_TXT: &str = "note";
pub(crate) const WARNING_TXT: &str = "warning";

fn newline_count(body: &str) -> usize {
    #[cfg(feature = "simd")]
    {
        memchr::memchr_iter(b'\n', body.as_bytes())
            .count()
            .saturating_sub(1)
    }
    #[cfg(not(feature = "simd"))]
    {
        body.lines().count().saturating_sub(1)
    }
}

#[derive(Debug)]
pub struct Message<'a> {
    pub(crate) id: Option<&'a str>, // for "correctness", could be sloppy and be on Title
    pub(crate) sections: Vec<Section<'a>>,
}

impl<'a> Message<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn section(mut self, section: impl Into<Section<'a>>) -> Self {
        self.sections.push(section.into());
        self
    }

    pub fn sections(mut self, sections: impl IntoIterator<Item = impl Into<Section<'a>>>) -> Self {
        self.sections.extend(sections.into_iter().map(Into::into));
        self
    }

    pub(crate) fn max_line_number(&self) -> usize {
        self.sections
            .iter()
            .map(|s| {
                if let Section::Cause(cause) = s {
                    let start = cause
                        .markers
                        .iter()
                        .map(|a| a.range.start)
                        .min()
                        .unwrap_or(0);

                    let end = cause
                        .markers
                        .iter()
                        .map(|a| a.range.end)
                        .max()
                        .unwrap_or(cause.source.len())
                        .min(cause.source.len());

                    cause.line_start + newline_count(&cause.source[start..end])
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(1)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Section<'a> {
    Title(Title<'a>),
    Cause(Snippet<'a, Annotation<'a>>), // or Context?
                                        // Empty,  // for forcing blank lines
}

#[derive(Debug)]
pub struct Title<'a> {
    pub(crate) level: Level,
    pub(crate) title: &'a str,
}

impl<'a> From<Title<'a>> for Section<'a> {
    fn from(value: Title<'a>) -> Self {
        Section::Title(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

impl Level {
    pub fn message(self, title: &str) -> Message<'_> {
        Message {
            id: None,
            sections: vec![Section::Title(Title { level: self, title })],
        }
    }

    pub fn title(self, title: &str) -> Title<'_> {
        Title { level: self, title }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Level::Error => ERROR_TXT,
            Level::Warning => WARNING_TXT,
            Level::Info => INFO_TXT,
            Level::Note => NOTE_TXT,
            Level::Help => HELP_TXT,
        }
    }

    pub(crate) fn style(&self, stylesheet: &Stylesheet) -> Style {
        match self {
            Level::Error => stylesheet.error,
            Level::Warning => stylesheet.warning,
            Level::Info => stylesheet.info,
            Level::Note => stylesheet.note,
            Level::Help => stylesheet.help,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Snippet<'a, T> {
    pub(crate) origin: Option<&'a str>,
    pub(crate) line_start: usize,
    pub(crate) source: &'a str,
    pub(crate) markers: Vec<T>,
    pub(crate) fold: bool,
}

impl<'a, T: Clone> Snippet<'a, T> {
    pub fn source(source: &'a str) -> Self {
        Self {
            origin: None,
            line_start: 1,
            source,
            markers: vec![],
            fold: false,
        }
    }

    pub fn line_start(mut self, line_start: usize) -> Self {
        self.line_start = line_start;
        self
    }

    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }
}

impl<'a> Snippet<'a, Annotation<'a>> {
    pub fn annotation(mut self, annotation: Annotation<'a>) -> Snippet<'a, Annotation<'a>> {
        self.markers.push(annotation);
        self
    }

    pub fn annotations(mut self, annotation: impl IntoIterator<Item = Annotation<'a>>) -> Self {
        self.markers.extend(annotation);
        self
    }
}

impl<'a> From<Snippet<'a, Annotation<'a>>> for Section<'a> {
    fn from(value: Snippet<'a, Annotation<'a>>) -> Self {
        Section::Cause(value)
    }
}

#[derive(Clone, Debug)]
pub struct Annotation<'a> {
    pub(crate) range: Range<usize>,
    pub(crate) label: Option<&'a str>,
    pub(crate) kind: AnnotationKind,
}

impl<'a> Annotation<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationKind {
    Primary, // in rustc terms, "primary"; color to message's level
    Context, // in rustc terms, "secondary"; fixed color
             // Level(Level),  // from old API, dropped
}

impl AnnotationKind {
    pub fn span<'a>(self, span: Range<usize>) -> Annotation<'a> {
        Annotation {
            range: span,
            label: None,
            kind: self,
        }
    }

    pub(crate) fn is_primary(&self) -> bool {
        matches!(self, AnnotationKind::Primary)
    }
}
