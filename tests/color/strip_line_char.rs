use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"                                                                                                                                                                                    let _: () = 42ñ"#;

    let input = &[
        Group::with_title(Level::ERROR.title("mismatched types").id("E0308")).element(
            Snippet::source(source)
                .path("$DIR/whitespace-trimming.rs")
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(192..194)
                        .label("expected (), found integer"),
                ),
        ),
    ];
    let expected = file!["strip_line_char.term.svg"];
    let renderer = Renderer::styled().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}
