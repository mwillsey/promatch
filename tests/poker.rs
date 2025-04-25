use promatch::*;

use std::collections::BTreeSet as Set;

// inspired by https://github.com/egison/sweet-egison/blob/master/sample/poker.hs

struct Card(u8, char);

// fn poker(hand: Set<Card>) {
//     let results = promatch!(() match hand {
//         [Card(r, _), Card(r, _)] => { vec!["pair"] }
//     });
// }
