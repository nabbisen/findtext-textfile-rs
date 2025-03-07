use findtext_textfile::{core::search::MatchedPart, search};

const CSV_RESOURCE_FILEPATH_01: &str = "./tests/fixtures/file1.csv";

#[test]
fn csv_found_01() {
    let expect: Vec<MatchedPart> = vec![
        MatchedPart {
            line_num: 1,
            col_pos: 4,
            text: "1,\"hej\",2".to_owned(),
        },
        MatchedPart {
            line_num: 2,
            col_pos: 4,
            text: "3,\"hejhej\",4".to_owned(),
        },
        MatchedPart {
            line_num: 2,
            col_pos: 7,
            text: "3,\"hejhej\",4".to_owned(),
        },
    ];

    let ret = search("hej", CSV_RESOURCE_FILEPATH_01);

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
fn csv_found_02() {
    let expect: Vec<MatchedPart> = vec![MatchedPart {
        line_num: 2,
        col_pos: 4,
        text: "3,\"hejhej\",4".to_owned(),
    }];

    let ret = search("hejhej", CSV_RESOURCE_FILEPATH_01);

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
fn csv_missing_01() {
    const EXPECTED_FOUND_COUNT: usize = 0;

    let ret = search("heJ", CSV_RESOURCE_FILEPATH_01);

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
