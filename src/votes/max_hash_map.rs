
use std::hash::Hash;
use std::collections::HashMap;

fn maxHashMap<K, V>(hash_map: HashMap<K, V>) -> (K, V)
where K: Hash + Eq + Debug + Default, V: Ord + Debug + Default {
    //! DESCRIPTION:
    //!     Return the (Key, Value) pair of the entry with the largest value in the hash_map
    //! PARAMS:
    //!     hash_map: Generic HashMap
    let mut maxKey: K = K::default();
    let mut maxVal: V = V::default();
    for (key, value) in hash_map {
        if value > maxVal {
            maxKey = key;
            maxVal = value;
        }
    }
    // let totalVotes  = voteCounts.iter().map(|(k, &v)| v).fold(0, |acc, i| acc+i);
    // let countValue = votes.iter().filter(|&n| *n != &Vote::NullVote).count() as i32;
    // let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
    // Ideally avoid taking ownership, just borrow
    (maxKey, maxVal)
}
