/// Определяет CLDR plural-форму для числа в заданном языке.
///
/// Поддерживаемые языки: ru, en, de, fr, pl, uk, be, ja, zh, ko
pub fn plural_form(lang: &str, n: f64) -> &'static str {
    let n = (n * 100.0).round() / 100.0;
    let i = n.trunc() as i64;
    let v = {
        let s = format!("{n:.2}");
        if let Some(dot) = s.find('.') {
            s[dot + 1..].trim_end_matches('0').len()
        } else {
            0
        }
    };
    let i_mod10 = i % 10;
    let i_mod100 = i % 100;

    match lang {
        "ru" | "uk" | "be" => {
            if v == 0 && i_mod10 == 1 && i_mod100 != 11 {
                "one"
            } else if v == 0 && (2..=4).contains(&i_mod10) && !(12..=14).contains(&i_mod100) {
                "few"
            } else if v == 0
                && (i_mod10 == 0 || (5..=9).contains(&i_mod10) || (11..=14).contains(&i_mod100))
            {
                "many"
            } else {
                "other"
            }
        }
        "en" | "de" if i == 1 && v == 0 => "one",
        "fr" if (i == 0 || i == 1) => "one",
        "pl" => {
            if v == 0 && i == 1 {
                "one"
            } else if v == 0 && (2..=4).contains(&i_mod10) && !(12..=14).contains(&i_mod100) {
                "few"
            } else if v == 0 && (i != 1) && (0..=1).contains(&i_mod10)
                || (5..=9).contains(&i_mod10)
                || (12..=14).contains(&i_mod100)
            {
                "many"
            } else {
                "other"
            }
        }
        "ja" | "zh" | "ko" => "other",
        _ => "other",
    }
}

#[allow(dead_code)]
pub fn plural_key(lang: &str, key: &str, n: f64) -> String {
    format!("{key}.{}.{lang}", plural_form(lang, n), lang = lang)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural_ru_all_forms() {
        assert_eq!(plural_form("ru", 1.0), "one");
        assert_eq!(plural_form("ru", 2.0), "few");
        assert_eq!(plural_form("ru", 5.0), "many");
        assert_eq!(plural_form("ru", 0.0), "many");
        assert_eq!(plural_form("ru", 21.0), "one");
        assert_eq!(plural_form("ru", 11.0), "many");
        assert_eq!(plural_form("ru", 1.5), "other");
    }

    #[test]
    fn test_plural_en() {
        assert_eq!(plural_form("en", 1.0), "one");
        assert_eq!(plural_form("en", 2.0), "other");
        assert_eq!(plural_form("en", 0.0), "other");
    }

    #[test]
    fn test_plural_de() {
        assert_eq!(plural_form("de", 1.0), "one");
        assert_eq!(plural_form("de", 2.0), "other");
    }

    #[test]
    fn test_plural_fr() {
        assert_eq!(plural_form("fr", 0.0), "one");
        assert_eq!(plural_form("fr", 1.0), "one");
        assert_eq!(plural_form("fr", 2.0), "other");
    }

    #[test]
    fn test_plural_ja() {
        assert_eq!(plural_form("ja", 1.0), "other");
        assert_eq!(plural_form("ja", 100.0), "other");
    }

    #[test]
    fn test_plural_key() {
        let key = plural_key("ru", "files", 1.0);
        assert_eq!(key, "files.one.ru");
        let key = plural_key("ru", "files", 5.0);
        assert_eq!(key, "files.many.ru");
    }
}
