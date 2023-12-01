use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string("input")?;

    let input_lines = input.split("\n");

    let mut calibration_total: i64 = 0;
    let mut full_calibration_total: i64 = 0;

    let _: Vec<_> = input_lines
        .into_iter()
        .map(|line| {
            if line == "" {
                return;
            }
            let x = Line::new((String::from(line) + ".").as_str());
            calibration_total += x.calibration().unwrap();
            full_calibration_total += x.calibration_full().unwrap();
        })
        .collect();

    println!("Calibration Total is {calibration_total}");
    println!("Full Calibration Total is {full_calibration_total}");
    Ok(())
}

pub struct Line {
    text: String,
}

impl Line {
    pub fn new(data: &str) -> Line {
        Line {
            text: String::from(data),
        }
    }

    pub fn calibration_full(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let mut first_value: i64 = 0;

        let mut first_pos = self
            .text
            .find(|x: char| -> bool {
                if x.is_numeric() {
                    first_value = String::from(x).parse::<i64>().unwrap();
                    return true;
                }

                return false;
            })
            .unwrap_or_else(|| 9999999999999);

        if let Some(word_pos) = self.text.as_str().find("one") {
            if word_pos < first_pos {
                first_value = 1;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("two") {
            if word_pos < first_pos {
                first_value = 2;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("three") {
            if word_pos < first_pos {
                first_value = 3;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("four") {
            if word_pos < first_pos {
                first_value = 4;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("five") {
            if word_pos < first_pos {
                first_value = 5;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("six") {
            if word_pos < first_pos {
                first_value = 6;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("seven") {
            if word_pos < first_pos {
                first_value = 7;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("eight") {
            if word_pos < first_pos {
                first_value = 8;
                first_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().find("nine") {
            if word_pos < first_pos {
                first_value = 9;
                first_pos = word_pos;
            }
        }

        let mut last_value: i64 = 0;

        let mut last_pos = self
            .text
            .rfind(|x: char| -> bool {
                if x.is_numeric() {
                    last_value = String::from(x).parse::<i64>().unwrap();
                    return true;
                }

                return false;
            })
            .unwrap_or_else(|| 0);

        if let Some(word_pos) = self.text.as_str().rfind("one") {
            if word_pos > last_pos {
                last_value = 1;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("two") {
            if word_pos > last_pos {
                last_value = 2;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("three") {
            if word_pos > last_pos {
                last_value = 3;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("four") {
            if word_pos > last_pos {
                last_value = 4;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("five") {
            if word_pos > last_pos {
                last_value = 5;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("six") {
            if word_pos > last_pos {
                last_value = 6;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("seven") {
            if word_pos > last_pos {
                last_value = 7;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("eight") {
            if word_pos > last_pos {
                last_value = 8;
                last_pos = word_pos;
            }
        }
        if let Some(word_pos) = self.text.as_str().rfind("nine") {
            if word_pos > last_pos {
                last_value = 9;
                last_pos = word_pos;
            }
        }

        println!("{:?}", self.text);
        println!("{first_value} + {last_value}");
        let calibration_value = format!("{first_value}{last_value}");

        Ok(calibration_value.parse::<i64>().unwrap())
    }

    pub fn calibration(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let first_pos = self
            .text
            .find(|x: char| -> bool {
                if let Ok(_) = String::from(x).parse::<i64>() {
                    return true;
                }

                return false;
            })
            .unwrap_or_default();

        let last_pos = self
            .text
            .rfind(|x: char| -> bool {
                if let Ok(_) = String::from(x).parse::<i64>() {
                    return true;
                }

                return false;
            })
            .unwrap_or_default();

        let first_val = self.text.chars().nth(first_pos).unwrap();
        let last_val = self.text.chars().nth(last_pos).unwrap();

        let calibration_value = format!("{first_val}{last_val}");

        Ok(calibration_value.parse::<i64>().unwrap())
    }
}
