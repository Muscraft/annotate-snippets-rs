use serde::Deserialize;
use std::ops::Range;

use annotate_snippets::renderer::DEFAULT_TERM_WIDTH;
use annotate_snippets::{Annotation, AnnotationKind, Level, Message, Renderer, Section, Snippet};

#[derive(Deserialize)]
pub(crate) struct Fixture {
    #[serde(default)]
    pub(crate) renderer: RendererDef,
    pub(crate) message: MessageDef,
}

#[derive(Deserialize)]
pub struct MessageDef {
    #[serde(with = "LevelDef")]
    pub level: Level,
    pub title: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub sections: Vec<SectionDef>,
}

impl<'a> From<&'a MessageDef> for Message<'a> {
    fn from(val: &'a MessageDef) -> Self {
        let MessageDef {
            level,
            title,
            id,
            sections,
        } = val;
        let mut message = level.message(title);
        if let Some(id) = id {
            message = message.id(id);
        }

        message = message.sections(sections.iter().map(|s| match s {
            SectionDef::Title(title) => Section::Title(title.level.title(&title.title)),
            SectionDef::Cause(cause) => Section::Cause(Snippet::from(cause)),
        }));
        message
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SectionDef {
    Title(TitleDef),
    Cause(SnippetAnnotationDef),
}

impl<'a> From<&'a SectionDef> for Section<'a> {
    fn from(val: &'a SectionDef) -> Self {
        match val {
            SectionDef::Title(title) => Section::Title(title.level.title(&title.title)),
            SectionDef::Cause(cause) => Section::Cause(Snippet::from(cause)),
        }
    }
}

#[derive(Deserialize)]
pub struct TitleDef {
    pub title: String,
    #[serde(with = "LevelDef")]
    pub level: Level,
}

#[derive(Deserialize)]
pub struct SnippetAnnotationDef {
    pub(crate) origin: Option<String>,
    pub(crate) line_start: usize,
    pub(crate) source: String,
    pub(crate) annotations: Vec<AnnotationDef>,
    #[serde(default)]
    pub(crate) fold: bool,
}

impl<'a> From<&'a SnippetAnnotationDef> for Snippet<'a, Annotation<'a>> {
    fn from(val: &'a SnippetAnnotationDef) -> Self {
        let SnippetAnnotationDef {
            origin,
            line_start,
            source,
            annotations,
            fold,
        } = val;
        let mut snippet = Snippet::source(source).line_start(*line_start).fold(*fold);
        if let Some(origin) = origin {
            snippet = snippet.origin(origin);
        }
        snippet = snippet.annotations(annotations.iter().map(Into::into));
        snippet
    }
}

#[derive(Deserialize)]
pub struct AnnotationDef {
    pub range: Range<usize>,
    pub label: String,
    #[serde(with = "AnnotationKindDef")]
    pub kind: AnnotationKind,
}

impl<'a> From<&'a AnnotationDef> for Annotation<'a> {
    fn from(val: &'a AnnotationDef) -> Self {
        let AnnotationDef { range, label, kind } = val;
        kind.span(range.start..range.end).label(label)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(remote = "AnnotationKind")]
enum AnnotationKindDef {
    Primary,
    Context,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(remote = "Level")]
enum LevelDef {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

#[derive(Default, Deserialize)]
pub struct RendererDef {
    #[serde(default)]
    anonymized_line_numbers: bool,
    #[serde(default)]
    term_width: Option<usize>,
    #[serde(default)]
    color: bool,
}

impl From<RendererDef> for Renderer {
    fn from(val: RendererDef) -> Self {
        let RendererDef {
            anonymized_line_numbers,
            term_width,
            color,
        } = val;

        let renderer = if color {
            Renderer::styled()
        } else {
            Renderer::plain()
        };
        renderer
            .anonymized_line_numbers(anonymized_line_numbers)
            .term_width(term_width.unwrap_or(DEFAULT_TERM_WIDTH))
    }
}
