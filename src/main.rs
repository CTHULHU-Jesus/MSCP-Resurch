use std::hash::Hash;
use std::collections::HashSet;

/// returns the set that minimally covers the `sets`.
fn backtrack<T: Eq+Hash+Clone>(sets: &[HashSet<T>]) -> HashSet<T>
{
    /// Helper function for backtrack
    /// # Arguments
    /// * min_found - the minimum length of an answer found.
    /// * found - the partial answer.
    /// * uncovered_sets - the sets left to cover.
    /// Returns the minimum set that covers the original sets.
    fn helper<T: Eq+Hash+Clone>
        ( min_found : usize
        , found: &HashSet<T>
        , uncovered_sets: &Vec<HashSet<T>>) -> Option<HashSet<T>> {
            // Skip Work if we have found a better solution all ready
            if found.len() >= min_found {
                return None;
            }
            // Return if all sets are covered.
            if uncovered_sets.len() == 0 {
                return Some(found.clone());
            };
            // empty list
            let empty_list = vec![];
            // first set
            let head : &HashSet<_> = uncovered_sets.get(0)?;
            // rest of sets to cover
            let tail = 
                if uncovered_sets.len() <= 1 {
                    &empty_list
                } else {
                    uncovered_sets.get(1..).unwrap()
                };
            // Best solution found
            let ref mut best_solution : Option<HashSet<_>> = None;
            for elem in head {
                // minimum set size found
                let min_found = best_solution.clone().map(|x| x.len()).unwrap_or(min_found);
                // new found to test
                let mut found_prime = found.clone();
                found_prime.insert(elem.clone());
                // filter out all of the sets that contain
                // elem, because they have been covered.
                let uncovered : Vec<HashSet<_>> = 
                    tail.clone().iter().filter(|x| ! x.contains(elem)).map(|x| x.clone()).collect();
                // if solution is better, save as new best.
                if let Some(solution) = helper(min_found,&found_prime,&uncovered) {
                    if solution.len() < min_found {
                        *best_solution = Some(solution);
                    }
                }
            }
            return best_solution.clone();
        }
    // remove empty sets (they cannot be covered)
    let sets : Vec<HashSet<_>> = 
        sets.iter()
            .filter(|x| ! x.is_empty())
            .map(|x| x.clone()).collect();
    // Use Helper function.
    return helper(usize::MAX, &HashSet::new(), &sets)
        .expect("Something has gone wrong with the simple backtracking algorithm.");
}

// Example code
fn main() {
    // declares the set {a,b}
    let mut set1 : HashSet<char> = HashSet::new(); 
    set1.insert('a');
    set1.insert('b');
    // declares the set {b,c}
    let mut set2 = HashSet::new();
    set2.insert('b');
    set2.insert('c');

    // Prints the set to minimally cover set 1 and 2
    let sets = [set1,set2];
    println!("Sets:   {:?}\nAnswer: {:?}",sets.clone(),backtrack(&sets.clone()));
}

#[cfg(test)]
mod tests {
    use super::backtrack;
    use std::collections::HashSet;
    #[test]
    fn one_set() {
        let mut set1 : HashSet<char> = HashSet::new(); 
        set1.insert('a');
        set1.insert('b');
        let solution = backtrack(&[set1]);
        assert_eq!(1,solution.len());
    }

    #[test]
    fn two_sets() {
        let mut set1 : HashSet<char> = HashSet::new();
        set1.insert('a');
        set1.insert('b');
        let mut set2 = HashSet::new();
        set2.insert('b');
        set2.insert('c');
        let sets = vec![set1,set2];
        let solution = backtrack(&sets);
        let mut answer = HashSet::new();
        answer.insert('b');
        assert_eq!(answer,solution);
    }
}

