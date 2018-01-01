
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

    pub fn get_digit(&self) -> Option<u8> {
        self.digit
    }

    fn fix(&mut self) -> bool {
        match self.digit {
            None => return false,
            Some(_) => self.fixed = true
        }
        true
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_candidates_and_solving() {
        let mut first_cell = Cell::empty_cell();
        assert!(first_cell.get_candidates() == CANDIDATE_MASK);
        first_cell.remove_candidate(15);
        assert!(first_cell.get_candidates() == CANDIDATE_MASK);
        let x = 4;
        assert!(first_cell.has_candidate(x));
        first_cell.remove_candidate(x);
        assert!(!first_cell.has_candidate(x));
        assert!(first_cell.get_candidates() == CANDIDATE_MASK ^ (1 << x));
        assert!(first_cell.get_candidate_vector() == vec![1,2,3,5,6,7,8,9]);
        assert!(first_cell.get_digit() == None);
        first_cell.try_solve();
        assert!(first_cell.get_digit() == None);
        first_cell.remove_candidates(vec![1,2,5,6,7,8,9]);
        assert!(first_cell.get_candidates() == (1 << 3));
        assert!(first_cell.get_candidate_vector() == vec![3]);
        assert!(first_cell.get_digit() == None);
        first_cell.try_solve();
        assert!(first_cell.get_digit() == Some(3));
    }

    #[test]
    fn cell_fixing() {
        let mut fixed_cell = Cell::fixed_cell(5);
        assert!(fixed_cell.is_fixed());
        assert!(!fixed_cell.set_to(4));
        assert!(fixed_cell.set_to(5));
        let mut empty_cell = Cell::empty_cell();
        assert!(empty_cell.set_to(4));
        assert!(empty_cell.get_digit()==Some(4));
        assert!(empty_cell.set_to(2));
        assert!(empty_cell.get_digit()==Some(2));
        assert!(empty_cell.fix());
        assert!(empty_cell.is_fixed());
        assert!(empty_cell.set_to(2));
        assert!(!empty_cell.set_to(4));
        let mut another_cell = Cell::empty_cell();
        assert!(another_cell.fix_to(7));
        assert!(another_cell.is_fixed());
        assert!(!another_cell.fix_to(3));
    }
}
