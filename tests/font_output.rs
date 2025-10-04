use termlogo::ascii_font::render_text;

#[test]
fn renders_letter_a_correctly() {
    let output = render_text("A");

    let expected = [
        " ##   ",
        "#  #  ",
        "####  ",
        "#  #  ",
        "#  #  ",
    ].join("\n");

    assert_eq!(output, expected);
}

#[test]
fn renders_unknown_char_as_question_mark() {
    let output = render_text("@");

    let expected = [
        "###   ",
        "   #  ",
        " ##   ",
        "      ",
        " #    ",
    ].join("\n");

    assert_eq!(output, expected);
}
