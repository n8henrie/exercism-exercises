// - If the number has 3 as a factor, output 'Pling'.
// - If the number has 5 as a factor, output 'Plang'.
// - If the number has 7 as a factor, output 'Plong'.
// - If the number does not have 3, 5, or 7 as a factor,
//   just pass the number's digits straight through.

use std::collections::BTreeMap;

pub fn raindrops(n: usize) -> String {
    let response_map: BTreeMap<_, _> = 
        [
         (3, "Pling"),
         (5, "Plang"),
         (7, "Plong"),
        ].iter().cloned().collect();

    let mut response = String::new();
    for (k, v) in response_map.iter() {
        if n % k == 0 {
            response.push_str(v);
        }
    }
    if response == "" {
        n.to_string()
    }
    else {
        response
    }
}
