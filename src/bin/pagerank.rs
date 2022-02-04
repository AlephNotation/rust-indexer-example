/* use rusty_index::Pagerank;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut pgr = Pagerank::<&str>::new();

    let stdin = io::stdin();

    let now = Instant::now();

    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let words: Vec<String> = line
            .trim()
            .split("\t")
            .map(|s| s.parse().unwrap())
            .collect();

        if words.len() != 4 {
            continue;
        }

        if i > 0 {
            pgr.add_edge(words[1].clone(), words[3].clone());
        }
    }
    println!("Graph size is {} nodes with {} edges between them", pgr.len(), pgr.len_node());

    Ok(())
}

*/
use rusty_index::Pagerank;
fn main(){}