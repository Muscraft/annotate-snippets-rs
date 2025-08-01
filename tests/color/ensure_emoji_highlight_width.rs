use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#""haha this isn't a valid name 🐛" = { package = "libc", version = "0.1" }
"#;

    let input = &[Group::with_title(Level::ERROR.title("invalid character ` ` in package name: `haha this isn't a valid name 🐛`, characters must be Unicode XID characters (numbers, `-`, `_`, or most letters)"))
        .element(
            Snippet::source(source)
                .path("<file>")
                .line_start(7)
                .annotation(AnnotationKind::Primary.span(0..35).label(""))
        )];
    let expected = file!["ensure_emoji_highlight_width.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
