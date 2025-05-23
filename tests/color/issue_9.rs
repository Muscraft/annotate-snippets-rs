use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let input = Level::ERROR.header("expected one of `.`, `;`, `?`, or an operator, found `for`")
        .group(
            Group::new()
                .element(
                    Snippet::source("let x = vec![1];")
                        .origin("/code/rust/src/test/ui/annotate-snippet/suggestion.rs")
                        .line_start(4)
                        .annotation(AnnotationKind::Context.span(4..5).label("move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait"))
                )
                .element(
                    Snippet::source("let y = x;")
                        .line_start(7)
                        .annotation(AnnotationKind::Context.span(8..9).label("value moved here"))
                )
                .element(
                    Snippet::source("x;")
                        .line_start(9)
                        .annotation(AnnotationKind::Primary.span(0..1).label("value used here after move"))
                )
            )
;
    let expected = file!["issue_9.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
