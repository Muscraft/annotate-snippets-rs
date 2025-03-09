use annotate_snippets::{
    Annotation, AnnotationKind, Group, Level, Origin, Patch, Renderer, Snippet,
};

use annotate_snippets::renderer::OutputTheme;
use snapbox::{assert_data_eq, str};

#[test]
fn test_i_29() {
    let snippets = Level::Error.message("oops").group(
        Group::new().element(
            Snippet::source("First line\r\nSecond oops line")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(19..23).label("oops"))
                .fold(true),
        ),
    );
    let expected = str![[r#"
error: oops
 --> <current file>:2:8
  |
2 | Second oops line
  |        ^^^^ oops
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こんにちは、世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(18..24).label("world")),
        ),
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:7
  |
1 | こんにちは、世界
  |             ^^^^ world
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters_across_lines() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("おはよう\nございます")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(6..22).label("Good morning")),
        ),
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:3
  |
1 |   おはよう
  |  _____^
2 | | ございます
  | |______^ Good morning
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters_multiple() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("お寿司\n食べたい🍣")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(0..9).label("Sushi1"))
                .annotation(AnnotationKind::Context.span(16..22).label("Sushi2")),
        ),
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:1
  |
1 | お寿司
  | ^^^^^^ Sushi1
2 | 食べたい🍣
  |     ---- Sushi2
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters_mixed() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こんにちは、新しいWorld！")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(18..32).label("New world")),
        ),
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:7
  |
1 | こんにちは、新しいWorld！
  |             ^^^^^^^^^^^ New world
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_format_title() {
    let input = Level::Error.message("This is a title").id("E0001");

    let expected = str![r#"error[E0001]: This is a title"#];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippet_only() {
    let source = "This is line 1\nThis is line 2";
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source(source).line_start(5402)));

    let expected = str![[r#"
error: 
     |
5402 | This is line 1
5403 | This is line 2
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippets_continuation() {
    let src_0 = "This is slice 1";
    let src_1 = "This is slice 2";
    let input = Level::Error.message("").group(
        Group::new()
            .element(
                Snippet::<Annotation<'_>>::source(src_0)
                    .line_start(5402)
                    .origin("file1.rs"),
            )
            .element(
                Snippet::<Annotation<'_>>::source(src_1)
                    .line_start(2)
                    .origin("file2.rs"),
            ),
    );
    let expected = str![[r#"
error: 
    --> file1.rs
     |
5402 | This is slice 1
     |
    ::: file2.rs:2
     |
   2 | This is slice 2
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippet_annotation_standalone() {
    let line_1 = "This is line 1";
    let line_2 = "This is line 2";
    let source = [line_1, line_2].join("\n");
    // In line 2
    let range = 22..24;
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(&source).line_start(5402).annotation(
                AnnotationKind::Context
                    .span(range.clone())
                    .label("Test annotation"),
            ),
        ),
    );
    let expected = str![[r#"
error: 
     |
5402 | This is line 1
5403 | This is line 2
     |        -- Test annotation
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_footer_title() {
    let input = Level::Error
        .message("")
        .group(Group::new().element(Level::Error.title("This __is__ a title")));
    let expected = str![[r#"
error: 
  |
  = error: This __is__ a title
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
#[should_panic]
fn test_i26() {
    let source = "short";
    let label = "label";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source).line_start(0).annotation(
                AnnotationKind::Primary
                    .span(0..source.len() + 2)
                    .label(label),
            ),
        ),
    );
    let renderer = Renderer::plain();
    let _ = renderer.render(input);
}

#[test]
fn test_source_content() {
    let source = "This is an example\nof content lines";
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source(source).line_start(56)));
    let expected = str![[r#"
error: 
   |
56 | This is an example
57 | of content lines
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_source_annotation_standalone_singleline() {
    let source = "tests";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .annotation(AnnotationKind::Context.span(0..5).label("Example string")),
        ),
    );
    let expected = str![[r#"
error: 
  |
1 | tests
  | ----- Example string
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_source_annotation_standalone_multiline() {
    let source = "tests";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .annotation(AnnotationKind::Context.span(0..5).label("Example string"))
                .annotation(AnnotationKind::Context.span(0..5).label("Second line")),
        ),
    );
    let expected = str![[r#"
error: 
  |
1 | tests
  | -----
  | |
  | Example string
  | Second line
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_only_source() {
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source("").origin("file.rs")));
    let expected = str![[r#"
error: 
 --> file.rs
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_anon_lines() {
    let source = "This is an example\nof content lines\n\nabc";
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source(source).line_start(56)));
    let expected = str![[r#"
error: 
   |
LL | This is an example
LL | of content lines
LL |
LL | abc
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn issue_130() {
    let input = Level::Error.message("dummy").group(
        Group::new().element(
            Snippet::source("foo\nbar\nbaz")
                .origin("file/path")
                .line_start(3)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(4..11)),
        ), // bar\nbaz
    );

    let expected = str![[r#"
error: dummy
 --> file/path:4:1
  |
4 | / bar
5 | | baz
  | |___^
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unterminated_string_multiline() {
    let source = "\
a\"
// ...
";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(0..10)),
        ), // 1..10 works
    );
    let expected = str![[r#"
error: 
 --> file/path:3:1
  |
3 | / a"
4 | | // ...
  | |_______^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn char_and_nl_annotate_char() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(0..2)),
        ), // a\r
    );
    let expected = str![[r#"
error: 
 --> file/path:3:1
  |
3 | a
  | ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn char_eol_annotate_char() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(0..3)),
        ), // a\r\n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:1
  |
3 | a
  | ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn char_eol_annotate_char_double_width() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こん\r\nにちは\r\n世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(3..8)),
        ), // ん\r\n
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:2
  |
1 | こん
  |   ^^
2 | にちは
3 | 世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn annotate_eol() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..2)),
        ), // \r
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 | a
  |  ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol2() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..3)),
        ), // \r\n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 | a
  |  ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol3() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..3)),
        ), // \n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 | a
  |  ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol4() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..2)),
        ), // \n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 | a
  |  ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol_double_width() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こん\r\nにちは\r\n世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(7..8)),
        ), // \n
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:4
  |
1 | こん
  |     ^
2 | にちは
3 | 世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn multiline_eol_start() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..4)),
        ), // \r\nb
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start2() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..4)),
        ), // \nb
    );
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start3() {
    let source = "a\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..3)),
        ), // \nb
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_double_width() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こん\r\nにちは\r\n世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(7..11)),
        ), // \r\nに
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:4
  |
1 |   こん
  |  _____^
2 | | にちは
  | |__^
3 |   世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn multiline_eol_start_eol_end() {
    let source = "a\nb\nc";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..4)),
        ), // \nb\n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |__^
5 |   c
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eol_end2() {
    let source = "a\r\nb\r\nc";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..5)),
        ), // \nb\r
    );
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
  | |__^
5 |   c
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eol_end3() {
    let source = "a\r\nb\r\nc";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..6)),
        ), // \nb\r\n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
  | |__^
5 |   c
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eof_end() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..5)),
        ), // \r\nb(EOF)
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |__^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eof_end_double_width() {
    let source = "ん\r\nに";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(3..9)),
        ), // \r\nに(EOF)
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   ん
  |  ___^
