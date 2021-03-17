use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;
use std::str::SplitWhitespace;

use once_cell::sync::Lazy;
use snafu::ensure;

#[derive(Debug, snafu::Snafu)]
enum Error {
    #[snafu(display("No path from '{}' to '{}' exists.", start_word, end_word))]
    NoPathExists {
        start_word: String,
        end_word: String,
    },

    #[snafu(display("The word '{}' is not in the dictionary.", word))]
    NotInDictionary { word: String },

    #[snafu(display(
        "The start word '{}' is not the same length as the end word '{}'.",
        start_word,
        end_word
    ))]
    WordsUnequalLen {
        start_word: String,
        end_word: String,
    },
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    parent: Option<Rc<Self>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<Self> {
        Rc::new(Self { data, parent: None })
    }

    fn with_parent(data: T, parent: Rc<Self>) -> Rc<Self> {
        let parent = Some(parent);
        Rc::new(Self { data, parent })
    }
}

fn main() {
    let matches = clap::App::new("shortest-path")
        .about("Find the shortest path, changing one character at a time, between two words of equal length.")
        .arg(clap::Arg::with_name("START_WORD").help("The starting word.").required(true))
        .arg(clap::Arg::with_name("END_WORD").help("The ending word.").required(true))
        .get_matches();

    // Clap will ensure START_WORD and END_WORD are provided and non-empty, so we will just unwrap here.
    let start_word = matches.value_of("START_WORD").unwrap();
    let end_word = matches.value_of("END_WORD").unwrap();

    match shortest_path(start_word, end_word) {
        Ok(path) => println!("The shortest path is {:?}", path),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}

fn hamming_distance<S>(word1: S, word2: S) -> usize
where
    S: AsRef<str>,
{
    let w1 = word1.as_ref();
    let w2 = word2.as_ref();

    w1.chars().zip(w2.chars()).fold(
        0,
        |distance, (c1, c2)| if c1 != c2 { distance + 1 } else { distance },
    )
}

fn load_dictionary(word_len: usize) -> BTreeSet<&'static str> {
    static DICTIONARY: Lazy<SplitWhitespace<'static>> =
        Lazy::new(|| include_str!("english3.txt").split_whitespace());

    DICTIONARY
        .clone()
        .filter(|line| line.len() == word_len)
        .collect()
}

fn shortest_path(start_word: &str, end_word: &str) -> Result<Vec<String>, Error> {
    // Ensure the two words are of equal length.
    ensure!(
        start_word.len() == end_word.len(),
        WordsUnequalLen {
            start_word,
            end_word
        }
    );

    let mut dictionary = load_dictionary(start_word.len());

    // Ensure the two words are in the dictionary.
    ensure!(
        dictionary.contains(start_word),
        NotInDictionary { word: start_word }
    );
    ensure!(
        dictionary.contains(end_word),
        NotInDictionary { word: end_word }
    );

    println!(
        "Finding the shortest path from '{}' to '{}'...",
        start_word, end_word
    );

    if start_word == end_word {
        return Ok(vec![start_word.to_string()]);
    }

    // To find the shortest path, we can use the Hamming distance as a heuristic for performing a tree traversal,
    // where each level is the Hamming distance from the root node. Once we find that our end word is only 1
    // distance away from another word, we can stop our search and return the path from the start word to the end word.
    // Our traversal will be a breadth-first type search.
    let mut search = VecDeque::new();
    search.push_back(Node::new(start_word));

    while let Some(current) = search.pop_front() {
        dictionary.remove(current.data);

        // Find all items in the dictionary that have a Hamming distance of 1, and queue them up for visitation.
        let one_away = dictionary
            .iter()
            .filter(|word| hamming_distance(current.data, word) == 1);
        for &word in one_away {
            let child = Node::with_parent(word, current.clone());

            if word == end_word {
                // Once we see our end word, we can stop. Because we're doing a level-order traversal of paths
                // stemming from the start word, we're definitely at what would be a minimum path. There could
                // be many other paths beyond this one that are of the same length, but we don't care.
                // Walk back up the ancestry tree to assemble the full path to this node and return.
                let mut path = VecDeque::new();
                path.push_front(child.data.to_string());

                let mut parent = child.parent.as_ref();
                while let Some(p) = parent {
                    path.push_front(p.data.to_string());
                    parent = p.parent.as_ref();
                }

                return Ok(path.into_iter().collect());
            }

            search.push_back(child);
        }
    }

    Err(Error::NoPathExists {
        start_word: start_word.to_string(),
        end_word: end_word.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat_dog() {
        assert_eq!(
            shortest_path("cat", "dog").unwrap(),
            &["cat", "cot", "cog", "dog"]
        );
    }

    #[test]
    fn van_car() {
        assert_eq!(shortest_path("van", "car").unwrap(), &["van", "can", "car"]);
    }

    #[test]
    fn name_norm() {
        assert_eq!(
            shortest_path("name", "norm").unwrap(),
            &["name", "nare", "nard", "nord", "norm"]
        );
    }

    #[test]
    fn bone_type() {
        assert_eq!(
            shortest_path("bone", "type").unwrap(),
            &["bone", "tone", "tope", "type"]
        );
    }

    #[test]
    fn equal_start_end() {
        assert_eq!(shortest_path("cat", "cat").unwrap(), &["cat"]);
    }

    #[test]
    fn not_in_dictionary() {
        assert!(matches!(
            shortest_path("cat", "bwq"),
            Err(Error::NotInDictionary { .. })
        ));
    }

    #[test]
    fn unequal_length() {
        assert!(matches!(
            shortest_path("cat", "fish"),
            Err(Error::WordsUnequalLen { .. })
        ));
    }

    #[test]
    fn no_path_exists() {
        assert!(matches!(
            shortest_path("distance", "keyboard"),
            Err(Error::NoPathExists { .. })
        ));
    }
}
