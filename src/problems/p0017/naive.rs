fn number_words(n: i64) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        _ => String::new(), // should never reach here
    }
}

fn number_in_english_1000(n: i64) -> String {
    if n == 1000 {
        return String::from("one thousand");
    }

    let mut m = n;
    let mut result = Vec::new();
    if n >= 100 {
        let hundreds = m / 100;
        result.push(number_words(hundreds));
        result.push(String::from("hundred"));
        m %= 100;
    }

    if m != 0 {
        if !result.is_empty() {
            result.push(String::from("and"));
        }

        if m >= 20 {
            let tens = m / 10 * 10;
            m %= 10;

            result.push(number_words(tens));
            if m != 0 {
                result.push(String::from("-"));
                result.push(number_words(m));
            }
        } else if m > 0 {
            result.push(number_words(m));
        }
    }

    result.join(" ")
}

fn count_letters(s: &str) -> usize {
    s.chars().filter(|c| 'a' <= *c && *c <= 'z').count()
}

pub fn solve() -> i64 {
    let mut result = 0;

    for i in 1..=1000 {
        let s = number_in_english_1000(i);
        let c = count_letters(&s) as i64;
        result += c;
    }

    result
}
