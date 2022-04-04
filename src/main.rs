use std::{collections::HashMap, fs, ops::Add};

use regex::Regex;

// Keywords: say, assign

fn main() {
    let text = fs::read_to_string("first.ss").unwrap();

    let lines: Vec<&str> = text.split(";").collect();

    let mut vars: HashMap<String, String> = HashMap::new();

    for statement in lines {
        if statement.contains("say") {
            let mut printable = statement
                .replace("say ", "")
                .replace("\n", "")
                .trim()
                .to_string();

            let mut run_count = 1;

            if statement.contains("times") {
                let re = Regex::new(r"\(([0-9]+) times\)").unwrap();

                let times = re
                    .captures(statement)
                    .unwrap()
                    .get(1)
                    .map_or("", |m| m.as_str());

                run_count = times.parse::<i128>().expect("times must be a number");
                printable = re.replace_all(statement, "").replace("\n", "");
            }

            if printable.contains("-") || printable.contains("+") {
                let parts: Vec<&str> = printable.split(" ").collect();
                let starting = parts[0].trim().parse::<i128>().expect("must be number");

                let result: i128 =
                    parts
                        .iter()
                        .skip(1)
                        .enumerate()
                        .fold(starting, |acc, (i, p)| {
                            let token = p.trim();

                            match token {
                                "+" | "-" => acc,

                                _ => {
                                    // Don't have to do i - 1 because we're skipping the first value (I think???)
                                    let operator = parts[i];

                                    let n = p
                                        .trim()
                                        .parse::<i128>()
                                        .expect("can't subtract non-number values");

                                    match operator {
                                        "+" => acc + n,
                                        "-" => acc - n,
                                        _ => acc,
                                    }
                                }
                            }
                        });

                for _ in 0..run_count {
                    println!("{}", result);
                }
            } else {
                if !printable.contains(r#"""#) {
                    let var_name = printable.replace(" ", "");
                    let val = vars.get(&var_name).expect("variable not found");

                    for _ in 0..run_count {
                        println!("{val}");
                    }
                } else {
                    for _ in 0..run_count {
                        println!("{printable}");
                    }
                }
            }
        }

        if statement.contains("assign") {
            let working = statement.replace("assign ", "").replace("\n", "");
            let working: Vec<&str> = working.split(" to ").collect();
            let name = String::from(working[0].trim());
            let value = String::from(working[1].trim());

            vars.insert(name, value);
        }
    }
}
