mod field;

fn main() {
    println!("Hello, Sudoku!");
    let mut first_cell = field::Cell::empty_cell();
    println!("Candidate Array for first Cell is {:016b}", first_cell.get_candidates());
    first_cell.remove_candidate(15);
    println!("Candidate Array for first Cell is {:016b}", first_cell.get_candidates());
    let x = 4;
    println!("first Cell still allows {}? {}", x, first_cell.has_candidate(x));
    first_cell.remove_candidate(x);
    println!("Candidate Array for first Cell is {:016b}", first_cell.get_candidates());
    println!("Candidate Vector for first Cell is {:?}", first_cell.get_candidate_vector());
    println!("first Cell still allows {}? {}", x, first_cell.has_candidate(x));
    first_cell.remove_candidates(vec![1,2,5,6,7,8,9]);
    println!("Candidate Array for first Cell is {:016b}", first_cell.get_candidates());
    println!("Candidate Vector for first Cell is {:?}", first_cell.get_candidate_vector());
    println!("The value in first Cell is {:?}", first_cell.get_digit());
    first_cell.try_solve();
    println!("The value in first Cell is {:?}", first_cell.get_digit());

}
