use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"fn add_title_line(result: &mut Vec<String>, main_annotation: Option<&Annotation>) {
    if let Some(annotation) = main_annotation {
        result.push(format_title_line(
            &annotation.annotation_type,
            None,
            &annotation.label,
        ));
    }
}
"#;

    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .line_start(96)
            .fold(false)
            .annotation(
                AnnotationKind::Primary
                    .span(100..110)
                    .label("Variable defined here"),
            )
            .annotation(
                AnnotationKind::Primary
                    .span(184..194)
                    .label("Referenced here"),
            )
            .annotation(
                AnnotationKind::Primary
                    .span(243..253)
                    .label("Referenced again here"),
            ),
    )];
    let expected = file!["multiple_annotations.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
