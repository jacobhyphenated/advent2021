use std::collections::HashMap;

pub fn stock_list(stock: Vec<&str>, categories: Vec<&str>) -> String {
  if stock.is_empty() || categories.is_empty() {
    return String::from("");
  }
  let mut stock_map: HashMap<&str, u32> = HashMap::new();
  for s in stock {
    let mut stock_iter = s.split_whitespace();
    let code = &stock_iter.next().unwrap()[..1];
    let quantity = stock_iter.next().unwrap().parse::<u32>().unwrap();
    *stock_map.entry(code).or_insert(0) += quantity;
  }
  categories.iter()
    .map(|cat| format!("({} : {})", cat, stock_map.get(cat).unwrap_or(&0)))
    .collect::<Vec<_>>()
    .join(" - ")
}