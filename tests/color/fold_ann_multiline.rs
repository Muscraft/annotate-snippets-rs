use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#") -> Option<String> {
    for ann in annotations {
        match (ann.range.0, ann.range.1) {
            (None, None) => continue,
            (Some(start), Some(end)) if start > end_index || end < start_index => continue,
            (Some(start), Some(end)) if start >= start_index && end <= end_index => {
                let label = if let Some(ref label) = ann.label {
                    format!(" {}", label)
                } else {
                    String::from("")
                };

                return Some(format!(
                    "{}{}{}",
                    " ".repeat(start - start_index),
                    "^".repeat(end - start),
                    label
                ));
            }
            _ => continue,
        }
    }
"#;

    let input = Level::ERROR.header("mismatched types").id("E0308").group(
        Group::new().element(
            Snippet::source(source)
                .path("src/format.rs")
                .line_start(51)
                .fold(true)
                .annotation(AnnotationKind::Context.span(5..19).label(
                    "expected `std::option::Option<std::string::String>` because of return type",
                ))
                .annotation(
                    AnnotationKind::Primary
                        .span(22..766)
                        .label("expected enum `std::option::Option`, found ()"),
                ),
        ),
    );
    let expected = file!["fold_ann_multiline.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
