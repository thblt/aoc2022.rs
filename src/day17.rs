use lib::*;
use lib::matrix::*;

type Shape = Matrix<bool>;

fn read_input(s: &str) -> Vec<isize>{
    read_lines(s)
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Bad input")
        })
        .collect()
}

fn make_shapes() -> impl Iterator<Item = Shape> {
    let mut cross  = Matrix::new_default(3,3,true);
    cross[(0,0)] = false;
    cross[(0,2)] = false;
    cross[(2,0)] = false;
    cross[(2,2)] = false;

    let mut l  = Matrix::new_default(3,3,true);
    l[(0,0)] = false;
    l[(0,1)] = false;
    l[(1,0)] = false;
    l[(1,1)] = false;

    let shapes: Vec<Shape> = vec![
        Matrix::new_default(4, 1, true),
        cross.clone(),
        l.clone(),
        Matrix::new_default(1, 4, true),
        Matrix::new_default(2, 2, true),
    ];

    shapes.into_iter().cycle()

}

fn main() {
    let winds = read_input("inputs/17.txt");
    for s in make_shapes() {
        s.draw_with(&|b| if *b {"#".to_string()} else {".".to_string()})
    }


}
