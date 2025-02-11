use annotate_snippets::{Annotation, Level, Renderer, Snippet};

fn main() {
    let message = Level::Error
        .message("mismatched types")
        .section(
            Snippet::<Annotation<'_>>::source("Foo")
                .line_start(51)
                .origin("src/format.rs"),
        )
        .section(
            Snippet::<Annotation<'_>>::source("Faa")
                .line_start(129)
                .origin("src/display.rs"),
        );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
