use std::collections::HashSet;

fn product_suggestions(product_prices: Vec<i32>, amount: i32) -> Vec<Vec<i32>> {
    let mut prices_hash = HashSet::new();
    let mut offers = Vec::new();

    for i in product_prices {
        let diff = amount - i;

        if prices_hash.get(&diff).is_none() {
            prices_hash.insert(i);
        } else {
            offers.push(vec![i, diff]);
        }
    }

    offers
}

fn main() {
    let products = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];

    println!("{:?}", product_suggestions(products, 50));
}
