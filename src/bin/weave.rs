use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};

fn get_lang_from_ext(ext: &str) -> &str {
    match ext {
        "v" | "vh" | "sv" => "verilog",
        "rs" => "rust",
        "c" | "h" => "c",
        "cpp" | "hpp" => "cpp",
        "py" => "python",
        "js" => "javascript",
        "sh" => "bash",
        "typ" => "typst",
        _ => "",
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let lines: Vec<&str> = input.lines().collect();

    // Pass 1: Build the Language Map
    let chunk_langs = build_language_map(&lines);

    // Pass 2: Output processed content
    let mut in_block = false;
    let mut current_lang = "";

    for line in lines {
        let trimmed = line.trim();

        // Check for chunk start: <<Name>>=
        if !in_block && trimmed.starts_with("<<") && trimmed.ends_with(">>=") {
            in_block = true;
            let name = &trimmed[2..trimmed.len() - 3];
            current_lang = chunk_langs.get(name).map(|s| s.as_str()).unwrap_or("");

            let label = chunk_name_to_label(name);
            println!("_\u{27E8}{}\u{27E9}_\u{2261} <{}> \\", name, label);
            continue;
        }

        // Check for chunk end: @
        if in_block && trimmed == "@" {
            in_block = false;
            continue;
        }

        if in_block {
            process_code_line(line, current_lang);
        } else {
            // Regular text outside of blocks
            println!("{}", line);
        }
    }
}

fn process_code_line(line: &str, lang: &str) {
    let trimmed = line.trim();

    // Match <<Reference>> pattern (no '=' at the end)
    if trimmed.starts_with("<<") && trimmed.ends_with(">>") {
        let ref_text = &trimmed[2..trimmed.len() - 2];
        let leading_ws = &line[..line.find("<<").unwrap()];

        let mut output = String::new();
        if !leading_ws.is_empty() {
            let escaped_ws = leading_ws.replace('\\', "\\").replace('"', "\\\"");
            output.push_str(&format!("#raw(lang: \"{}\", \"{}\")", lang, escaped_ws));
        }

        let ref_label = chunk_name_to_label(ref_text);
        output.push_str(&format!(
            "#link(<{0}>)[_\u{27E8}{1} _*#link(<{0}>)[#context counter(page).at(<{0}>).first()]*\u{27E9}]\\",
            ref_label, ref_text
        ));
        println!("{}", output);
    } else {
        // Regular code line
        let escaped_line = line.replace('\\', "\\").replace('"', "\\\"");
        println!("#raw(lang: \"{}\", \"{}\") \\", lang, escaped_line);
    }
}

fn build_language_map(lines: &[&str]) -> HashMap<String, String> {
    let mut chunk_to_lang = HashMap::new();
    let mut relations: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_chunk = String::new();
    let mut in_block = false;

    // First, find roots and capture references
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("<<") && trimmed.ends_with(">>=") {
            in_block = true;
            current_chunk = trimmed[2..trimmed.len() - 3].to_string();

            // If name has a dot, it's a root. Assign language immediately.
            if let Some(pos) = current_chunk.rfind('.') {
                let ext = &current_chunk[pos + 1..];
                chunk_to_lang.insert(current_chunk.clone(), get_lang_from_ext(ext).to_string());
            }
        } else if in_block && trimmed == "@" {
            in_block = false;
        } else if in_block && trimmed.starts_with("<<") && trimmed.ends_with(">>") {
            let ref_name = trimmed[2..trimmed.len() - 2].to_string();
            relations
                .entry(current_chunk.clone())
                .or_default()
                .push(ref_name);
        }
    }

    // Propagate language from roots to children
    let mut lang_map: HashMap<String, String> = HashMap::new();
    for (chunk, lang) in chunk_to_lang {
        propagate_lang(&chunk, &lang, &relations, &mut lang_map);
    }

    lang_map
}

fn propagate_lang(
    parent: &str,
    lang: &str,
    relations: &HashMap<String, Vec<String>>,
    map: &mut HashMap<String, String>,
) {
    map.insert(parent.to_string(), lang.to_string());

    if let Some(children) = relations.get(parent) {
        for child in children {
            // Only recurse if the child doesn't already have this language
            if map.get(child).map(|s| s.as_str()) != Some(lang) {
                propagate_lang(child, lang, relations, map);
            }
        }
    }
}

fn chunk_name_to_label(name: &str) -> String {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    format!("chunk-{:x}", hasher.finish())
}
