use annotate_snippets::{Level, Renderer, Snippet};

fn main() {
    let message = Level::Error
        .message("mismatched types")
        .section(
            Snippet::source("Foo")
                .line_start(51)
                .origin("src/format.rs"),
        )
        .section(
            Snippet::source("Faa")
                .line_start(129)
                .origin("src/display.rs"),
        );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
