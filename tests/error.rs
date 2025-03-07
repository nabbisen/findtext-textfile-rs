use findtext_textfile::search;

const TXT_DUMMY_FILEPATH_01: &str = "./tests/fixtures/_file0.txt";

#[test]
fn error_01() {
    let ret = search("hej", TXT_DUMMY_FILEPATH_01);

    assert!(ret.is_err(), "Expected Err, got {:?}", ret);
}
