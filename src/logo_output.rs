use termlogo::logo::LOGO;
use termlogo::writer::LogoWriter;

#[test]
fn test_logo_output_matches_expected() {
    let mut buffer = Vec::new();
    {
        let mut writer = LogoWriter::new(&mut buffer);
        writer.write_logo(LOGO).expect("Failed to write logo");
    }

    let output = String::from_utf8(buffer).expect("Invalid UTF-8 output");
    assert_eq!(output, LOGO);
}
