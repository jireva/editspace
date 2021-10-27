
use std::process;
use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let (distance, known_file) = parse_args(env::args());
	let trie = trie_from_known(known_file);
	
	let mut stdout = io::stdout();
	let mut stderr = io::stderr();

	let mut buf = Vec::with_capacity(500);
	let mut reader = BufReader::new(io::stdin());
	while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
		if bytes_read == 0 {
			break
		}
		let word;
		let rest;
		if let Some(tab) = buf.iter().position(|&x| x == b'\t') {
			word = &buf[0..tab];
			rest = &buf[tab+1..];
		} else {
			word = &buf[..buf.len()-1];
			rest = &buf[buf.len()-1..];
		}

		let matches: Vec<editspace::Match> = trie.iter_matches(word, distance).collect();
		if matches.is_empty() {
			stderr.write_all(b"no_matches	")?;
			stderr.write_all(&buf)?;
		} else if matches.len() == 1 {
			stdout.write_all(trie.item(matches[0].index).as_ref().unwrap())?;
			stdout.write_all(b"\t")?;
			stdout.write_all(rest)?;
		} else {
			let mut best_distance = distance;
			let mut best_match = None;
			for m in matches.iter() {
				if m.distance < best_distance {
					best_distance = m.distance;
					best_match = Some(m.index);
				}
			}
			if let Some(i) = best_match {
				stdout.write_all(trie.item(i).as_ref().unwrap())?;
				stdout.write_all(b"\t")?;
				stdout.write_all(rest)?;
			} else {
				stderr.write_all(b"multiple_matches	")?;
				stderr.write_all(&buf)?;
			}
		}
		buf.clear();
	}
	Ok(())
}

fn parse_args(args: env::Args) -> (u8, fs::File) {
	let args: Vec<String> = args.collect();
	if args.len() != 3 {
		eprintln!("usage: correct distance known-words.tsv");
		process::exit(1);
	}
	let distance = match args[1].parse() {
		Ok(distance) => distance,
		Err(_) => {
			eprintln!("{} is not a number", args[1]);
			process::exit(1);
		},
	};
	if distance > 4 {
		eprintln!("distance should not be greater than 4");
		process::exit(1);
	}
	let file = match fs::File::open(args[2].clone()) {
		Ok(file) => file,
		Err(_) => {
			eprintln!("could not open {}", args[2]);
			process::exit(1);
		},
	};
	(distance, file)	
}

fn trie_from_known(file: fs::File) -> editspace::Trie<Vec<u8>> {
	let mut trie = editspace::Trie::new();
	let mut buf = Vec::with_capacity(100);
	let mut reader = BufReader::new(file);
	while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
		if bytes_read == 0 {
			break
		}
		if buf.len() == 1 {
			continue;
		}
		if let Some(tab) = buf.iter().position(|&x| x == b'\t') {
			let word = &buf[0..tab];
			let replacement = &buf[tab+1..buf.len()-1];
			*trie.add(word) = Some(replacement.to_vec());
		} else {
			let word = &buf[..buf.len()-1];
			*trie.add(word) = Some(word.to_vec());
		}
		buf.clear();
	}
	trie
}