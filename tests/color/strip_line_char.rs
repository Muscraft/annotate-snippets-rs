use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"                                                                                                                                                                                    let _: () = 42ñ"#;

    let input = &[
        Group::with_title(Level::ERROR.primary_title("mismatched types").id("E0308")).element(
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

    let expected_ascii = file!["strip_line_char.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["strip_line_char.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
