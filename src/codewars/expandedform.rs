fn expanded_form(n: u64) -> String {
    let mut expanded: Vec<String> = n.to_string().chars()
        .rev()
        .enumerate()
        .map(|(index, c)| format!("{}{}", c, "0".repeat(index)))
        .filter(|val| val.parse::<i32>().unwrap_or(0) != 0)
        .collect();
    expanded.reverse();
    expanded.join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}