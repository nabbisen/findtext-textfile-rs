#[cfg(test)]
use findtext_textfile::{core::search::MatchedPart, search};

const TXT_RESOURCE_FILEPATH_02_SJIS: &str = "./tests/fixtures/file2-sjis.txt";

#[test]
fn txt_sjis_found_01() {
    let expect: Vec<MatchedPart> = vec![
        MatchedPart {
            line_num: 3,
            col_pos: 1,
            text: "ヘイ".to_owned(),
        },
        MatchedPart {
            line_num: 4,
            col_pos: 1,
            text: "ヘイヘイ".to_owned(),
        },
        MatchedPart {
            line_num: 4,
            col_pos: 7,
            text: "ヘイヘイ".to_owned(),
        },
    ];

    let ret = search("ヘイ", TXT_RESOURCE_FILEPATH_02_SJIS);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        assert_eq!(
            value.matched, expect,
            "Expected count to be {:?}, got {:?}",
            expect, value.matched
        );
    } else {
        panic!("Unexpected Err value");
    }
}

#[test]
fn txt_sjis_found_02() {
    let expect: Vec<MatchedPart> = vec![MatchedPart {
        line_num: 4,
        col_pos: 1,
        text: "ヘイヘイ".to_owned(),
    }];

    let ret = search("ヘイヘイ", TXT_RESOURCE_FILEPATH_02_SJIS);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        assert_eq!(
            value.matched, expect,
            "Expected count to be {:?}, got {:?}",
            expect, value.matched
        );
    } else {
        panic!("Unexpected Err value");
    }
}

#[test]
fn txt_sjis_missing_01() {
    const EXPECTED_FOUND_COUNT: usize = 0;

    let ret = search("ヘジ", TXT_RESOURCE_FILEPATH_02_SJIS);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        let len = value.matched.len();
        assert_eq!(
            len, EXPECTED_FOUND_COUNT,
            "Expected count to be {}, got {}",
            EXPECTED_FOUND_COUNT, len
        );
    } else {
        panic!("Unexpected Err value");
    }
}
