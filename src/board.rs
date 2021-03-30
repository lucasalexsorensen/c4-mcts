

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    pub position: u64,
    pub mask: u64,
    pub moves_count: u8
}

impl Board {
    pub fn get_opponent_position(&self) -> u64 {
        return self.position ^ self.mask;
    }

    pub fn get_legal_moves(&self) -> Vec<u8> {
        return (0..7).filter(|mv| self.is_move_legal(*mv)).collect();
    }
    
    pub fn is_move_legal(&self, mv: u8) -> bool {
        return self.mask & (1 << mv*7 + 5) == 0;
    }

    pub fn into_move(&self, mv: u8) -> Self {
        return Self {
            position: self.position ^ self.mask,
            mask: self.mask | (self.mask + (1 << (mv*7))),
            moves_count: self.moves_count + 1
        }
    }

    pub fn is_draw(&self) -> bool {
        return self.mask >= 279258638311359;
    }

    pub fn is_player_win(&self) -> bool {
        if self.moves_count % 2 == 1 { // 0, 2, 4 => is actually player's turn
            return Self::are_four_connected(self.get_opponent_position());
        }
        return false;
    }

    pub fn is_game_over(&self) -> bool {
        return Self::are_four_connected(self.position) || Self::are_four_connected(self.get_opponent_position());
    }

    pub fn are_four_connected(position: u64) -> bool {
        let mut mask: u64;
        // horizontal check
        mask = position & (position >> 7);
        if (mask & (mask >> 14)) > 0 { return true }
        // diagonal \ (i.e. southeast) check
        mask = position & (position >> 6);
        if (mask & (mask >> 12)) > 0 { return true }
        // diagonal / (i.e. southwest) check
        mask = position & (position >> 8);
        if (mask & (mask >> 16)) > 0 { return true }
        // vertical check
        mask = position & (position >> 1);
        if (mask & (mask >> 2)) > 0 { return true }
        return false;
    }
}

impl From<Vec<u8>> for Board {
    fn from (cells: Vec<u8>) -> Self {
        let mut position_str = String::from("");
        let mut mask_str = String::from("");

        // start with left-most column
        for j in 0..7 {
            // start with bottom-most row
            for i_raw in 0..6 {
                let i = 5 - i_raw;
                let pos = (i*7 + j) as usize;

                mask_str = (if cells[pos] == 0 { "0" } else { "1" }).to_owned() + mask_str.as_str();
                position_str = (if cells[pos]== 1 { "1" } else { "0" }).to_owned() + position_str.as_str();
            }
            // pad with zeros for the 'sentinel' row
            mask_str = "0".to_owned() + mask_str.as_str();
            position_str = "0".to_owned() + position_str.as_str();
        }

        let position = u64::from_str_radix(position_str.as_str(), 2).unwrap();
        let mask = u64::from_str_radix(mask_str.as_str(), 2).unwrap();

        return Board {
            position,
            mask,
            moves_count: mask.count_ones() as u8
        };
    }
}
