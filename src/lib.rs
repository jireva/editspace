use std::cmp;
use std::ops;

#[cfg(test)]
mod tests;

const MAXLEN: usize = 100;

pub struct Trie<T> (Vec<Node<T>>);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Index(usize);

struct Node<T> {
	base: u8,
	parent: Option<Index>,
	a: Option<Index>,
	c: Option<Index>,
	g: Option<Index>,
	t: Option<Index>,
	item: Option<T>,
}
impl<T> Node<T> {
	fn new(base: u8, parent: Index) -> Node<T> {
		Node{base, parent: Some(parent), a: None,  c: None,  g: None,  t: None, item: None}
	}
}

const BASES: [u8; 4] = [65, 67, 71, 84];

impl<T> ops::Index<u8> for Node<T> {
	type Output = Option<Index>;

	fn index(&self, index: u8) -> &Self::Output {
		match index {
			65 => &self.a,
			67 => &self.c,
			71 => &self.g,
			84 => &self.t,
			_ => panic!("invalid base"),
		}
	}
}

impl<T> ops::IndexMut<u8> for Node<T> {
	fn index_mut(&mut self, index: u8) -> &mut Self::Output {
		match index {
			65 => &mut self.a,
			67 => &mut self.c,
			71 => &mut self.g,
			84 => &mut self.t,
			_ => panic!("invalid base"),
		}
	}
}

impl<T> Trie<T> {
	pub fn new() -> Trie<T> {
		Trie(vec![Node{base: 0, parent: None, a: None,  c: None,  g: None,  t: None, item: None}])
	}

	pub fn item(&self, i: Index) -> &Option<T> {
		&self.0[i.0].item
	}

	pub fn word(&self, i: Index) -> Vec<u8> {
		let mut word = Vec::with_capacity(MAXLEN);
		let mut i = i;
		while let Some(parent) = self.0[i.0].parent {
			word.push(self.0[i.0].base);
			i = parent;
		}
		word.reverse();
		word
	}

	pub fn add(&mut self, word: &[u8]) -> &mut Option<T> {
		let mut node = 0;
	
		for c in word.iter() {
			match self.0[node][*c] {
				Some(child) => node = child.0,
				None => {
					self.0[node][*c] = Some(Index(self.0.len()));
					self.0.push(Node::new(*c, Index(node)));
					node = self.0.len() - 1;
				},
			}
		}
		&mut self.0[node].item
	}

	pub fn iter_words<'a>(&'a self) -> Indices<'a, T> {
		Indices{
			trie: self,
			branches: vec![Index(0)],
		}
	}

	pub fn iter_matches<'a>(&'a self, word: &'a [u8], distance: u8) -> Matches<'a, T> {
		assert!(word.len() < MAXLEN);
		assert!(distance < 5);
		let mut row = [0u8; MAXLEN];
		for i in 0..(word.len()+1) {
			row[i] = i as u8;
		}
		let mut branches = Vec::with_capacity(16);
		for b in BASES.iter() {
			if let Some(node) = self.0[0][*b] {
				branches.push(ProtoMatch{index: node, row})
			}
		}
		Matches{
			trie: self,
			word,
			distance,
			branches,
		}
	}
}

pub struct Indices<'a, T> {
	trie: &'a Trie<T>,
	branches: Vec<Index>,
}

impl<'a, T> Iterator for Indices<'a, T> {
	type Item = Index;

	fn next(&mut self) -> Option<Index> {
		while let Some(ix) = self.branches.pop() {
			for b in BASES.iter() {
				if let Some(node) = self.trie.0[ix.0][*b] {
					self.branches.push(node)
				}
			}
			if self.trie.0[ix.0].item.is_some() {
				return Some(ix);
			}
		}
		None
	}
}

pub struct Match {
	pub index: Index,
	pub distance: u8,
}

struct ProtoMatch {
	index: Index,
	row: [u8; MAXLEN],
}

pub struct Matches<'a, T> {
	trie: &'a Trie<T>,
	word: &'a [u8],
	distance: u8,
	branches: Vec<ProtoMatch>,
}

impl<'a, T> Iterator for Matches<'a, T> {
	type Item = Match;

	fn next(&mut self) -> Option<Match> {
		while let Some(proto_match) = self.branches.pop() {
			let mut row = [0u8; MAXLEN];
			row[0] = proto_match.row[0] + 1;
			let mut insert_cost;
			let mut delete_cost;
			let mut replace_cost;
			for column in 1..(self.word.len()+1) {
				insert_cost = row[column-1] + 1;
				delete_cost = proto_match.row[column] + 1;
				if self.word[column-1] != self.trie.0[proto_match.index.0].base {
					replace_cost = proto_match.row[column-1] + 1;
				} else {
					replace_cost = proto_match.row[column-1];
				}
				row[column] = cmp::min(cmp::min(insert_cost, delete_cost), replace_cost);
			}
			// if any entries in the row are less than the maximum cost, then
			// recursively search each branch of the trie
			if row[..(self.word.len()+1)].iter().min().unwrap() <= &self.distance {
				for b in BASES.iter() {
					if let Some(node) = self.trie.0[proto_match.index.0][*b] {
						self.branches.push(ProtoMatch{index: node, row})
					}
				}
			}
			// if the last entry in the row indicates the optimal cost is less than the
			// maximum cost, and there is a word in this trie node, then return it.
			if (row[self.word.len()] <= self.distance) && (self.trie.0[proto_match.index.0].item.is_some()) {
				return Some(Match{
					index: proto_match.index,
					distance: row[self.word.len()],
				});
			}
		}
		None
	}
}
