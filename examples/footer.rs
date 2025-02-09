use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};

fn main() {
    let message =
        Level::Error
            .message("mismatched types")
            .id("E0308")
            .section(
                Snippet::source("        slices: vec![\"A\",")
                    .line_start(13)
                    .origin("src/multislice.rs")
                    .annotation(AnnotationKind::Primary.span(21..24).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    )),
            )
            .section(Level::Note.title(
                "expected type: `snippet::Annotation`\n   found type: `__&__snippet::Annotation`",
            ));

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
