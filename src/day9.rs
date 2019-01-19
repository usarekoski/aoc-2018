use linked_list::LinkedList;

// Solution 1: 367634
// Solution 2: 3020072891
pub fn solve1(n_players: usize, last_marble: u64) -> u64 {
    // player number (indices) to score.
    let mut scores: Vec<u64> = vec![];
    scores.resize(n_players, 0);

    let mut marbles: LinkedList<u64> = LinkedList::new();
    marbles.push_back(0);
    let mut next_marble = 1;
    let mut cur_player = 0;
    let mut cur_marble = marbles.cursor();

    while next_marble <= last_marble {
        if next_marble % 23 == 0 {
            scores[cur_player] += next_marble;
            for _ in 0..7 {
                if let None = cur_marble.prev() {
                    cur_marble.prev();
                }
            }
            scores[cur_player] += cur_marble.remove().unwrap();
        } else {
            for _ in 0..2 {
                if let None = cur_marble.next() {
                    cur_marble.next();
                }
            }
            cur_marble.insert(next_marble);
        }

        // Advance turn.
        next_marble += 1;
        if cur_player == n_players - 1 {
            cur_player = 0;
        } else {
            cur_player += 1;
        }
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve1(9, 25), 32);
        assert_eq!(solve1(10, 1618), 8317);
        assert_eq!(solve1(13, 7999), 146373);
        assert_eq!(solve1(17, 1104), 2764);
        assert_eq!(solve1(21, 6111), 54718);
        assert_eq!(solve1(30, 5807), 37305);
    }
}
