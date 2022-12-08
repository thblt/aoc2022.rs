use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn to_index(&self, (x, y): (isize, isize)) -> usize {
        let x = x as usize;
        let y = y as usize;

        y * self.width + x
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn to_coords(&self, idx: usize) -> (isize, isize) {
        let x = (idx % self.width) as isize;
        let y = (idx / self.width) as isize;
        (x, y)
    }

    pub fn test_coords(&self, x: isize, y: isize) -> bool {
        if x < 0 {
            return false;
        }
        if y < 0 {
            return false;
        }
        let x = x as usize;
        let y = y as usize;

        if x >= self.width {
            return false;
        }
        if y >= self.height {
            return false;
        }
        true
    }
}

impl<T: Copy> Matrix<T> {
    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Self {
        let width = vecs[0].len();
        let height = vecs.len();
        let mut ret = Self::new_default(width, height, vecs[0][0]);
        for y in 0..width {
            for x in 0..height {
                ret[(x as isize, y as isize)] = vecs[y][x]
            }
        }
        ret
    }

    pub fn new_default(width: usize, height: usize, value: T) -> Matrix<T> {
        let mut vec = Vec::with_capacity(height * width);

        for _ in 0..width * height {
            vec.push(value);
        }

        Matrix { vec, width, height }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<T> {
        if self.test_coords(x, y) {
            Some(self[(x, y)])
        } else {
            None
        }
    }

    pub fn get_or(&self, x: isize, y: isize, default: T) -> T {
        if let Some(ret) = self.get(x, y) {
            ret
        } else {
            default
        }
    }

    pub fn draw_with(&self, func: &dyn Fn(&T) -> String) {
        for i in 0..self.vec.len() {
            if i % self.width == 0 {
                println!();
            }
            let val = self.vec[i];
            print!("{}", func(&val));
        }
        println!();
    }
}

impl<T: Copy + Default> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_default(width, height, T::default())
    }
}

impl<T> Index<(isize, isize)> for Matrix<T> {
    type Output = T;

    fn index(&self, coords: (isize, isize)) -> &Self::Output {
        &self.vec[self.to_index(coords)]
    }
}

impl<T> IndexMut<(isize, isize)> for Matrix<T> {
    fn index_mut(&mut self, coords: (isize, isize)) -> &mut Self::Output {
        let i = self.to_index(coords);
        &mut self.vec[i]
    }
}

impl Matrix<u8> {
    pub fn draw09(&self) {
        for i in 0..self.vec.capacity() {
            if i % self.width == 0 {
                println!();
            }
            let val = self.vec[i];
            print!(
                "{}",
                if val < 10 {
                    val.to_string()
                } else {
                    String::from("X")
                }
            );
        }
        println!();
    }
}
