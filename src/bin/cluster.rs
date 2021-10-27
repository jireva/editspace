use std::env;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn main() -> io::Result<()> {
	let (distance, min_count, max_error_fraction) = parse_args(env::args());
	let trie = trie_from_stdin();

	let mut stdout = io::stdout();
	let mut stderr = io::stderr();

	for ix in trie.iter_words() {
		let word = trie.word(ix);
		let matches: Vec<editspace::Match> =
			trie.iter_matches(&word, distance).collect();
		if matches.len() == 1 {
			if trie.item(ix).unwrap() > min_count {
				stdout.write_all(&word)?;
				stdout.write_all(b"\n")?;
			} else {
				write!(stderr, "low_count	{}	", trie.item(ix).unwrap())?;
				stderr.write_all(&word)?;
				stderr.write_all(b"\n")?;
			}
			continue;
		}
		let mut best_count = trie.item(ix).unwrap();
		let mut best_match = ix;
		for m in matches.iter() {
			if trie.item(m.index).unwrap() > best_count {
				best_count = trie.item(m.index).unwrap();
				best_match = m.index;
			}
		}
		if best_count < min_count {
			write!(stderr, "low_count	{}	", trie.item(ix).unwrap())?;
			stderr.write_all(&word)?;
			stderr.write_all(b"\n")?;
			continue;
		}
		let mut unique_cluster = true;
		for m in matches.iter() {
			if m.index == best_match {
				continue;
			}
			if (trie.item(m.index).unwrap() as f32) / (best_count as f32)
				> max_error_fraction
			{
				unique_cluster = false;
				break;
			}
		}
		if unique_cluster {
			if ix == best_match {
				stdout.write_all(&trie.word(best_match))?;
				stdout.write_all(b"\n")?;
			}
		} else {
			write!(stderr, "multiple_peaks	{}	", trie.item(ix).unwrap())?;
			stderr.write_all(&word)?;
			stderr.write_all(b"\n")?;
		}
	}
	Ok(())
}

fn parse_args(args: env::Args) -> (u8, u32, f32) {
	let args: Vec<String> = args.collect();
	if args.len() != 4 {
		eprintln!("usage: cluster distance min_counts max_error_fraction");
		process::exit(1);
	}
	let distance = match args[1].parse() {
		Ok(distance) => distance,
		Err(_) => {
			eprintln!("{} is not a number", args[1]);
			process::exit(1);
		}
	};
	if distance > 4 {
		eprintln!("distance should not be greater than 4");
		process::exit(1);
	}
	let min_counts = match args[2].parse() {
		Ok(min_counts) => min_counts,
		Err(_) => {
			eprintln!("{} is not a number", args[2]);
			process::exit(1);
		}
	};
	let max_error_fraction = match args[3].parse() {
		Ok(max_error_fraction) => max_error_fraction,
		Err(_) => {
			eprintln!("{} is not a number", args[2]);
			process::exit(1);
		}
	};
	(distance, min_counts, max_error_fraction)
}

fn trie_from_stdin() -> editspace::Trie<u32> {
	let mut trie = editspace::Trie::new();
	let mut buf = Vec::with_capacity(100);
	let mut reader = BufReader::new(io::stdin());
	while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
		if bytes_read == 0 {
			break;
		}
		if buf.len() == 1 {
			continue;
		}
		let item = trie.add(&buf[.. buf.len() - 1]);
		match item {
			None => *item = Some(1),
			Some(i) => *i += 1,
		};
		buf.clear();
	}
	trie
}
