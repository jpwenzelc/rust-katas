pub fn calculate_score(player_1_score: u8, player_2_score: u8) -> String {
    if have_same_score(player_1_score, player_2_score) {
        return same_score_output(player_1_score);
    }

    match (player_1_score > 3, player_2_score > 3) {
        (true, false) => {
            if player_2_score == 3 {
                return advantage_output(player_1_score, player_2_score);
            }
            return String::from("player 1 wins!");
        }
        (false, true) => {
            if player_1_score == 3 {
                return advantage_output(player_1_score, player_2_score);
            }
            return String::from("player 2 wins!");
        }
        (true, true) => match player_1_score.abs_diff(player_2_score) {
            1 => return advantage_output(player_1_score, player_2_score),
            _ => return winner_output(player_1_score, player_2_score),
        },
        (false, false) => {
            let player_1_score = named_score(player_1_score);
            let player_2_score = named_score(player_2_score);

            return score_output(player_1_score, player_2_score);
        }
    }
}

fn winner_output(score_1: u8, score_2: u8) -> String {
    if score_1 > score_2 {
        return String::from("player 1 wins!");
    }
    return String::from("player 2 wins!");
}

fn advantage_output(score_1: u8, score_2: u8) -> String {
    if score_1 > score_2 {
        return String::from("advantage player 1");
    }
    return String::from("advantage player 2");
}

fn same_score_output(score: u8) -> String {
    if score > 2 {
        return String::from("deuce");
    }
    score_output(named_score(score), "all")
}

fn have_same_score(player_1_score: u8, player_2_score: u8) -> bool {
    player_1_score == player_2_score
}

fn score_output(score_1: &str, score_2: &str) -> String {
    String::from(format!("{}-{}", score_1, score_2))
}

fn named_score(score: u8) -> &'static str {
    match score {
        3 => return "forty",
        2 => return "thirty",
        1 => return "fifteen",
        _ => return "love",
    };
}

#[cfg(test)]
mod tests {
    use crate::tennis::scorer::calculate_score;
    use rstest::rstest;

    #[rstest]
    #[case::score_0_0("love-all", 0, 0)]
    #[case::score_0_1("love-fifteen", 0, 1)]
    #[case::score_0_2("love-thirty", 0, 2)]
    #[case::score_0_3("love-forty", 0, 3)]
    #[case::score_0_4("player 2 wins!", 0, 4)]
    #[case::score_1_0("fifteen-love", 1, 0)]
    #[case::score_2_0("thirty-love", 2, 0)]
    #[case::score_3_0("forty-love", 3, 0)]
    #[case::score_4_0("player 1 wins!", 4, 0)]
    #[case::score_1_1("fifteen-all", 1, 1)]
    #[case::score_2_2("thirty-all", 2, 2)]
    #[case::score_3_3("deuce", 3, 3)]
    #[case::score_5_5("deuce", 5, 5)]
    #[case::score_4_3("advantage player 1", 4, 3)]
    #[case::score_3_4("advantage player 2", 3, 4)]
    #[case::score_5_4("advantage player 1", 5, 4)]
    #[case::score_6_5("advantage player 1", 6, 5)]
    #[case::score_4_5("advantage player 2", 4, 5)]
    #[case::score_6_4("player 1 wins!", 6, 4)]
    #[case::score_4_6("player 2 wins!", 4, 6)]
    fn that_returns_score_of(
        #[case] expected_score: &str,
        #[case] player_1_score: u8,
        #[case] player_2_score: u8,
    ) {
        let score = calculate_score(player_1_score, player_2_score);

        assert_eq!(score, String::from(expected_score));
    }
}
