use itertools::Itertools;
use std::fmt::Write;

fn main() {
    println!("Hello, world!");
}

fn to_string_and_join(values: &[usize]) -> String {
    values
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn fold_by_to_string_without_capacity(values: &[usize]) -> String {
    values
        .iter()
        .map(|s| s.to_string())
        .fold(String::new(), |mut acc, cur| {
            acc.push_str(&cur);
            acc.push_str(" ");
            acc
        })
}

fn fold_by_to_string_with_capacity(values: &[usize]) -> String {
    values
        .iter()
        .map(|s| s.to_string())
        // 数値一つが最大6桁 + separator1文字で7文字
        .fold(String::with_capacity(7 * values.len()), |mut acc, cur| {
            acc.push_str(&cur);
            acc.push_str(" ");
            acc
        })
}

fn itertools_join(values: &[usize]) -> String {
    values.iter().join(" ")
}

fn fold_by_write_without_capacity(values: &[usize]) -> String {
    values.iter().fold(String::new(), |mut acc, cur| {
        write!(&mut acc, "{} ", cur).unwrap();
        acc
    })
}

fn fold_by_write_with_capacity(values: &[usize]) -> String {
    values
        .iter()
        // 数値一つが最大6桁 + separator1文字で7文字
        .fold(String::with_capacity(7 * values.len()), |mut acc, cur| {
            write!(&mut acc, "{} ", cur).unwrap();
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = (1..10).collect::<Vec<_>>();

        let results = [
            to_string_and_join(&arr),
            fold_by_to_string_without_capacity(&arr),
            fold_by_to_string_with_capacity(&arr),
            itertools_join(&arr),
            fold_by_write_without_capacity(&arr),
            fold_by_write_with_capacity(&arr),
        ];

        assert!(
            results
                .iter()
                .map(|s| {
                    // 末尾の空白除去
                    s.trim()
                })
                .all_equal(),
            "{:?}",
            results
        );
    }
}
