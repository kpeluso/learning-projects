mod mat {
    use std::fmt;

    pub struct Matrix {
        pub shape: (usize, usize),
        pub vals: Vec<Vec<f64>>,
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[\n")?;
            let vec = &self.vals;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", \n")?; }
                let vec2 = &self.vals[count];
                write!(f, "    [")?; // 4 spaces
                for (count2, v2) in vec2.iter().enumerate() {
                    if count2 != 0 { write!(f, ", ")?; }
                    write!(f, "{:?}", v2)?;
                }
                write!(f, "]")?;
            }
            write!(f, "\n]")
        }
    }

    impl Matrix {
        pub fn scalar_mul(self, s: f64) -> Matrix {
            let mut ans: Vec<Vec<f64>> = Vec::new();
            let mut r: Vec<f64>;
            for i in 0..self.shape.0 {
                r = Vec::new();
                for j in 0..self.shape.1 {
                    r.push(self.vals[i][j] * s);
                }
                ans.push(r);
            }
            let output: Matrix = Matrix { vals: ans, ..self };
            return output;
        }

        pub fn add(self, m: Matrix) -> Matrix {
            if self.shape != m.shape { panic!(".add() ERROR: Matrices must have same shape!"); }
            let mut ans: Vec<Vec<f64>> = Vec::new();
            let mut r: Vec<f64>;
            for i in 0..self.shape.0 {
                r = Vec::new();
                for j in 0..self.shape.1 {
                    r.push(self.vals[i][j] + m.vals[i][j]);
                }
                ans.push(r);
            }
            let output: Matrix = Matrix { vals: ans, ..self };
            return output;
        }

        pub fn sub(self, m: Matrix) -> Matrix {
            if self.shape != m.shape { panic!(".sub() ERROR: Matrices must have same shape!"); }
            return self.add(m.scalar_mul(-1.0_f64));
        }

        pub fn mul(self, m: Matrix) -> Matrix {
            if self.shape.1 != m.shape.0 { panic!(".mul() ERROR: self's num columns not equal to arg's num rows!"); }
            let mut ans: Vec<Vec<f64>> = Vec::new();
            let mut r: Vec<f64>;
            let mut el: f64 = 0.0;
            for i in 0..self.shape.0 {
                r = Vec::new();
                for j in 0..self.shape.1 {
                    el = 0_f64;
                    for k in 0..self.shape.1 {
                        el += self.vals[i][k] * m.vals[k][j]
                    }
                    r.push(el);
                }
                ans.push(r);
            }
            let output: Matrix = Matrix { vals: ans, ..self };
            return output;
        }
    }
}

fn main() {
    let m: mat::Matrix = mat::Matrix {
        shape: (2_usize, 2_usize),
        vals: vec![vec![1_f64,2_f64], vec![3_f64,4_f64]],
    };
    let k: mat::Matrix = mat::Matrix {
        shape: (2_usize, 2_usize),
        vals: vec![vec![1_f64,2_f64], vec![3_f64,4_f64]],
    };
    println!("{}", m);
    println!("{}", k);
    println!("{}", m.mul(k));
}
