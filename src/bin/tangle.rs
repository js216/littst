use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead, Write};

enum Fragment {
    Literal(String),
    Reference(String, String),
}

#[derive(Default)]
struct Chunk {
    fragments: Vec<Fragment>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: tangle <root-chunk-name> [...]");
        std::process::exit(1);
    }

    let chunks = parse_input(io::stdin().lock())?;
    let mut visited = HashSet::new();

    // iterate over all root chunk names provided on the command line
    for root_name in &args[1..] {
        if let Some(root_chunk) = chunks.get(root_name) {
            // open output file named after the chunk
            let mut file = std::fs::File::create(root_name)?;
            expand(root_name, root_chunk, &chunks, "", &mut visited, &mut file)?;
        } else {
            eprintln!("\x1b[31mError:\x1b[0m Root chunk <<{}>> not found.", root_name);
        }
    }

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> io::Result<HashMap<String, Chunk>> {
    let mut chunks: HashMap<String, Chunk> = HashMap::new();
    let mut current_chunk_name: Option<String> = None;

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if let Some(name) = current_chunk_name.as_ref() {
            if trimmed == "@" {
                current_chunk_name = None;
            } else if trimmed.starts_with("<<") && trimmed.ends_with(">>") {
                let ref_name = &trimmed[2..trimmed.len() - 2];
                let indent_pos = line.find("<<").unwrap_or(0);
                let indent = line[..indent_pos].to_string();

                chunks.entry(name.clone()).or_default()
                    .fragments.push(Fragment::Reference(ref_name.to_string(), indent));
            } else {
                chunks.entry(name.clone()).or_default()
                    .fragments.push(Fragment::Literal(line));
            }
        } else if trimmed.starts_with("<<") && trimmed.ends_with(">>=") {
            current_chunk_name = Some(trimmed[2..trimmed.len() - 3].to_string());
        }
    }
    Ok(chunks)
}

fn expand<W: Write>(
    name: &str,
    chunk: &Chunk,
    all_chunks: &HashMap<String, Chunk>,
    base_indent: &str,
    stack: &mut HashSet<String>,
    out: &mut W,
) -> io::Result<()> {
    if stack.contains(name) {
        eprintln!("\x1b[31mError:\x1b[0m Infinite recursion detected in chunk <<{}>>", name);
        std::process::exit(1);
    }

    stack.insert(name.to_string());

    for fragment in &chunk.fragments {
        match fragment {
            Fragment::Literal(text) => {
                writeln!(out, "{}{}", base_indent, text)?;
            }
            Fragment::Reference(ref_name, local_indent) => {
                if let Some(target) = all_chunks.get(ref_name) {
                    let next_indent = format!("{}{}", base_indent, local_indent);
                    expand(ref_name, target, all_chunks, &next_indent, stack, out)?;
                }
            }
        }
    }

    stack.remove(name);
    Ok(())
}
