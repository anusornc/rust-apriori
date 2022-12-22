mod apriori;

fn main() {
    let transactions = vec![
        vec!["apple".to_string(), "banana".to_string()],
        vec!["apple".to_string(), "orange".to_string()],
        vec!["banana".to_string(), "orange".to_string(), "apple".to_string()],
        vec!["banana".to_string()],
    ];
    let frequent_itemsets = apriori::apriori(&transactions, 0.5);
    println!("Frequent itemsets: {:?}", frequent_itemsets);
    println!("Transactions: {:?}", transactions);
}
