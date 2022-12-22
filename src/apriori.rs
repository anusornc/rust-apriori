use std::collections::HashSet;

pub fn apriori(transactions: &Vec<Vec<String>>, min_support: f64) -> Vec<Vec<String>> {
    let mut frequent_itemsets = Vec::new();

    // First, generate the set of frequent items by counting the occurrences of each item in the transactions
    let mut item_counts = std::collections::HashMap::new();
    for transaction in transactions {
        for item in transaction {
            *item_counts.entry(item).or_insert(0) += 1;
        }
    }

    // Filter the item counts to get the set of frequent items
    let num_transactions = transactions.len() as f64;
    let frequent_items: HashSet<String> = item_counts
        .into_iter()
        .filter(|(_, count)| (*count as f64) / num_transactions >= min_support)
        .map(|(item, _)| item)
        .collect();

    // Add the frequent items to the list of frequent item sets
    for item in frequent_items {
        frequent_itemsets.push(vec![item]);
    }

    // Generate frequent item sets of increasing size until no more can be found
    let mut _k = 2;
    while !frequent_itemsets.is_empty() {
        // Generate candidate item sets by joining the frequent item sets of size k-1
        let mut candidate_itemsets = Vec::new();
        for i in 0..frequent_itemsets.len() {
            for j in i+1..frequent_itemsets.len() {
                let mut candidate = frequent_itemsets[i].clone();
                candidate.extend(frequent_itemsets[j].clone());
                candidate.sort();
                candidate_itemsets.push(candidate);
            }
        }

        // Count the occurrences of the candidate item sets in the transactions
        let mut itemset_counts = std::collections::HashMap::new();
        for transaction in transactions {
            for itemset in candidate_itemsets.iter() {
                if transaction.iter().all(|item| itemset.contains(item)) {
                    *itemset_counts.entry(itemset).or_insert(0) += 1;
                }
            }
        }

        // Filter the candidate item sets to get the set of frequent item sets
        frequent_itemsets = itemset_counts
            .into_iter()
            .filter(|(_, count)| (*count as f64) / num_transactions >= min_support)
            .map(|(itemset, _)| itemset.clone())
            .collect();

        _k += 1;
    }

    frequent_itemsets
}
