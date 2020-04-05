use regex::Regex;

// pub fn new(lines: Vec<String>) {
//     for l in lines {
//         let deleteL = deleteComment(l);
//     }
// }

fn deleteComment(line: String) -> String {
    let re = Regex::new(r"[/]{2}.+$").unwrap();
    let result = re.replace_all(line.as_str(), "");
    return result.to_string();
}

#[test]
fn testDeleteComment() {
    struct Test {
        input: String,
        expected: String,
    }

    let tests = [Test {
        input: String::from("hoge"),
        expected: String::from("hoge"),
    }, Test {
        input: String::from("// comment"),
        expected: String::from(""),
    }, Test {
        input: String::from("hoge// comment"),
        expected: String::from("hoge"),
    }];

    for t in tests.iter() {
        let res = deleteComment(t.input.clone());
        assert_eq!(res, t.expected);
    }
}