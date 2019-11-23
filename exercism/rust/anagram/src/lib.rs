use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
  possible_anagrams
    .iter()
    .map(AsRef::as_ref)
    .filter(|cand| is_anagram(word, *cand))
    .collect()
}

fn is_anagram(source: &str, test: &str) -> bool {
  let mut up_test = test.to_uppercase().chars().collect::<Vec<char>>();
  let mut up_source = source.to_uppercase().chars().collect::<Vec<char>>();

  if up_test == up_source {
    return false;
  }

  up_test.sort();
  up_source.sort();

  up_test == up_source
}
