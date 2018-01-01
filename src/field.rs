
const CANDIDATE_MASK: u16 = 0b0000_0011_1111_1110; // 0x03FE


pub struct Field {
    cells: [[Cell; 9];9]
}



pub struct Cell {
    digit: Option<u8>,
    fixed: bool,
    candidates: u16
}

impl Cell {

    pub fn get_digit(&self) -> Option<u8> {
        self.digit
    }

    pub fn empty_cell() -> Cell {
        Cell {
            digit: None,
            fixed: false,
            candidates: CANDIDATE_MASK
        }
    }

    pub fn fixed_cell(dig: u8) -> Cell {
        Cell {
            digit: Some(dig),
            fixed: true,
            candidates: 1u16 << dig
        }
    }
    
    pub fn is_solved(&self) -> bool {
        match self.digit {
            Some(_) => return true,
            None => return false
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    fn set_to(&mut self, dig: u8) -> bool {
        if self.fixed {
            match self.digit {
                Some(x) if (dig == x) => return true,
                _ => return false
            }
        }
        self.digit = Some(dig);
        true  
    }

    fn fix(&mut self) -> bool {
        match self.digit {
            None => return false,
            Some(_) => self.fixed = true
        }
        true
    }

    pub fn fix_to(&mut self, dig: u8) -> bool {
        if self.set_to(dig) && self.fix() {
             return true;
        }
        false
    } 

    pub fn has_candidate(&self, cand: u8) -> bool {
        self.candidates & ( 1u16 << cand ) != 0
    }

    pub fn get_candidates(&self) -> u16 {
        self.candidates
    }

    pub fn get_candidate_vector(&self) -> Vec<u8> {
        let mut v = Vec::new();
        for i in 1..10 {
            if self.candidates & (1u16 << i) != 0 {
                v.push(i)
            }
        }
        v
    }

    pub fn remove_candidate(&mut self, cand: u8) {
        self.candidates &= !(1u16 << cand);
        self.candidates &= CANDIDATE_MASK;
    }

    pub fn remove_candidates(&mut self, cands: Vec<u8>) {
        for cand in cands {
            self.remove_candidate(cand);
        }
    }

    pub fn has_single_candidate(&self) -> bool {
        self.candidates.is_power_of_two()
    }

    pub fn get_naked_single(&self) -> Option<u8> {
        if self.has_single_candidate() {
            let mut bit = 1;
            let mut i = 2; // = 2^bit
            while i != self.candidates {
                i = i << 1;
                bit += 1;
            }
            return Some(bit);
        }
        None
    }

    pub fn try_solve(&mut self) -> bool {
        self.digit = self.get_naked_single();
        match self.digit {
            Some(_) => return true,
            None => return false
        }
    }
}
