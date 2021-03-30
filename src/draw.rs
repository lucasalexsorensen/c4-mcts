use crate::board::Board;

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::from("");
        let is_players_turn = self.moves_count % 2 == 1;

        let position = if !is_players_turn { self.position } else { self.get_opponent_position() };
        for row_idx in 1..7 {
            result += "\t";
            let i = 6 - row_idx;

            for j in 0..7 {
                let idx = j*7 + i;
                let mask_bit = self.mask & (1 << idx) != 0;
                let pos_bit = position & (1 << idx) != 0;
                
                if !mask_bit {
                    result += ". ";
                } else {
                    if !pos_bit { result += "O "; } else { result += "P "; }
                }
            }
            result += "\n";
        }

        return write!(f, "{}\nNext turn: {}", result.as_str(), if is_players_turn { "PLAYER" } else { "AGENT" });
    }
}
