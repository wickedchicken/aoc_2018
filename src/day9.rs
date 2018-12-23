#[derive(Debug)]
struct MarbleCircle {
    marbles: Vec<u32>,
    current_marble_idx: usize,
}

impl MarbleCircle {
    fn modulo_index(&self, input: usize) -> usize {
        input % self.marbles.len()
    }

    fn insert(&mut self, marble: u32) -> u32 {
        if self.marbles.is_empty() {
            self.marbles.push(marble);
            self.current_marble_idx = 0;
            0
        } else if marble % 23 == 0 {
            // Avoid going negative here by adding self.marbles.len())
            // Risk positive overflow but that should be rare
            let remove_idx = self.modulo_index((self.current_marble_idx + self.marbles.len()) - 7);
            let score = marble + self.marbles[remove_idx];
            self.marbles.remove(remove_idx);
            // Old index might be out of bounds so we reset it here.
            self.current_marble_idx = self.modulo_index(remove_idx);
            score
        } else {
            let new_idx = self.modulo_index(self.current_marble_idx + 2);
            self.marbles.insert(new_idx, marble);
            self.current_marble_idx = new_idx;
            0
        }
    }
}

pub fn run(players: usize, last_marble_worth: u32) -> u32 {
    let mut marble_circle = MarbleCircle {
        marbles: Vec::new(),
        current_marble_idx: 0,
    };
    let mut player_scores = Vec::with_capacity(players);
    for _ in 0..players {
        player_scores.push(0);
    }
    let mut current_player = 0;

    for current_marble in 0..=last_marble_worth {
        let mut player_score = player_scores[current_player];
        player_score += marble_circle.insert(current_marble);
        player_scores[current_player] = player_score;

        current_player = (current_player + 1) % players;
    }
    *player_scores.iter().max().unwrap()
}