4 | | に
  | |___^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn two_single_line_same_line() {
    let source = r#"bar = { version = "0.1.0", optional = true }"#;
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
            Snippet::source(source)
                .origin("Cargo.toml")
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(0..3)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    );
    let expected = str![[r#"
error: unused optional dependency
 --> Cargo.toml:4:1
  |
4 | bar = { version = "0.1.0", optional = true }
  | ^^^                        --------------- This should also be long but not too long
  | |
  | I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multi_and_single() {
    let source = r#"bar = { version = "0.1.0", optional = true }
this is another line
so is this
bar = { version = "0.1.0", optional = true }
"#;
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(41..119)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    );
    let expected = str![[r#"
error: unused optional dependency
  |
4 |   bar = { version = "0.1.0", optional = true }
  |  ____________________________--------------^
  | |                            |
  | |                            This should also be long but not too long
5 | | this is another line
6 | | so is this
7 | | bar = { version = "0.1.0", optional = true }
  | |__________________________________________^ I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn two_multi_and_single() {
    let source = r#"bar = { version = "0.1.0", optional = true }
this is another line
so is this
bar = { version = "0.1.0", optional = true }
"#;
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(41..119)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(8..102)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    );
    let expected = str![[r#"
error: unused optional dependency
  |
4 |    bar = { version = "0.1.0", optional = true }
  |   _________^__________________--------------^
  |  |         |                  |
  |  |_________|                  This should also be long but not too long
  | ||
5 | || this is another line
6 | || so is this
7 | || bar = { version = "0.1.0", optional = true }
  | ||_________________________^________________^ I need this to be really long so I can test overlaps
  | |__________________________|
  |                            I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn three_multi_and_single() {
    let source = r#"bar = { version = "0.1.0", optional = true }
this is another line
so is this
bar = { version = "0.1.0", optional = true }
this is another line
"#;
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(41..119)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(8..102)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(48..126)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    );
    let expected = str![[r#"
error: unused optional dependency
  |
4 |     bar = { version = "0.1.0", optional = true }
  |   __________^__________________--------------^
  |  |          |                  |
  |  |__________|                  This should also be long but not too long
  | ||
5 | ||  this is another line
  | || ____^
6 | ||| so is this
7 | ||| bar = { version = "0.1.0", optional = true }
  | |||_________________________^________________^ I need this to be really long so I can test overlaps
  | |_|_________________________|
  |   |                         I need this to be really long so I can test overlaps
8 |   | this is another line
  |   |____^ I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn origin_correct_start_line() {
    let source = "aaa\nbbb\nccc\nddd\n";
    let input = Level::Error.message("title").group(
        Group::new().element(
            Snippet::source(source)
                .origin("origin.txt")
                .fold(false)
                .annotation(AnnotationKind::Primary.span(8..8 + 3).label("annotation")),
        ),
    );

    let expected = str![[r#"
error: title
 --> origin.txt:3:1
  |
1 | aaa
2 | bbb
3 | ccc
  | ^^^ annotation
4 | ddd
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn origin_correct_mid_line() {
    let source = "aaa\nbbb\nccc\nddd\n";
    let input = Level::Error.message("title").group(
        Group::new().element(
            Snippet::source(source)
                .origin("origin.txt")
                .fold(false)
                .annotation(
                    AnnotationKind::Primary
                        .span(8 + 1..8 + 3)
                        .label("annotation"),
                ),
        ),
    );

    let expected = str![[r#"
error: title
 --> origin.txt:3:2
  |
1 | aaa
2 | bbb
3 | ccc
  |  ^^ annotation
4 | ddd
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn two_suggestions_same_span() {
    let source = r#"    A.foo();"#;
    let input_new = Level::Error
        .message("expected value, found enum `A`")
        .id("E0423")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(4..5)),
            ),
        )
        .group(
            Group::new()
                .element(
                    Level::Help
                        .title("you might have meant to use one of the following enum variants"),
                )
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(4..5, "(A::Tuple())")),
                )
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(4..5, "A::Unit")),
                ),
        );

    let expected = str![[r#"
error[E0423]: expected value, found enum `A`
   |
LL |     A.foo();
   |     ^
   |
help: you might have meant to use one of the following enum variants
   |
LL -     A.foo();
LL +     (A::Tuple()).foo();
   |
LL |     A::Unit.foo();
   |      ++++++
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn two_suggestions_same_span2() {
    let source = r#"
mod banana {
    pub struct Chaenomeles;

    pub trait Apple {
        fn pick(&self) {}
    }
    impl Apple for Chaenomeles {}

    pub trait Peach {
        fn pick(&self, a: &mut ()) {}
    }
    impl<Mango: Peach> Peach for Box<Mango> {}
    impl Peach for Chaenomeles {}
}

fn main() {
    banana::Chaenomeles.pick()
}"#;
    let input_new =
        Level::Error
            .message("no method named `pick` found for struct `Chaenomeles` in the current scope")
            .id("E0599")
            .group(
                Group::new().element(
                    Snippet::source(source)
                        .line_start(1)
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(18..40)
                                .label("method `pick` not found for this struct"),
                        )
                        .annotation(
                            AnnotationKind::Primary
                                .span(318..322)
                                .label("method not found in `Chaenomeles`"),
                        ),
                ),
            )
            .group(
                Group::new()
                    .element(Level::Help.title(
                        "the following traits which provide `pick` are implemented but not in scope; perhaps you want to import one of them",
                    ))
                    .element(
                        Snippet::source(source)
                            .fold(true)
                            .patch(Patch::new(1..1, "use banana::Apple;\n")),
                    )
                    .element(
                        Snippet::source(source)
                            .fold(true)
                            .patch(Patch::new(1..1, "use banana::Peach;\n")),
                    ),
            );
    let expected = str![[r#"
error[E0599]: no method named `pick` found for struct `Chaenomeles` in the current scope
   |
LL |     pub struct Chaenomeles;
   |     ---------------------- method `pick` not found for this struct
...
LL |     banana::Chaenomeles.pick()
   |                         ^^^^ method not found in `Chaenomeles`
   |
help: the following traits which provide `pick` are implemented but not in scope; perhaps you want to import one of them
   |
LL + use banana::Apple;
   |
LL + use banana::Peach;
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn single_line_non_overlapping_suggestions() {
    let source = r#"    A.foo();"#;

    let input_new = Level::Error
        .message("expected value, found enum `A`")
        .id("E0423")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .fold(true)
                    .line_start(1)
                    .annotation(AnnotationKind::Primary.span(4..5)),
            ),
        )
        .group(
            Group::new()
                .element(Level::Help.title("make these changes and things will work"))
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .fold(true)
                        .patch(Patch::new(4..5, "(A::Tuple())"))
                        .patch(Patch::new(6..9, "bar")),
                ),
        );

    let expected = str![[r#"
error[E0423]: expected value, found enum `A`
   |
LL |     A.foo();
   |     ^
   |
help: make these changes and things will work
   |
LL -     A.foo();
LL +     (A::Tuple()).bar();
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn single_line_non_overlapping_suggestions2() {
    let source = r#"    ThisIsVeryLong.foo();"#;
    let input_new = Level::Error
        .message("Found `ThisIsVeryLong`")
        .id("E0423")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .fold(true)
                    .line_start(1)
                    .annotation(AnnotationKind::Primary.span(4..18)),
            ),
        )
        .group(
            Group::new()
                .element(Level::Help.title("make these changes and things will work"))
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .fold(true)
                        .patch(Patch::new(4..18, "(A::Tuple())"))
                        .patch(Patch::new(19..22, "bar")),
                ),
        );

    let expected = str![[r#"
error[E0423]: Found `ThisIsVeryLong`
   |
LL |     ThisIsVeryLong.foo();
   |     ^^^^^^^^^^^^^^
   |
help: make these changes and things will work
   |
LL -     ThisIsVeryLong.foo();
LL +     (A::Tuple()).bar();
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiple_replacements() {
    let source = r#"
    let y = || {
        self.bar();
    };
    self.qux();
    y();
"#;

    let input_new = Level::Error
        .message("cannot borrow `*self` as mutable because it is also borrowed as immutable")
        .id("E0502")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(49..59)
                            .label("mutable borrow occurs here"),
                    )
                    .annotation(
                        AnnotationKind::Primary
                            .span(13..15)
                            .label("immutable borrow occurs here"),
                    )
                    .annotation(
                        AnnotationKind::Primary
                            .span(26..30)
                            .label("first borrow occurs due to use of `*self` in closure"),
                    )
                    .annotation(
                        AnnotationKind::Primary
                            .span(65..66)
                            .label("immutable borrow later used here"),
                    ),
            ),
        )
        .group(
            Group::new()
                .element(
                    Level::Help
                        .title("try explicitly pass `&Self` into the Closure as an argument"),
                )
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(14..14, "this: &Self"))
                        .patch(Patch::new(26..30, "this"))
                        .patch(Patch::new(66..68, "(self)")),
                ),
        );
    let expected = str![[r#"
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   |
LL |     let y = || {
   |             ^^ immutable borrow occurs here
LL |         self.bar();
   |         ^^^^ first borrow occurs due to use of `*self` in closure
LL |     };
LL |     self.qux();
   |     ^^^^^^^^^^ mutable borrow occurs here
LL |     y();
   |     ^ immutable borrow later used here
   |
help: try explicitly pass `&Self` into the Closure as an argument
   |
LL ~     let y = |this: &Self| {
LL ~         this.bar();
LL |     };
LL |     self.qux();
LL ~     y(self);
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiple_replacements2() {
    let source = r#"
fn test1() {
    let mut chars = "Hello".chars();
    for _c in chars.by_ref() {
        chars.next();
    }
}

fn main() {
    test1();
}"#;

    let input_new = Level::Error
        .message("cannot borrow `chars` as mutable more than once at a time")
        .id("E0499")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(65..70)
                            .label("first mutable borrow occurs here"),
                    )
                    .annotation(
                        AnnotationKind::Primary
                            .span(90..95)
                            .label("second mutable borrow occurs here"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(65..79)
                            .label("first borrow later used here"),
                    ),
            ),
        )
        .group(
            Group::new()
                .element(
                    Level::Help
                        .title("if you want to call `next` on a iterator within the loop, consider using `while let`")
                )
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(
                            55..59,
                            "let iter = chars.by_ref();\n    while let Some(",
                        ))
                        .patch(Patch::new(61..79, ") = iter.next()"))
                        .patch(Patch::new(90..95, "iter")),
                ),
        );

    let expected = str![[r#"
error[E0499]: cannot borrow `chars` as mutable more than once at a time
   |
LL |     for _c in chars.by_ref() {
   |               --------------
   |               |
   |               first mutable borrow occurs here
   |               first borrow later used here
LL |         chars.next();
   |         ^^^^^ second mutable borrow occurs here
   |
help: if you want to call `next` on a iterator within the loop, consider using `while let`
   |
LL ~     let iter = chars.by_ref();
LL ~     while let Some(_c) = iter.next() {
LL ~         iter.next();
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn diff_format() {
    let source = r#"
use st::cell::Cell;

mod bar {
    pub fn bar() { bar::baz(); }

    fn baz() {}
}

use bas::bar;

struct Foo {
    bar: st::cell::Cell<bool>
}

fn main() {}"#;

    let input_new = Level::Error
        .message("failed to resolve: use of undeclared crate or module `st`")
        .id("E0433")
        .group(
            Group::new().element(
                Snippet::source(source).line_start(1).fold(true).annotation(
                    AnnotationKind::Primary
                        .span(122..124)
                        .label("use of undeclared crate or module `st`"),
                ),
            ),
        )
        .group(
            Group::new()
                .element(Level::Help.title("there is a crate or module with a similar name"))
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(122..124, "std")),
                ),
        )
        .group(
            Group::new()
                .element(Level::Help.title("consider importing this module"))
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(1..1, "use std::cell;\n")),
                ),
        )
        .group(
            Group::new()
                .element(Level::Help.title("if you import `cell`, refer to it directly"))
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(122..126, "")),
                ),
        );
    let expected = str![[r#"
error[E0433]: failed to resolve: use of undeclared crate or module `st`
   |
LL |     bar: st::cell::Cell<bool>
   |          ^^ use of undeclared crate or module `st`
   |
help: there is a crate or module with a similar name
   |
LL |     bar: std::cell::Cell<bool>
   |            +
help: consider importing this module
   |
LL + use std::cell;
   |
help: if you import `cell`, refer to it directly
   |
LL -     bar: st::cell::Cell<bool>
LL +     bar: cell::Cell<bool>
   |
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiline_removal() {
    let source = r#"
struct Wrapper<T>(T);

fn foo<T>(foo: Wrapper<T>)

where
    T
    :
    ?
    Sized
{
    //
}

fn main() {}"#;

    let input_new = Level::Error
        .message("the size for values of type `T` cannot be known at compilation time")
        .id("E0277")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(39..49)
                            .label("doesn't have a size known at compile-time"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(31..32)
                            .label("this type parameter needs to be `Sized`"),
                    ),
            ),
        )
        .group(
            Group::new()
                .element(Level::Help.title(
                    "consider removing the `?Sized` bound to make the type parameter `Sized`",
                ))
                .element(
                    Snippet::source(source)
                        .fold(true)
                        .patch(Patch::new(52..86, "")),
                ),
        );
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
LL | fn foo<T>(foo: Wrapper<T>)
   |        -       ^^^^^^^^^^ doesn't have a size known at compile-time
   |        |
   |        this type parameter needs to be `Sized`
   |
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - where
LL -     T
LL -     :
LL -     ?
LL -     Sized
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiline_replacement() {
    let source = r#"
struct Wrapper<T>(T);

fn foo<T>(foo: Wrapper<T>)

and where
    T
    :
    ?
    Sized
{
    //
}

fn main() {}"#;
    let input_new = Level::Error
        .message("the size for values of type `T` cannot be known at compilation time")
        .id("E0277")
        .group(Group::new().element(Snippet::source(source)
            .line_start(1)
            .origin("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(39..49)
                    .label("doesn't have a size known at compile-time"),
            )
            .annotation(
                AnnotationKind::Context
                    .span(31..32)
                    .label("this type parameter needs to be `Sized`"),
            )))
        .group(Group::new().element(
            Level::Note
                .title("required by an implicit `Sized` bound in `Wrapper`")
        ).element(
            Snippet::source(source)
                .line_start(1)
                .origin("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(16..17)
                        .label("required by the implicit `Sized` requirement on this type parameter in `Wrapper`"),
                )
        ))
        .group(Group::new().element(
            Level::Help
                .title("you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`")
            )
            .element(
            Snippet::source(source)
                .line_start(1)
                .origin("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(16..17)
                        .label("this could be changed to `T: ?Sized`..."),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(19..20)
                        .label("...if indirection were used here: `Box<T>`"),
                )

        ))
        .group(Group::new().element(
            Level::Help
                .title("consider removing the `?Sized` bound to make the type parameter `Sized`")
        ).element(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(56..90, ""))
                .patch(Patch::new(90..90, "+ Send"))
                ,
        ));
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> $DIR/removal-of-multiline-trait-bound-in-where-clause.rs:4:16
   |
LL | fn foo<T>(foo: Wrapper<T>)
   |        -       ^^^^^^^^^^ doesn't have a size known at compile-time
   |        |
   |        this type parameter needs to be `Sized`
   |
note: required by an implicit `Sized` bound in `Wrapper`
  --> $DIR/removal-of-multiline-trait-bound-in-where-clause.rs:2:16
   |
LL | struct Wrapper<T>(T);
   |                ^ required by the implicit `Sized` requirement on this type parameter in `Wrapper`
help: you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`
  --> $DIR/removal-of-multiline-trait-bound-in-where-clause.rs:2:16
   |
LL | struct Wrapper<T>(T);
   |                ^  - ...if indirection were used here: `Box<T>`
   |                |
   |                this could be changed to `T: ?Sized`...
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL ~ and 
LL ~ + Send{
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiline_removal2() {
    let source = r#"
cargo
fuzzy
pizza
jumps
crazy
quack
zappy
"#;

    let input_new = Level::Error
        .message("the size for values of type `T` cannot be known at compilation time")
        .id("E0277")
        .group(
            Group::new()
                .element(Level::Help.title(
                    "consider removing the `?Sized` bound to make the type parameter `Sized`",
                ))
                .element(
                    Snippet::source(source)
                        .line_start(7)
                        .fold(true)
                        .patch(Patch::new(3..21, ""))
                        .patch(Patch::new(22..40, "")),
                ),
        );
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
8  - cargo
9  - fuzzy
10 - pizza
11 - jumps
8  + campy
   |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn e0271() {
    let source = r#"
trait Future {
    type Error;
}

impl<T, E> Future for Result<T, E> {
    type Error = E;
}

impl<T> Future for Option<T> {
    type Error = ();
}

struct Foo;

fn foo() -> Box<dyn Future<Error=Foo>> {
    Box::new(
        Ok::<_, ()>(
            Err::<(), _>(
                Ok::<_, ()>(
                    Err::<(), _>(
                        Ok::<_, ()>(
                            Err::<(), _>(Some(5))
                        )
                    )
                )
            )
        )
    )
}
fn main() {
}
"#;

    let input_new = Level::Error
        .message("type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`")
        .id("E0271")
        .group(Group::new().element(Snippet::source(source)
            .line_start(4)
            .origin("$DIR/E0271.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(208..510)
                    .label("type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`"),
            )))
        .group(Group::new().element(
            Level::Note.title("expected this to be `Foo`")
        ).element(
            Snippet::source(source)
                .line_start(4)
                .origin("$DIR/E0271.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(89..90))
        ).element(
            Level::Note
                .title("required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`")
                ,
        ));

    let expected = str![[r#"
error[E0271]: type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`
   ╭▸ $DIR/E0271.rs:20:5
   │
LL │ ┏     Box::new(
LL │ ┃         Ok::<_, ()>(
LL │ ┃             Err::<(), _>(
LL │ ┃                 Ok::<_, ()>(
   ‡ ┃
LL │ ┃     )
   │ ┗━━━━━┛ type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`
   ╰╴
note: expected this to be `Foo`
   ╭▸ $DIR/E0271.rs:10:18
   │
LL │     type Error = E;
   │                  ━
   ╰ note: required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`
"#]];
    let renderer = Renderer::plain()
        .term_width(40)
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn e0271_2() {
    let source = r#"
trait Future {
    type Error;
}

impl<T, E> Future for Result<T, E> {
    type Error = E;
}

impl<T> Future for Option<T> {
    type Error = ();
}

struct Foo;

fn foo() -> Box<dyn Future<Error=Foo>> {
    Box::new(
        Ok::<_, ()>(
            Err::<(), _>(
                Ok::<_, ()>(
                    Err::<(), _>(
                        Ok::<_, ()>(
                            Err::<(), _>(Some(5))
                        )
                    )
                )
            )
        )
    )
}
fn main() {
}
"#;

    let input_new = Level::Error
        .message("type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`")
        .id("E0271")
        .group(Group::new().element(Snippet::source(source)
            .line_start(4)
            .origin("$DIR/E0271.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(208..510)
                    .label("type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`"),
            )))
        .group(Group::new().element(
            Level::Note.title("expected this to be `Foo`")
        ).element(
            Snippet::source(source)
                .line_start(4)
                .origin("$DIR/E0271.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(89..90))
        ).element(
            Level::Note
                .title("required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`")
        ).element(
            Level::Note.title("a second note"),
        ));

    let expected = str![[r#"
error[E0271]: type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`
   ╭▸ $DIR/E0271.rs:20:5
   │
LL │ ┏     Box::new(
LL │ ┃         Ok::<_, ()>(
LL │ ┃             Err::<(), _>(
LL │ ┃                 Ok::<_, ()>(
   ‡ ┃
LL │ ┃     )
   │ ┗━━━━━┛ type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`
   ╰╴
note: expected this to be `Foo`
   ╭▸ $DIR/E0271.rs:10:18
   │
LL │     type Error = E;
   │                  ━
   ├ note: required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`
   ╰ note: a second note
"#]];
    let renderer = Renderer::plain()
        .term_width(40)
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn long_e0308() {
    let source = r#"
mod a {
    // Force the "short path for unique types" machinery to trip up
    pub struct Atype;
    pub struct Btype;
    pub struct Ctype;
}

mod b {
    pub struct Atype<T, K>(T, K);
    pub struct Btype<T, K>(T, K);
    pub struct Ctype<T, K>(T, K);
}

use b::*;

fn main() {
    let x: Atype<
      Btype<
        Ctype<
          Atype<
            Btype<
              Ctype<
                Atype<
                  Btype<
                    Ctype<i32, i32>,
                    i32
                  >,
                  i32
                >,
                i32
              >,
              i32
            >,
            i32
          >,
          i32
        >,
        i32
      >,
      i32
    > = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok("")
        ))))))))))))))))))))))))))))))
    ))))))))))))))))))))))))))))));
    //~^^^^^ ERROR E0308

    let _ = Some(Ok(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
        Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
            Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
                Some(Some(Some(Some(Some(Some(Some(Some(Some("")))))))))
            )))))))))))))))))
        ))))))))))))))))))
    ))))))))))))))))) == Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok(Ok(Ok(Ok(Ok(Ok(Ok("")))))))
        ))))))))))))))))))))))))))))))
    ))))))))))))))))))))))));
    //~^^^^^ ERROR E0308

    let x: Atype<
      Btype<
        Ctype<
          Atype<
            Btype<
              Ctype<
                Atype<
                  Btype<
                    Ctype<i32, i32>,
                    i32
                  >,
                  i32
                >,
                i32
              >,
              i32
            >,
            i32
          >,
          i32
        >,
        i32
      >,
      i32
    > = ();
    //~^ ERROR E0308

    let _: () = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok(Ok(Ok(Ok(Ok(Ok(Ok("")))))))
        ))))))))))))))))))))))))))))))
    ))))))))))))))))))))))));
    //~^^^^^ ERROR E0308
}
"#;

    let input_new = Level::Error
        .message("mismatched types")
        .id("E0308")
        .group(Group::new().element(
            Snippet::source(source)
                .line_start(7)
                .origin("$DIR/long-E0308.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(719..1001)
                        .label("expected `Atype<Btype<Ctype<..., i32>, i32>, i32>`, found `Result<Result<Result<..., _>, _>, _>`"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(293..716)
                        .label("expected due to this"),
                )
        ).element(
            Level::Note
                .title("expected struct `Atype<Btype<..., i32>, i32>`\n     found enum `Result<Result<..., _>, _>`")
        ).element(
            Level::Note
                .title("the full name for the type has been written to '$TEST_BUILD_DIR/$FILE.long-type-hash.txt'")
        ).element(
            Level::Note
                .title("consider using `--verbose` to print the full type name to the console")
                ,
        ));

    let expected = str![[r#"
error[E0308]: mismatched types
   ╭▸ $DIR/long-E0308.rs:48:9
   │
LL │        let x: Atype<
   │ ┌─────────────┘
LL │ │        Btype<
LL │ │          Ctype<
LL │ │            Atype<
   ‡ │
LL │ │        i32
LL │ │      > = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(O…
   │ │┏━━━━━│━━━┛
   │ └┃─────┤
   │  ┃     expected due to this
LL │  ┃         Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(O…
LL │  ┃             Ok("")
LL │  ┃         ))))))))))))))))))))))))))))))
LL │  ┃     ))))))))))))))))))))))))))))));
   │  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ expected `Atype<Btype<Ctype<..., i32>, i32>, i32>`, found `Result<Result<Result<..., _>, _>, _>`
   ├ note: expected struct `Atype<Btype<..., i32>, i32>`
   │            found enum `Result<Result<..., _>, _>`
   ├ note: the full name for the type has been written to '$TEST_BUILD_DIR/$FILE.long-type-hash.txt'
   ╰ note: consider using `--verbose` to print the full type name to the console
"#]];
    let renderer = Renderer::plain()
        .term_width(60)
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn highlighting() {
    let source = r#"
use core::pin::Pin;
use core::future::Future;
use core::any::Any;

fn query(_: fn(Box<(dyn Any + Send + '_)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>>) {}

fn wrapped_fn<'a>(_: Box<(dyn Any + Send)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>> {
    Box::pin(async { Err("nope".into()) })
}

fn main() {
    query(wrapped_fn);
}
"#;

    let input_new = Level::Error
        .message("mismatched types")
        .id("E0308")
        .group(Group::new().element(
            Snippet::source(source)
                .line_start(7)
                .origin("$DIR/unicode-output.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(430..440)
                        .label("one type is more general than the other"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(424..429)
                        .label("arguments to this function are incorrect"),
                ),
        ).element(
            Level::Note
                .title("expected fn pointer `for<'a> fn(Box<(dyn Any + Send + 'a)>) -> Pin<_>`\n      found fn item `fn(Box<(dyn Any + Send + 'static)>) -> Pin<_> {wrapped_fn}`")
                ,
        ))
        .group(Group::new().element(
            Level::Note.title("function defined here"),
        ).element(
            Snippet::source(source)
                .line_start(7)
                .origin("$DIR/unicode-output.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(77..210))
                .annotation(AnnotationKind::Context.span(71..76)),
        ));

    let expected = str![[r#"
error[E0308]: mismatched types
   ╭▸ $DIR/unicode-output.rs:23:11
   │
LL │     query(wrapped_fn);
   │     ┬──── ━━━━━━━━━━ one type is more general than the other
   │     │
   │     arguments to this function are incorrect
   │
   ╰ note: expected fn pointer `for<'a> fn(Box<(dyn Any + Send + 'a)>) -> Pin<_>`
                 found fn item `fn(Box<(dyn Any + Send + 'static)>) -> Pin<_> {wrapped_fn}`
note: function defined here
   ╭▸ $DIR/unicode-output.rs:12:10
   │
LL │   fn query(_: fn(Box<(dyn Any + Send + '_)>) -> Pin<Box<(
   │ ┏━━━━─────━┛
LL │ ┃     dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
LL │ ┃ )>>) {}
   ╰╴┗━━━┛
"#]];
    let renderer = Renderer::plain()
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn issue_91334() {
    let source = r#"// Regression test for the ICE described in issue #91334.

//@ error-pattern: this file contains an unclosed delimiter

#![feature(coroutines)]

fn f(){||yield(((){),
"#;
    let input_new = Level::Error
        .message("this file contains an unclosed delimiter")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/issue-91334.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(151..152)
                            .label("unclosed delimiter"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(159..160)
                            .label("unclosed delimiter"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(164..164)
                            .label("missing open `(` for this delimiter"),
                    )
                    .annotation(AnnotationKind::Primary.span(167..167)),
            ),
        );
    let expected = str![[r#"
error: this file contains an unclosed delimiter
  --> $DIR/issue-91334.rs:7:23
   |
LL | fn f(){||yield(((){),
   |       -       -    - ^
   |       |       |    |
   |       |       |    missing open `(` for this delimiter
   |       |       unclosed delimiter
   |       unclosed delimiter
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]

fn issue_114529_illegal_break_with_value() {
    // tests/ui/typeck/issue-114529-illegal-break-with-value.rs
    let source = r#"// Regression test for issue #114529
// Tests that we do not ICE during const eval for a
// break-with-value in contexts where it is illegal

#[allow(while_true)]
fn main() {
    [(); {
        while true {
            break 9; //~ ERROR `break` with value from a `while` loop
        };
        51
    }];

    [(); {
        while let Some(v) = Some(9) {
            break v; //~ ERROR `break` with value from a `while` loop
        };
        51
    }];

    while true {
        break (|| { //~ ERROR `break` with value from a `while` loop
            let local = 9;
        });
    }
}
"#;
    let input_new = Level::Error
        .message("`break` with value from a `while` loop")
        .id("E0571")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/issue-114529-illegal-break-with-value.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(483..581)
                            .label("can only break with a value inside `loop` or breakable block"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(462..472)
                            .label("you can't `break` with a value in a `while` loop"),
                    ),
            ),
        )
        .group(
            Group::new()
                .element(
                    Level::Help
                        .title("use `break` on its own without a value inside this `while` loop"),
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/issue-114529-illegal-break-with-value.rs")
                        .fold(true)
                        .patch(Patch::new(483..581, "break")),
                ),
        );
    let expected = str![[r#"
error[E0571]: `break` with value from a `while` loop
  --> $DIR/issue-114529-illegal-break-with-value.rs:22:9
   |
LL |       while true {
   |       ---------- you can't `break` with a value in a `while` loop
LL | /         break (|| { //~ ERROR `break` with value from a `while` loop
LL | |             let local = 9;
LL | |         });
   | |__________^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
   |
LL -         break (|| { //~ ERROR `break` with value from a `while` loop
LL -             let local = 9;
LL -         });
LL +         break;
   |
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn primitive_reprs_should_have_correct_length() {
    // tests/ui/transmutability/enums/repr/primitive_reprs_should_have_correct_length.rs
    let source = r#"//! An enum with a primitive repr should have exactly the size of that primitive.

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src, {
            Assume {
                alignment: true,
                lifetimes: true,
                safety: true,
                validity: true,
            }
        }>
    {}
}

#[repr(C)]
struct Zst;

#[derive(Clone, Copy)]
#[repr(i8)] enum V0i8 { V }
#[repr(u8)] enum V0u8 { V }
#[repr(i16)] enum V0i16 { V }
#[repr(u16)] enum V0u16 { V }
#[repr(i32)] enum V0i32 { V }
#[repr(u32)] enum V0u32 { V }
#[repr(i64)] enum V0i64 { V }
#[repr(u64)] enum V0u64 { V }
#[repr(isize)] enum V0isize { V }
#[repr(usize)] enum V0usize { V }

fn n8() {
    type Smaller = Zst;
    type Analog = u8;
    type Larger = u16;

    fn i_should_have_correct_length() {
        type Current = V0i8;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u8;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn n16() {
    type Smaller = u8;
    type Analog = u16;
    type Larger = u32;

    fn i_should_have_correct_length() {
        type Current = V0i16;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u16;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn n32() {
    type Smaller = u16;
    type Analog = u32;
    type Larger = u64;

    fn i_should_have_correct_length() {
        type Current = V0i32;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u32;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn n64() {
    type Smaller = u32;
    type Analog = u64;
    type Larger = u128;

    fn i_should_have_correct_length() {
        type Current = V0i64;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u64;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn nsize() {
    type Smaller = u8;
    type Analog = usize;
    type Larger = [usize; 2];

    fn i_should_have_correct_length() {
        type Current = V0isize;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0usize;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}
"#;
    let input_new =
        Level::Error
            .message("`V0usize` cannot be safely transmuted into `[usize; 2]`")
            .id("E0277")
            .group(
                Group::new().element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/primitive_reprs_should_have_correct_length.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(4375..4381).label(
                            "the size of `V0usize` is smaller than the size of `[usize; 2]`",
                        )),
                ),
            )
            .group(
                Group::new()
                    .element(Level::Note.title("required by a bound in `is_transmutable`"))
                    .element(
                        Snippet::source(source)
                            .line_start(1)
                            .origin("$DIR/primitive_reprs_should_have_correct_length.rs")
                            .fold(true)
                            .annotation(
                                AnnotationKind::Context
                                    .span(225..240)
                                    .label("required by a bound in this function"),
                            )
                            .annotation(
                                AnnotationKind::Primary
                                    .span(276..470)
                                    .label("required by this bound in `is_transmutable`"),
                            ),
                    ),
            );
    let expected = str![[r#"
error[E0277]: `V0usize` cannot be safely transmuted into `[usize; 2]`
  --> $DIR/primitive_reprs_should_have_correct_length.rs:144:44
   |
LL |         assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
   |                                            ^^^^^^ the size of `V0usize` is smaller than the size of `[usize; 2]`
   |
note: required by a bound in `is_transmutable`
  --> $DIR/primitive_reprs_should_have_correct_length.rs:12:14
   |
LL |       pub fn is_transmutable<Src, Dst>()
   |              --------------- required by a bound in this function
LL |       where
LL |           Dst: TransmuteFrom<Src, {
   |  ______________^
LL | |             Assume {
LL | |                 alignment: true,
LL | |                 lifetimes: true,
...  |
LL | |         }>
   | |__________^ required by this bound in `is_transmutable`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn temp() {
    let source = r#"//@ check-fail
#![feature(transmutability)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src, {
            Assume {
                alignment: false,
                lifetimes: true,
                safety: true,
                validity: true,
            }
        }>
    {}
}

fn main() {
    assert::is_maybe_transmutable::<&'static [u8; 0], &'static [u16; 0]>(); //~ ERROR `&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`
}
"#;
    let input_new = Level::Error
        .message("`&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .fold(true)
                    .origin("$DIR/align-fail.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(442..459)
                            .label("the minimum alignment of `&[u8; 0]` (1) should be greater than that of `&[u16; 0]` (2)")
                    ),
            ),
        );
    let expected = str![[r#"
error: `&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`
  --> $DIR/align-fail.rs:21:55
   |
LL | ...ic [u8; 0], &'static [u16; 0]>(); //~ ERROR `&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`
   |                ^^^^^^^^^^^^^^^^^ the minimum alignment of `&[u8; 0]` (1) should be greater than that of `&[u16; 0]` (2)
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn object_fail() {
    // tests/ui/traits/alias/object-fail.rs
    let source = r#"#![feature(trait_alias)]

trait EqAlias = Eq;
trait IteratorAlias = Iterator;

fn main() {
    let _: &dyn EqAlias = &123;
    //~^ ERROR the trait alias `EqAlias` is not dyn compatible [E0038]
    let _: &dyn IteratorAlias = &vec![123].into_iter();
    //~^ ERROR must be specified
}
"#;
    let input_new = Level::Error
        .message("the trait alias `EqAlias` is not dyn compatible")
        .id("E0038")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/object-fail.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(107..114)
                            .label("`EqAlias` is not dyn compatible"),
                    ),
            ),
        )
        .group(
            Group::new()
                .element(
                    Level::Note
                        .title("for a trait to be dyn compatible it needs to allow building a vtable\nfor more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>"))
                .element(
                    Origin::new("$SRC_DIR/core/src/cmp.rs")
                        .line(334)
                        .char_column(14)
                        .primary(true).standalone(true)
                        .label("...because it uses `Self` as a type parameter")

                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/object-fail.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(32..39)
                                .label("this trait is not dyn compatible..."),
                        ),
                ),
        );
    let expected = str![[r#"
error[E0038]: the trait alias `EqAlias` is not dyn compatible
  --> $DIR/object-fail.rs:7:17
   |
LL |     let _: &dyn EqAlias = &123;
   |                 ^^^^^^^ `EqAlias` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> $SRC_DIR/core/src/cmp.rs:334:14
   |
   = note: ...because it uses `Self` as a type parameter
   |
  ::: $DIR/object-fail.rs:3:7
   |
LL | trait EqAlias = Eq;
   |       ------- this trait is not dyn compatible...
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn missing_semicolon() {
    // tests/ui/suggestions/missing-semicolon.rs
    let source = r#"//@ run-rustfix
#![allow(dead_code, unused_variables, path_statements)]
fn a() {
    let x = 5;
    let y = x //~ ERROR expected function
    () //~ ERROR expected `;`, found `}`
}

fn b() {
    let x = 5;
    let y = x //~ ERROR expected function
    ();
}
fn c() {
    let x = 5;
    x //~ ERROR expected function
    ()
}
fn d() { // ok
    let x = || ();
    x
    ()
}
fn e() { // ok
    let x = || ();
    x
    ();
}
fn f()
 {
    let y = 5 //~ ERROR expected function
    () //~ ERROR expected `;`, found `}`
}
fn g() {
    5 //~ ERROR expected function
    ();
}
fn main() {}
"#;
    let input_new = Level::Error
        .message("expected function, found `{integer}`")
        .id("E0618")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/missing-semicolon.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(108..144)
                            .label("call expression requires function"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(89..90)
                            .label("`x` has type `{integer}`"),
                    )
                    .annotation(AnnotationKind::Context.span(109..109).label(
                        "help: consider using a semicolon here to finish the statement: `;`",
                    ))
                    .annotation(AnnotationKind::Primary.span(108..109)),
            ),
        );
    let expected = str![[r#"
error[E0618]: expected function, found `{integer}`
  --> $DIR/missing-semicolon.rs:5:13
   |
LL |       let x = 5;
   |           - `x` has type `{integer}`
LL |       let y = x //~ ERROR expected function
   |               ^- help: consider using a semicolon here to finish the statement: `;`
   |  _____________|
   | |
LL | |     () //~ ERROR expected `;`, found `}`
   | |______- call expression requires function
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn nested_macro_rules() {
    // tests/ui/proc-macro/nested-macro-rules.rs
    let source = r#"//@ run-pass
//@ aux-build:nested-macro-rules.rs
//@ proc-macro: test-macros.rs
//@ compile-flags: -Z span-debug -Z macro-backtrace
//@ edition:2018

#![no_std] // Don't load unnecessary hygiene information from std
#![warn(non_local_definitions)]

extern crate std;

extern crate nested_macro_rules;
extern crate test_macros;

use test_macros::{print_bang, print_attr};

use nested_macro_rules::FirstStruct;
struct SecondStruct;

fn main() {
    nested_macro_rules::inner_macro!(print_bang, print_attr);

    nested_macro_rules::outer_macro!(SecondStruct, SecondAttrStruct);
    //~^ WARN non-local `macro_rules!` definition
    inner_macro!(print_bang, print_attr);
}
"#;

    let aux_source = r#"pub struct FirstStruct;

#[macro_export]
macro_rules! outer_macro {
    ($name:ident, $attr_struct_name:ident) => {
        #[macro_export]
        macro_rules! inner_macro {
            ($bang_macro:ident, $attr_macro:ident) => {
                $bang_macro!($name);
                #[$attr_macro] struct $attr_struct_name {}
            }
        }
    }
}

outer_macro!(FirstStruct, FirstAttrStruct);
"#;
    let input_new = Level::Warning
        .message("non-local `macro_rules!` definition, `#[macro_export]` macro should be written at top level module")
        .group(
            Group::new()
                .element(
                    Snippet::source(aux_source)
                        .line_start(1)
                        .origin("$DIR/auxiliary/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(41..65)
                                .label("in this expansion of `nested_macro_rules::outer_macro!`"),
                        )
                        .annotation(AnnotationKind::Primary.span(148..350)),
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(510..574)
                                .label("in this macro invocation"),
                        ),
                )
                .element(
                    Level::Help
                        .title("remove the `#[macro_export]` or move this `macro_rules!` outside the of the current function `main`")
                )
                .element(
                    Level::Note
                        .title("a `macro_rules!` definition is non-local if it is nested inside an item and has a `#[macro_export]` attribute")
                ),
        )
        .group(
            Group::new()
                .element(Level::Note.title("the lint level is defined here"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(224..245)),
                ),
        );
    let expected = str![[r#"
warning: non-local `macro_rules!` definition, `#[macro_export]` macro should be written at top level module
  --> $DIR/auxiliary/nested-macro-rules.rs:7:9
   |
LL |   macro_rules! outer_macro {
   |   ------------------------ in this expansion of `nested_macro_rules::outer_macro!`
...
LL | /         macro_rules! inner_macro {
LL | |             ($bang_macro:ident, $attr_macro:ident) => {
LL | |                 $bang_macro!($name);
LL | |                 #[$attr_macro] struct $attr_struct_name {}
LL | |             }
LL | |         }
   | |_________^
   |
  ::: $DIR/nested-macro-rules.rs:23:5
   |
LL |       nested_macro_rules::outer_macro!(SecondStruct, SecondAttrStruct);
   |       ---------------------------------------------------------------- in this macro invocation
   |
   = help: remove the `#[macro_export]` or move this `macro_rules!` outside the of the current function `main`
   = note: a `macro_rules!` definition is non-local if it is nested inside an item and has a `#[macro_export]` attribute
note: the lint level is defined here
  --> $DIR/nested-macro-rules.rs:8:9
   |
LL | #![warn(non_local_definitions)]
   |         ^^^^^^^^^^^^^^^^^^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn method_on_ambiguous_numeric_type() {
    // tests/ui/methods/method-on-ambiguous-numeric-type.rs
    let source = r#"//@ aux-build:macro-in-other-crate.rs

#[macro_use] extern crate macro_in_other_crate;

macro_rules! local_mac {
    ($ident:ident) => { let $ident = 42; }
}
macro_rules! local_mac_tt {
    ($tt:tt) => { let $tt = 42; }
}

fn main() {
    let x = 2.0.neg();
    //~^ ERROR can't call method `neg` on ambiguous numeric type `{float}`

    let y = 2.0;
    let x = y.neg();
    //~^ ERROR can't call method `neg` on ambiguous numeric type `{float}`
    println!("{:?}", x);

    for i in 0..100 {
        println!("{}", i.pow(2));
        //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`
    }

    local_mac!(local_bar);
    local_bar.pow(2);
    //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`

    local_mac_tt!(local_bar_tt);
    local_bar_tt.pow(2);
    //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`
}

fn qux() {
    mac!(bar);
    bar.pow(2);
    //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`
}
"#;

    let aux_source = r#"#[macro_export]
macro_rules! mac {
    ($ident:ident) => { let $ident = 42; }
}

#[macro_export]
macro_rules! inline {
    () => ()
}
"#;
    let input_new = Level::Error
        .message("can't call method `pow` on ambiguous numeric type `{integer}`")
        .id("E0689")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/method-on-ambiguous-numeric-type.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(916..919)),
            ),
        )
        .group(
            Group::new()
                .element(Level::Help.title("you must specify a type for this binding, like `i32`"))
                .element(
                    Snippet::source(aux_source)
                        .line_start(1)
                        .origin("$DIR/auxiliary/macro-in-other-crate.rs")
                        .fold(true)
                        .patch(Patch::new(69..69, ": i32")),
                ),
        );
    let expected = str![[r#"
error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
  --> $DIR/method-on-ambiguous-numeric-type.rs:37:9
   |
LL |     bar.pow(2);
   |         ^^^
   |
help: you must specify a type for this binding, like `i32`
  --> $DIR/auxiliary/macro-in-other-crate.rs:3:35
   |
LL |     ($ident:ident) => { let $ident: i32 = 42; }
   |                                   +++++
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn issue_42234_unknown_receiver_type() {
    // tests/ui/span/issue-42234-unknown-receiver-type.rs
    let source = r#"//@ revisions: full generic_arg
#![cfg_attr(generic_arg, feature(generic_arg_infer))]

// When the type of a method call's receiver is unknown, the span should point
// to the receiver (and not the entire call, as was previously the case before
// the fix of which this tests).

fn shines_a_beacon_through_the_darkness() {
    let x: Option<_> = None; //~ ERROR type annotations needed
    x.unwrap().method_that_could_exist_on_some_type();
}

fn courier_to_des_moines_and_points_west(data: &[u32]) -> String {
    data.iter()
        .sum::<_>() //~ ERROR type annotations needed
        .to_string()
}

fn main() {}
"#;

    let input_new = Level::Error
        .message("type annotations needed")
        .id("E0282")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/issue-42234-unknown-receiver-type.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(536..539).label(
                        "cannot infer type of the type parameter `S` declared on the method `sum`",
                    )),
            ),
        );
    let expected = str![[r#"
error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:15:10
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `S` declared on the method `sum`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}
#[test]
fn temp2() {
    let source = r##"//@ revisions: normal exhaustive_patterns
//
// This tests a match with no arms on various types.
#![feature(never_type)]
#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![deny(unreachable_patterns)]

fn nonempty<const N: usize>(arrayN_of_empty: [!; N]) {
    macro_rules! match_no_arms {
        ($e:expr) => {
            match $e {}
        };
    }
    macro_rules! match_guarded_arm {
        ($e:expr) => {
            match $e {
                _ if false => {}
            }
        };
    }

    struct NonEmptyStruct1;
    struct NonEmptyStruct2(bool);
    union NonEmptyUnion1 {
        foo: (),
    }
    union NonEmptyUnion2 {
        foo: (),
        bar: !,
    }
    enum NonEmptyEnum1 {
        Foo(bool),
    }
    enum NonEmptyEnum2 {
        Foo(bool),
        Bar,
    }
    enum NonEmptyEnum5 {
        V1,
        V2,
        V3,
        V4,
        V5,
    }
    let array0_of_empty: [!; 0] = [];

    match_no_arms!(0u8); //~ ERROR type `u8` is non-empty
    match_no_arms!(0i8); //~ ERROR type `i8` is non-empty
    match_no_arms!(0usize); //~ ERROR type `usize` is non-empty
    match_no_arms!(0isize); //~ ERROR type `isize` is non-empty
    match_no_arms!(NonEmptyStruct1); //~ ERROR type `NonEmptyStruct1` is non-empty
    match_no_arms!(NonEmptyStruct2(true)); //~ ERROR type `NonEmptyStruct2` is non-empty
    match_no_arms!((NonEmptyUnion1 { foo: () })); //~ ERROR type `NonEmptyUnion1` is non-empty
    match_no_arms!((NonEmptyUnion2 { foo: () })); //~ ERROR type `NonEmptyUnion2` is non-empty
    match_no_arms!(NonEmptyEnum1::Foo(true)); //~ ERROR `NonEmptyEnum1::Foo(_)` not covered
    match_no_arms!(NonEmptyEnum2::Foo(true)); //~ ERROR `NonEmptyEnum2::Foo(_)` and `NonEmptyEnum2::Bar` not covered
    match_no_arms!(NonEmptyEnum5::V1); //~ ERROR `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
    match_no_arms!(array0_of_empty); //~ ERROR type `[!; 0]` is non-empty
    match_no_arms!(arrayN_of_empty); //~ ERROR type `[!; N]` is non-empty

    match_guarded_arm!(0u8); //~ ERROR `0_u8..=u8::MAX` not covered
    match_guarded_arm!(0i8); //~ ERROR `i8::MIN..=i8::MAX` not covered
    match_guarded_arm!(0usize); //~ ERROR `0_usize..` not covered
    match_guarded_arm!(0isize); //~ ERROR `_` not covered
    match_guarded_arm!(NonEmptyStruct1); //~ ERROR `NonEmptyStruct1` not covered
    match_guarded_arm!(NonEmptyStruct2(true)); //~ ERROR `NonEmptyStruct2(_)` not covered
    match_guarded_arm!((NonEmptyUnion1 { foo: () })); //~ ERROR `NonEmptyUnion1 { .. }` not covered
    match_guarded_arm!((NonEmptyUnion2 { foo: () })); //~ ERROR `NonEmptyUnion2 { .. }` not covered
    match_guarded_arm!(NonEmptyEnum1::Foo(true)); //~ ERROR `NonEmptyEnum1::Foo(_)` not covered
    match_guarded_arm!(NonEmptyEnum2::Foo(true)); //~ ERROR `NonEmptyEnum2::Foo(_)` and `NonEmptyEnum2::Bar` not covered
    match_guarded_arm!(NonEmptyEnum5::V1); //~ ERROR `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
    match_guarded_arm!(array0_of_empty); //~ ERROR `[]` not covered
    match_guarded_arm!(arrayN_of_empty); //~ ERROR `[]` not covered
}

fn main() {}
"##;

    let input_new = Level::Error
        .message(
            "non-exhaustive patterns: `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered"
        )
        .id("E0004")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/empty-match.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(2911..2928)
                            .label("patterns `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered")
                    ),
            ),
        )
        .group(
            Group::new()
                .element(Level::Note.title("`NonEmptyEnum5` defined here"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/empty-match.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(818..831))
                        .annotation(AnnotationKind::Context.span(842..844).label("not covered"))
                        .annotation(AnnotationKind::Context.span(854..856).label("not covered"))
                        .annotation(AnnotationKind::Context.span(866..868).label("not covered"))
                        .annotation(AnnotationKind::Context.span(878..880).label("not covered"))
                        .annotation(AnnotationKind::Context.span(890..892).label("not covered"))
                )
                .element(Level::Note.title("the matched value is of type `NonEmptyEnum5`"))
                .element(Level::Note.title("match arms with guards don't count towards exhaustivity"))
        )
        .group(
            Group::new()
                .element(
                    Level::Help
                        .title("ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms")
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/empty-match.rs")
                        .fold(true)
                        .patch(Patch::new(485..485, ",\n                _ => todo!()"))
                )
        );
    let expected = str![[r#"
error[E0004]: non-exhaustive patterns: `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
  --> $DIR/empty-match.rs:71:24
   |
LL |     match_guarded_arm!(NonEmptyEnum5::V1); //~ ERROR `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
   |                        ^^^^^^^^^^^^^^^^^ patterns `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
   |
note: `NonEmptyEnum5` defined here
  --> $DIR/empty-match.rs:38:10
   |
LL |     enum NonEmptyEnum5 {
   |          ^^^^^^^^^^^^^
LL |         V1,
   |         -- not covered
LL |         V2,
   |         -- not covered
LL |         V3,
   |         -- not covered
LL |         V4,
   |         -- not covered
LL |         V5,
   |         -- not covered
   = note: the matched value is of type `NonEmptyEnum5`
   = note: match arms with guards don't count towards exhaustivity
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
   |
LL ~                 _ if false => {},
LL +                 _ => todo!()
   |
"#]];
    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(annotate_snippets::renderer::DEFAULT_TERM_WIDTH + 4);
    assert_data_eq!(renderer.render(input_new), expected);
}
