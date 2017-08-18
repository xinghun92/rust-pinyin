extern crate pinyin;

struct TestCase {
    result: Vec<Vec<String>>,
}

impl TestCase {
    pub fn new(result: Vec<Vec<String>>) -> TestCase {
        TestCase{result: result}
    }
}


#[test]
fn test_pinyin() {
    let hans = "中国人";
    let test_data = vec![
        TestCase::new(
            vec![vec!["zhong".to_string()],
                 vec!["guo".to_string()],
                 vec!["ren".to_string()]
                 ]
        ),
    ];
    for data in &test_data {
         assert_eq!(data.result, pinyin::pinyin(hans));
    }
}
