use std::collections::{HashSet, VecDeque};

/// https://adventofcode.com/2020/day/22
/// Card game with a crab

/// both players draw their top card, and the player with the higher-valued card wins the round.
/// The winner keeps both cards, placing them on the bottom of their own deck so that
/// the winner's card is above the other card.

const _TEST_INPUT: &str = "
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"; // --> answer
   //    3 * 10
   // +  2 *  9
   // + 10 *  8
   // +  6 *  7
   // +  8 *  6
   // +  5 *  5
   // +  9 *  4
   // +  4 *  3
   // +  7 *  2
   // +  1 *  1
   // = 306

type Deck = VecDeque<usize>;

/// play game rounds until one player has all the cards and return the player index, and that deck
fn play_combat(mut players: Vec<Deck>) -> (usize, Deck) {
    while !players.iter().any(|deck| deck.is_empty()) {
        let trick: Vec<_> = players
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        let (winner_index, &max_num) = trick
            .iter()
            .enumerate()
            .max_by_key(|(_, &val)| val)
            .unwrap();
        players[winner_index].push_back(max_num);
        players[winner_index].push_back(trick[1 - winner_index]);
    }
    players
        .into_iter()
        .enumerate()
        .find(|(_, deck)| !deck.is_empty())
        .unwrap()
}

/// play recursive game rounds until one player has all the cards and return the player index, and that deck
/// NOTE: this is still pretty slow. Probably because I wrote it to possible allow more than two players
/// (could try representing deck1 and deck2 directly rather than as a Vec<Deck>)
fn play_recursive_combat(mut players: Vec<Deck>) -> (usize, Deck) {
    let mut game_history = HashSet::new();
    loop {
        if !game_history.insert(players.clone()) {
            // repeat configuration: player 1 wins:
            return (0, players.remove(0));
        }
        let trick: Vec<_> = players
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        let winner_index;
        // recursive game?
        if players
            .iter()
            .zip(trick.iter())
            .all(|(deck, &num)| deck.len() >= num)
        {
            // TODO
            let recurse_players = players
                .iter()
                .zip(trick.iter())
                .map(|(deck, &num)| deck.iter().take(num).copied().collect())
                .collect();
            winner_index = play_recursive_combat(recurse_players).0;
        } else {
            // normal round: higher card wins:
            winner_index = trick
                .iter()
                .enumerate()
                .max_by_key(|(_, &val)| val)
                .unwrap()
                .0;
        }
        // winner takes the cards: (NOTE: for simplicity this relies on it being a two-player game!)
        players[winner_index].push_back(trick[winner_index]);
        players[winner_index].push_back(trick[1 - winner_index]);
        // check whether one players lost all their cards, then the other wins:
        if players.iter().any(|deck| deck.is_empty()) {
            return players
                .into_iter()
                .enumerate()
                .find(|(_, deck)| !deck.is_empty())
                .unwrap();
        }
    }
}

/// calculate the score: bottom card * 1, next * 2, and so on
fn calc_score(deck: Deck) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .fold(0, |acc, (&a, b)| acc + a * b)
}

fn cards_splitter(lines: &str) -> Deck {
    lines
        .split('\n')
        .skip(1)
        .map(|a| a.parse::<usize>().unwrap())
        .collect()
}

pub fn run() {
    let input = include_str!("input").trim().split("\n\n");
    let players: Vec<Deck> = input.map(cards_splitter).collect();
    println!("Input: {:?}", &players);
    let (winner, winning_deck) = play_combat(players.clone());
    println!(
        "Part one Winner {} with deck: {:?}",
        winner + 1,
        &winning_deck
    );
    println!("Winner scores: {:?}", calc_score(winning_deck));
    let (recursive_winner, winning_deck) = play_recursive_combat(players);
    println!(
        "Part two Winner {} with deck: {:?}",
        recursive_winner + 1,
        &winning_deck
    );
    println!("Winner scores: {:?}", calc_score(winning_deck));
}
