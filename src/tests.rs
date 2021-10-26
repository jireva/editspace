
use super::*;

#[test]
fn distance() {
	let mut trie = Trie::new();

	*trie.add(b"TAATACGACTCACTATAGGG") = Some("item");

	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGG", 0).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGA", 0).count(), 0);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGG", 1).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGA", 1).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGAA", 1).count(), 0);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGG", 2).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGA", 2).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGAA", 2).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAAAA", 2).count(), 0);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGG", 3).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGGA", 3).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAGAA", 3).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTATAAAA", 3).count(), 1);
	assert_eq!(trie.iter_matches(b"TAATACGACTCACTAAAAAA", 3).count(), 0);
}

#[test]
fn empty_word() {
	let mut trie = Trie::new();

	*trie.add(b"TAATACGACTCACTATAGGG") = Some("not empty");
	*trie.add(b"") = Some("empty");

	assert_eq!(trie.item(trie.iter_matches(b"", 0).next().unwrap().index), &Some("empty"));
	assert_eq!(trie.iter_matches(b"", 0).count(), 1);
	assert_eq!(trie.iter_matches(b"A", 0).count(), 0);
	assert_eq!(trie.iter_matches(b"A", 1).count(), 1);
	assert_eq!(trie.iter_matches(b"AA", 0).count(), 0);
	assert_eq!(trie.iter_matches(b"AA", 1).count(), 0);
	assert_eq!(trie.iter_matches(b"AA", 2).count(), 1);
	assert_eq!(trie.iter_matches(b"AAA", 0).count(), 0);
	assert_eq!(trie.iter_matches(b"AAA", 1).count(), 0);
	assert_eq!(trie.iter_matches(b"AAA", 2).count(), 0);
	assert_eq!(trie.iter_matches(b"AAA", 3).count(), 1);
}
