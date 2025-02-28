use annotate_snippets::{Annotation, AnnotationKind, Level, Patch, Renderer, Snippet};

use annotate_snippets::renderer::OutputTheme;
use snapbox::{assert_data_eq, str};

#[test]
fn test_i_29() {
    let snippets = Level::Error.message("oops").section(
        Snippet::source("First line\r\nSecond oops line")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(19..23).label("oops"))
            .fold(true),
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
    let snippets = Level::Error.message("").section(
        Snippet::source("こんにちは、世界")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(18..24).label("world")),
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
    let snippets = Level::Error.message("").section(
        Snippet::source("おはよう\nございます")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(6..22).label("Good morning")),
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
    let snippets = Level::Error.message("").section(
        Snippet::source("お寿司\n食べたい🍣")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(0..9).label("Sushi1"))
            .annotation(AnnotationKind::Context.span(16..22).label("Sushi2")),
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
    let snippets = Level::Error.message("").section(
        Snippet::source("こんにちは、新しいWorld！")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(18..32).label("New world")),
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
        .section(Snippet::<Annotation<'_>>::source(source).line_start(5402));

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
    let input = Level::Error
        .message("")
        .section(
            Snippet::<Annotation<'_>>::source(src_0)
                .line_start(5402)
                .origin("file1.rs"),
        )
        .section(
            Snippet::<Annotation<'_>>::source(src_1)
                .line_start(2)
                .origin("file2.rs"),
        );
    let expected = str![[r#"
error: 
    --> file1.rs
     |
5402 | This is slice 1
     |
    ::: file2.rs
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
    let input = Level::Error.message("").section(
        Snippet::source(&source).line_start(5402).annotation(
            AnnotationKind::Context
                .span(range.clone())
                .label("Test annotation"),
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
        .section(Level::Error.title("This __is__ a title"));
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
    let input = Level::Error.message("").section(
        Snippet::source(source).line_start(0).annotation(
            AnnotationKind::Primary
                .span(0..source.len() + 2)
                .label(label),
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
        .section(Snippet::<Annotation<'_>>::source(source).line_start(56));
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .line_start(1)
            .annotation(AnnotationKind::Context.span(0..5).label("Example string")),
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .line_start(1)
            .annotation(AnnotationKind::Context.span(0..5).label("Example string"))
            .annotation(AnnotationKind::Context.span(0..5).label("Second line")),
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
        .section(Snippet::<Annotation<'_>>::source("").origin("file.rs"));
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
        .section(Snippet::<Annotation<'_>>::source(source).line_start(56));
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
    let input = Level::Error.message("dummy").section(
        Snippet::source("foo\nbar\nbaz")
            .origin("file/path")
            .line_start(3)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(4..11)), // bar\nbaz
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(0..10)), // 1..10 works
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(0..2)), // a\r
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(0..3)), // a\r\n
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
    let snippets = Level::Error.message("").section(
        Snippet::source("こん\r\nにちは\r\n世界")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(3..8)), // ん\r\n
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..2)), // \r
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..3)), // \r\n
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..3)), // \n
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
fn annotate_eol4() {
    let source = "a\r\nb";
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..2)), // \n
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
fn annotate_eol_double_width() {
    let snippets = Level::Error.message("").section(
        Snippet::source("こん\r\nにちは\r\n世界")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(7..8)), // \n
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:3
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..4)), // \r\nb
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..4)), // \nb
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
fn multiline_eol_start3() {
    let source = "a\nb";
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..3)), // \nb
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
    let snippets = Level::Error.message("").section(
        Snippet::source("こん\r\nにちは\r\n世界")
            .origin("<current file>")
            .annotation(AnnotationKind::Primary.span(7..11)), // \r\nに
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:3
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..4)), // \nb\n
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..5)), // \nb\r
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
fn multiline_eol_start_eol_end3() {
    let source = "a\r\nb\r\nc";
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..6)), // \nb\r\n
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
fn multiline_eol_start_eof_end() {
    let source = "a\r\nb";
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..5)), // \r\nb(EOF)
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
    let input = Level::Error.message("").section(
        Snippet::source(source)
            .origin("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(3..9)), // \r\nに(EOF)
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
    let input = Level::Error.message("unused optional dependency").section(
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
    let input = Level::Error.message("unused optional dependency").section(
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
    let input = Level::Error.message("unused optional dependency").section(
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
    let input = Level::Error.message("unused optional dependency").section(
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
    let input = Level::Error.message("title").section(
        Snippet::source(source)
            .origin("origin.txt")
            .fold(false)
            .annotation(AnnotationKind::Primary.span(8..8 + 3).label("annotation")),
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
    let input = Level::Error.message("title").section(
        Snippet::source(source)
            .origin("origin.txt")
            .fold(false)
            .annotation(
                AnnotationKind::Primary
                    .span(8 + 1..8 + 3)
                    .label("annotation"),
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
        .section(
            Snippet::source(source)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(4..5)),
        )
        .section(
            Level::Help.title("you might have meant to use one of the following enum variants"),
        )
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(4..5, "(A::Tuple())")),
        )
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(4..5, "A::Unit")),
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
    let input_new = Level::Error
        .message("no method named `pick` found for struct `Chaenomeles` in the current scope")
        .id("E0599")
        .section(
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
                )
        )
        .section(Level::Help.title(
            "the following traits which provide `pick` are implemented but not in scope; perhaps you want to import one of them"
        ))
        .section(
            Snippet::source(source).fold(true)
                .patch(Patch::new(1..1, "use banana::Apple;\n"))
        )
        .section(
            Snippet::source(source).fold(true)
                .patch(Patch::new(1..1, "use banana::Peach;\n"))
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
   |
LL + use banana::Peach;
   |"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn single_line_non_overlapping_suggestions() {
    let source = r#"    A.foo();"#;

    let input_new = Level::Error
        .message("expected value, found enum `A`")
        .id("E0423")
        .section(
            Snippet::source(source)
                .fold(true)
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..5)),
        )
        .section(Level::Help.title("make these changes and things will work"))
        .section(
            Snippet::source(source)
                .fold(true)
                .fold(true)
                .patch(Patch::new(4..5, "(A::Tuple())"))
                .patch(Patch::new(6..9, "bar")),
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
        .section(
            Snippet::source(source)
                .fold(true)
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..18)),
        )
        .section(Level::Help.title("make these changes and things will work"))
        .section(
            Snippet::source(source)
                .fold(true)
                .fold(true)
                .patch(Patch::new(4..18, "(A::Tuple())"))
                .patch(Patch::new(19..22, "bar")),
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
        .section(
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
        )
        .section(Level::Help.title("try explicitly pass `&Self` into the Closure as an argument"))
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(14..14, "this: &Self"))
                .patch(Patch::new(26..30, "this"))
                .patch(Patch::new(66..68, "(self)")),
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
        .section(
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
        )
        .section(Level::Help.title(
            "if you want to call `next` on a iterator within the loop, consider using `while let`",
        ))
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(
                    55..59,
                    "let iter = chars.by_ref();\n    while let Some(",
                ))
                .patch(Patch::new(61..79, ") = iter.next()"))
                .patch(Patch::new(90..95, "iter")),
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
        .section(
            Snippet::source(source).line_start(1).fold(true).annotation(
                AnnotationKind::Primary
                    .span(122..124)
                    .label("use of undeclared crate or module `st`"),
            ),
        )
        .section(Level::Help.title("there is a crate or module with a similar name"))
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(122..124, "std")),
        )
        .section(Level::Help.title("consider importing this module"))
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(1..1, "use std::cell;\n")),
        )
        .section(Level::Help.title("if you import `cell`, refer to it directly"))
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(122..126, "")),
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
        .section(
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
        )
        .section(
            Level::Help
                .title("consider removing the `?Sized` bound to make the type parameter `Sized`"),
        )
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(52..86, "")),
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
        .section(
            Snippet::source(source)
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
                )
        )
        .section(
            Level::Note.title("required by an implicit `Sized` bound in `Wrapper`")
        )
        .section(
            Snippet::source(source)
                .line_start(1)
                .origin("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(16..17)
                        .label("required by the implicit `Sized` requirement on this type parameter in `Wrapper`"),
                ),
        )
        .section(
            Level::Help.title("you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`")
        )
        .section(
            Snippet::source(source)
                .line_start(1)
                .origin("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(16..17)
                        .label("this could be changed to `T: ?Sized`...")
                )
                .annotation(
                    AnnotationKind::Context
                        .span(19..20)
                        .label("...if indirection were used here: `Box<T>`")
                ),
        )
        .section(
            Level::Help.title(
                "consider removing the `?Sized` bound to make the type parameter `Sized`",
            )
        )
        .section(
            Snippet::source(source)
                .fold(true)
                .patch(Patch::new(56..90, ""))
                .patch(Patch::new(90..90, "+ Send"))
        );
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
LL - and where
LL -     T
LL -     :
LL -     ?
LL -     Sized
LL + and + Send
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
        .section(
            Level::Help
                .title("consider removing the `?Sized` bound to make the type parameter `Sized`"),
        )
        .section(
            Snippet::source(source)
                .line_start(7)
                .fold(true)
                .patch(Patch::new(3..21, ""))
                .patch(Patch::new(22..40, "")),
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
12 - crazy
13 - quack
14 - zappy
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
        .section(
            Snippet::source(source)
                .line_start(4)
                .origin("$DIR/E0271.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(208..510)
                        .label("type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`"),
                ),
        )
        .section(
            Level::Note.title("expected this to be `Foo`")
        )
        .section(
            Snippet::source(source)
                .line_start(4)
                .origin("$DIR/E0271.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(89..90)
                ),
        )
        .section(
            Level::Note.title("required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`")

        );

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
        .section(
            Snippet::source(source)
                .line_start(4)
                .origin("$DIR/E0271.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(208..510)
                        .label("type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`"),
                ),
        )
        .section(
            Level::Note.title("expected this to be `Foo`")
        )
        .section(
            Snippet::source(source)
                .line_start(4)
                .origin("$DIR/E0271.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(89..90)
                ),
        )
        .section(
            Level::Note.title("required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`")
        ).section(
            Level::Note.title("a second note")
        );

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
        .section(
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
                ),
        )
        .section(
            Level::Note.title("expected struct `Atype<Btype<..., i32>, i32>`\n     found enum `Result<Result<..., _>, _>`")
        )
        .section(
            Level::Note.title("the full name for the type has been written to '$TEST_BUILD_DIR/$FILE.long-type-hash.txt'")
        )
        .section(
            Level::Note.title("consider using `--verbose` to print the full type name to the console")
        );

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
   │
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
        .section(
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
        )
        .section(
            Level::Note.title("expected fn pointer `for<'a> fn(Box<(dyn Any + Send + 'a)>) -> Pin<_>`\n      found fn item `fn(Box<(dyn Any + Send + 'static)>) -> Pin<_> {wrapped_fn}`")
        )
        .section(
            Level::Note.title("function defined here")
        )
        .section(
            Snippet::source(source)
                .line_start(7)
                .origin("$DIR/unicode-output.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(77..210),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(71..76)
                ),
        );

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
