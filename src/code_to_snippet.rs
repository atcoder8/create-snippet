use itertools::join;

/// List of escapes required when converting from code to snippets.
const ESCAPES: [(&str, &str); 3] = [(r"\", r"\\"), (r#"""#, r#"\""#), (r"$", r"\\$")];

fn escape_code(line: &str) -> String {
    let mut escaped_code = line.to_owned();

    for (from, to) in ESCAPES {
        escaped_code = escaped_code.replace(from, to);
    }

    escaped_code
}

pub fn code_to_snippet_body(code: &[String]) -> String {
    if code.is_empty() {
        String::from("\t\t\"body\": \"\",")
    } else if code.len() == 1 {
        format!("\t\t\"body\": \"{}\",", escape_code(&code[0]))
    } else {
        let converted_code = code
            .iter()
            .map(|line| format!("\t\t\t\"{}\",", escape_code(line)));
        let converted_code = join(converted_code, "\n");

        format!("\t\t\"body\": [\n{}\n\t\t],", converted_code)
    }
}

pub fn code_to_snippet_body_as_module(code: &[String], module_name: &str) -> String {
    let converted_code = code
        .iter()
        .map(|line| format!("\t\t\t\"    {}\",", escape_code(line)));
    let converted_code = join(converted_code, "\n");

    format!(
        "\
\t\t\"body\": [
\t\t\t\"pub mod {} {{\",
{}
\t\t\t\"}}\",
\t\t],\
",
        module_name, converted_code,
    )
}

pub fn create_snippet(
    code: &[String],
    name: &str,
    prefix: &str,
    description: &str,
    template: bool,
    module_name: Option<&str>,
) -> String {
    let snippet_body = if let Some(module_name) = module_name {
        code_to_snippet_body_as_module(code, module_name)
    } else {
        code_to_snippet_body(code)
    };

    format!(
        "\
\t\"{}\": {{
\t\t\"prefix\": \"{}\",
{}
\t\t\"description\": \"{}\",
\t\t\"isFileTemplate\": {},
\t}},",
        name, prefix, snippet_body, description, template,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_code() {
        assert_eq!(escape_code(r#"$$\\"""#), r#"\\$\\$\\\\\"\""#,)
    }

    #[test]
    fn test_empty_code_to_snippet() {
        assert_eq!(code_to_snippet_body(&[r"".to_owned()]), r#"		"body": "","#,)
    }

    #[test]
    fn test_one_line_code_to_snippet_1() {
        assert_eq!(
            code_to_snippet_body(&[r"hello world!!".to_owned()]),
            r#"		"body": "hello world!!","#,
        )
    }

    #[test]
    fn test_one_line_code_to_snippet_2() {
        assert_eq!(
            code_to_snippet_body(&[r#"\"$"#.to_owned()]),
            r#"		"body": "\\\"\\$","#,
        )
    }

    #[test]
    fn test_multiple_line_code_to_snippet() {
        assert_eq!(
            code_to_snippet_body(&[
                r"fn main() {".to_owned(),
                r"    // comment".to_owned(),
                r#"    println!("$ echo \"hello world!!\"");"#.to_owned(),
                r"}".to_owned(),
            ]),
            r#"		"body": [
			"fn main() {",
			"    // comment",
			"    println!(\"\\$ echo \\\"hello world!!\\\"\");",
			"}",
		],"#
        )
    }

    #[test]
    fn test_one_line_code_to_snippet_as_module() {
        assert_eq!(
            code_to_snippet_body_as_module(&[r"pub struct Value(i32);".to_owned()], "value"),
            r#"		"body": [
			"pub mod value {",
			"    pub struct Value(i32);",
			"}",
		],"#,
        )
    }

    #[test]
    fn test_multiple_line_code_to_snippet_as_module() {
        assert_eq!(
            code_to_snippet_body_as_module(
                &[
                    r"pub struct Coordinate {".to_owned(),
                    r"    pub x: i32,".to_owned(),
                    r"    pub y: i32,".to_owned(),
                    r"}".to_owned(),
                ],
                "coordinate"
            ),
            r#"		"body": [
			"pub mod coordinate {",
			"    pub struct Coordinate {",
			"        pub x: i32,",
			"        pub y: i32,",
			"    }",
			"}",
		],"#,
        )
    }

    #[test]
    fn test_create_snippet() {
        let code = [
            r"pub struct Coordinate {".to_owned(),
            r"    pub x: i32,".to_owned(),
            r"    pub y: i32,".to_owned(),
            r"}".to_owned(),
        ];

        let snippet = create_snippet(
            &code,
            "[module] coordinate",
            "module-coordinate",
            "Structure for two-dimensional coordinate.",
            false,
            Some("coordinate"),
        );

        assert_eq!(
            snippet,
            r#"	"[module] coordinate": {
		"prefix": "module-coordinate",
		"body": [
			"pub mod coordinate {",
			"    pub struct Coordinate {",
			"        pub x: i32,",
			"        pub y: i32,",
			"    }",
			"}",
		],
		"description": "Structure for two-dimensional coordinate.",
		"isFileTemplate": false,
	},"#,
        )
    }
}
