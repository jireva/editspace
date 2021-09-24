
use super::*;

#[test]
fn iter_matches() {
	let mut trie = Trie::new();

	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTC");
	trie.add(b"ACTATG");
	trie.add(b"AGGGTTTG");
	trie.add(b"AGGGTTTTTTG");
	trie.add(b"AGGGTTTTTTG");
	trie.add(b"TT");

	assert_eq!(trie.iter_matches(b"AGCGTTTATTG", 2).next().unwrap().word(), b"AGGGTTTTTTG");
	assert_eq!(trie.iter_matches(b"AGCGTTTATTG", 2).next().unwrap().count, 2);
	assert_eq!(trie.iter_matches(b"ACTTTG", 0).next().unwrap().word(), b"ACTTTG");
	assert_eq!(trie.iter_matches(b"ACTTTG", 0).next().unwrap().count, 8);

}

#[test]
fn iter_words() {
	let mut trie = Trie::new();

	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTG");
	trie.add(b"ACTTTC");
	trie.add(b"ACTATG");
	trie.add(b"AGGGTTTG");
	trie.add(b"AGGGTTTTTTG");
	trie.add(b"AGGGTTTTTTG");
	trie.add(b"TT");

	assert_eq!(trie.iter_words().map(|w| w.count).collect::<Vec<u32>>(), vec![1, 2, 1, 8, 1, 1]);

}