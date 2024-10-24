use sha2::{Digest, Sha256};

use crate::primitives::Transaction;

pub fn hash(raw_string: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_string.as_bytes());
    let result = hasher.finalize();
    let prefix = "0x".to_string();
    prefix + &hex::encode(result)

}


pub fn generate_merkle_tree(transactions: &Vec<&Transaction>) -> Vec<String> {
    // we need to initialise an array of the size 2
    let n = transactions.len() as u64;

    let height = 64 - n.leading_zeros();

    let null_string = format!("0x{:0>64}", ""); // Fill with 64 zeros

    let size = (1 << (height + 1)) - 1;

    let mut tree: Vec<String> = vec![null_string; size];

    // iteratively populate the tree by starting from the index 2^height - 2 and going back

    let mut idx = (1 << height) - 1;

    for transaction in transactions {
        tree[idx] = transaction.generate_hash();
        idx += 1;
    }

    let end_idx = 1 << height - 2;

    for i in (0..end_idx).rev() {
        let left_child = &tree[(2 * i) + 1];
        let right_child = &tree[(2 * i) + 2];

        if left_child < right_child {
            tree[i] = hash(format!("{}{}", left_child, right_child));
        } else {
            tree[i] = hash(format!("{}{}", right_child, left_child));
        }
    }

    tree
}
