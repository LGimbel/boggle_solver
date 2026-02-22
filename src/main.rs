use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_word = true;
    }
}

struct BoggleSolver {
    trie: TrieNode,
    board: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl BoggleSolver {
    fn new(board: Vec<Vec<char>>, dict_path: &str) -> std::io::Result<Self> {
        let mut trie = TrieNode::default();
        let file = File::open(dict_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let word = line?;
            let trimmed = word.trim();
            if trimmed.len() >= 3 && trimmed.len() <= 16 {
                trie.insert(&trimmed.to_uppercase());
            }
        }

        let rows = board.len() as i32;
        let cols = board[0].len() as i32;
        Ok(BoggleSolver { trie, board, rows, cols })
    }

    fn solve(&self) -> (usize, Vec<String>) {
        let mut found_words = HashSet::new();
        let mut visited = vec![vec![false; self.cols as usize]; self.rows as usize];

        for r in 0..self.rows {
            for c in 0..self.cols {
                self.dfs(r, c, &self.trie, String::new(), &mut visited, &mut found_words);
            }
        }

        let count = found_words.len();
        let mut sorted_words: Vec<String> = found_words.into_iter().collect();
        sorted_words.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
        let longest_6 = sorted_words.into_iter().take(6).collect();

        (count, longest_6)
    }

    fn dfs(
        &self,
        r: i32,
        c: i32,
        node: &TrieNode,
        mut path: String,
        visited: &mut Vec<Vec<bool>>,
        found: &mut HashSet<String>,
    ) {
        if r < 0 || r >= self.rows || c < 0 || c >= self.cols || visited[r as usize][c as usize] {
            return;
        }

        let ch = self.board[r as usize][c as usize];
        if let Some(next_node) = node.children.get(&ch) {
            visited[r as usize][c as usize] = true;
            path.push(ch);

            if next_node.is_word {
                found.insert(path.clone());
            }

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr != 0 || dc != 0 {
                        self.dfs(r + dr, c + dc, next_node, path.clone(), visited, found);
                    }
                }
            }

            visited[r as usize][c as usize] = false;
        }
    }
}

fn main() {
    // env::args() includes the executable path at index 0
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: cargo run -- <row1> <row2> <row3> <row4>");
        eprintln!("Example: cargo run -- srps euim eahw wdzr");
        return;
    }

    let mut board = Vec::new();
    for i in 1..5 {
        let row_str = &args[i];
        if row_str.len() != 4 {
            eprintln!("Error: Each argument must be exactly 4 letters long.");
            return;
        }
        // Convert to uppercase characters for matching the Trie
        let row_chars: Vec<char> = row_str.to_uppercase().chars().collect();
        board.push(row_chars);
    }

    match BoggleSolver::new(board, "words.txt") {
        Ok(solver) => {
            let (total, longest) = solver.solve();
            println!("Total words found: {}", total);
            println!("Longest 3 words: {:?}", longest);
        }
        Err(e) => eprintln!("Error loading dictionary: {}. Ensure words.txt is in the project root.", e),
    }
}