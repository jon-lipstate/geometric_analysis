type float_t = f64;

#[derive(Debug, Clone, Copy)]
pub enum Blade {
    one,
    e0,
    e1,
    e2,
    e01,
    e20,
    e12,
    e012,
}
//TODO: https://stackoverflow.com/questions/41637978/how-to-get-the-number-of-elements-variants-in-an-enum-as-a-constant-value

impl Blade {
    pub fn len() -> usize {
        8
    }
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Blade::one,  //Grade-0, Scalar
            1 => Blade::e0,   //Grade-1?
            2 => Blade::e1,   //Grade-1
            3 => Blade::e2,   //Grade-1
            4 => Blade::e01,  //Grade-2
            5 => Blade::e20,  //Grade-2 //SIGGRAPH CONVENTIONm, https://youtu.be/tX4H_ctggYo?t=3948
            6 => Blade::e12,  //Grade-2
            7 => Blade::e012, //Grade-3, PseudoScalar
            _ => panic!("invalid index"),
        }
    }
    pub fn cayley_str(self: &Blade, other: &Blade) -> String {
        let res = &self.cayley(&other);
        let resStr = match res.0 {
            -1 => format!("-{}", res.1),
            1 => format!("+{}", res.1),
            0 => format!("0"),
            _ => panic!("unreachable"),
        };
        format!("{:?} * {:?} = {}", self, other, &resStr)
    }
    fn cayley(self: &Blade, other: &Blade) -> (isize, Blade) {
        match self {
            Blade::one => match other {
                Blade::one => (1, Blade::one),
                Blade::e0 => (1, Blade::e0),
                Blade::e1 => (1, Blade::e1),
                Blade::e2 => (1, Blade::e2),
                Blade::e01 => (1, Blade::e01),
                Blade::e20 => (1, Blade::e20),
                Blade::e12 => (1, Blade::e12),
                Blade::e012 => (1, Blade::e012),
            },
            Blade::e0 => match other {
                Blade::one => (1, Blade::e0),
                Blade::e0 => (0, Blade::e0),
                Blade::e1 => (1, Blade::e01),
                Blade::e2 => (-1, Blade::e20),
                Blade::e01 => (0, Blade::e01),
                Blade::e20 => (0, Blade::e20),
                Blade::e12 => (1, Blade::e012),
                Blade::e012 => (0, Blade::e012),
            },
            Blade::e1 => match other {
                Blade::one => (1, Blade::e1),
                Blade::e0 => (-1, Blade::e01),
                Blade::e1 => (1, Blade::one),
                Blade::e2 => (1, Blade::e12),
                Blade::e01 => (-1, Blade::e0),
                Blade::e20 => (1, Blade::e012),
                Blade::e12 => (1, Blade::e2),
                Blade::e012 => (1, Blade::e20),
            },
            Blade::e2 => match other {
                Blade::one => (1, Blade::e2),
                Blade::e0 => (1, Blade::e20),
                Blade::e1 => (-1, Blade::e12),
                Blade::e2 => (1, Blade::one),
                Blade::e01 => (1, Blade::e012),
                Blade::e20 => (1, Blade::e0),
                Blade::e12 => (-1, Blade::e1),
                Blade::e012 => (1, Blade::e01),
            },
            Blade::e01 => match other {
                Blade::one => (1, Blade::e01),
                Blade::e0 => (0, Blade::one),
                Blade::e1 => (1, Blade::e0),
                Blade::e2 => (1, Blade::e012),
                Blade::e01 => (0, Blade::one),
                Blade::e20 => (0, Blade::one),
                Blade::e12 => (-1, Blade::e20),
                Blade::e012 => (0, Blade::one),
            },
            Blade::e20 => match other {
                Blade::one => (1, Blade::e20),
                Blade::e0 => (0, Blade::one),
                Blade::e1 => (1, Blade::e012),
                Blade::e2 => (-1, Blade::e0),
                Blade::e01 => (0, Blade::one),
                Blade::e20 => (0, Blade::one),
                Blade::e12 => (1, Blade::e01),
                Blade::e012 => (0, Blade::one),
            },
            Blade::e12 => match other {
                Blade::one => (1, Blade::e12),
                Blade::e0 => (1, Blade::e012),
                Blade::e1 => (-1, Blade::e2),
                Blade::e2 => (1, Blade::e1),
                Blade::e01 => (1, Blade::e20),
                Blade::e20 => (-1, Blade::e01),
                Blade::e12 => (-1, Blade::one),
                Blade::e012 => (-1, Blade::e0),
            },
            Blade::e012 => match other {
                Blade::one => (1, Blade::e012),
                Blade::e0 => (0, Blade::one),
                Blade::e1 => (1, Blade::e20),
                Blade::e2 => (1, Blade::e01),
                Blade::e01 => (0, Blade::one),
                Blade::e20 => (0, Blade::one),
                Blade::e12 => (-1, Blade::e0),
                Blade::e012 => (0, Blade::one),
            },
        }
    }
    pub fn dual_str(&self) -> String {
        let res = match &self {
            Blade::one => (1, Blade::e012),
            Blade::e0 => (1, Blade::e12),
            Blade::e1 => (-1, Blade::e20), //e1 -> -e02
            Blade::e2 => (1, Blade::e01),
            Blade::e01 => (1, Blade::e2),
            Blade::e20 => (-1, Blade::e1), //e02 -> -e1
            Blade::e12 => (1, Blade::e0),
            Blade::e012 => (1, Blade::one),
        };
        let resStr = match res.0 {
            -1 => format!("-{}", res.1),
            1 => format!("+{}", res.1),
            0 => format!("0"),
            _ => panic!("unreachable"),
        };
        format!("!{:?}  = {}", &self, &resStr)
    }
}
impl std::fmt::Display for Blade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Blade::one => write!(f, "1"),
            Blade::e0 => write!(f, "e0"),
            Blade::e1 => write!(f, "e1"),
            Blade::e2 => write!(f, "e2"),
            Blade::e01 => write!(f, "e01"),
            Blade::e20 => write!(f, "e20"),
            Blade::e12 => write!(f, "e12"),
            Blade::e012 => write!(f, "e012"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PGA2D {
    pub mvec: Vec<float_t>,
}

impl PGA2D {
    pub fn zero() -> Self {
        Self {
            mvec: vec![0.0; Blade::len()],
        }
    }
    pub fn basis(blade: Blade) -> Self {
        let mut ret = Self::zero();
        match blade {
            Blade::one => ret.mvec[0] = 1.0,
            Blade::e0 => ret.mvec[1] = 1.0,
            Blade::e1 => ret.mvec[2] = 1.0,
            Blade::e2 => ret.mvec[3] = 1.0,
            Blade::e01 => ret.mvec[4] = 1.0,
            Blade::e20 => ret.mvec[5] = 1.0,
            Blade::e12 => ret.mvec[6] = 1.0,
            Blade::e012 => ret.mvec[7] = 1.0,
        }
        ret
    }
    pub fn new(v: Vec<float_t>) -> Self {
        if v.len() != Blade::len() {
            panic!("mismatched vector length.");
        }
        let mut ret = Self::zero();
        ret.mvec = v;
        ret
    }

    pub fn point_euclidean(x: float_t, y: float_t) -> Self {
        let mut ret = Self::zero(); // !(e0, xe1, ye2)
        ret.set_value(Blade::e01, y);
        ret.set_value(Blade::e20, -x); // (!e1 -> -e20)
        ret.set_value(Blade::e12, 1.0);
        ret
    }
    pub fn direction(x: float_t, y: float_t) -> Self {
        let mut ret = Self::zero();
        ret.set_value(Blade::e20, -x); // flip sign(e20 -> e02)
        ret.set_value(Blade::e01, y);
        ret
    }
    pub fn line(a: float_t, b: float_t, c: float_t) -> Self {
        let mut ret = Self::zero();
        ret.set_value(Blade::e1, a);
        ret.set_value(Blade::e2, b);
        ret.set_value(Blade::e0, c);
        ret
    }

    pub fn set_value(&mut self, Blade: Blade, value: float_t) {
        match Blade {
            Blade::one => self.mvec[0] = value,
            Blade::e0 => self.mvec[1] = value,
            Blade::e1 => self.mvec[2] = value,
            Blade::e2 => self.mvec[3] = value,
            Blade::e01 => self.mvec[4] = value,
            Blade::e20 => self.mvec[5] = value,
            Blade::e12 => self.mvec[6] = value,
            Blade::e012 => self.mvec[7] = value,
        }
    }

    pub fn dual(self: Self) -> Self {
        let mut ret_val = PGA2D::zero();
        ret_val[0] = self[7]; //one
        ret_val[1] = self[6]; //e0
        ret_val[2] = -self[5]; //e1  //Hodges Dual?
        ret_val[3] = self[4]; //e2
        ret_val[4] = self[3]; //e01
        ret_val[5] = -self[2]; //e20 //Hodges Dual?
        ret_val[6] = self[1]; //e12
        ret_val[7] = self[0]; //e012
        ret_val
    }

    pub fn conjugate(self: Self) -> Self {
        let mut ret_val = PGA2D::zero();
        ret_val[0] = self[0];
        ret_val[1] = -self[1];
        ret_val[2] = -self[2];
        ret_val[3] = -self[3];
        ret_val[4] = -self[4];
        ret_val[5] = -self[5];
        ret_val[6] = -self[6];
        ret_val[7] = self[7];
        ret_val
    }

    pub fn involute(self: Self) -> Self {
        let mut res = PGA2D::zero();
        res[0] = self[0];
        res[1] = -self[1];
        res[2] = -self[2];
        res[3] = -self[3];
        res[4] = self[4];
        res[5] = self[5];
        res[6] = self[6];
        res[7] = -self[7];
        res
    }

    pub fn geometric_product(self, other: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        let b = other;
        res[0] = b[0] * a[0] + b[2] * a[2] + b[3] * a[3] - b[6] * a[6];
        res[1] = b[1] * a[0] + b[0] * a[1] - b[4] * a[2] + b[5] * a[3] + b[2] * a[4]
            - b[3] * a[5]
            - b[7] * a[6]
            - b[6] * a[7];
        res[2] = b[2] * a[0] + b[0] * a[2] - b[6] * a[3] + b[3] * a[6];
        res[3] = b[3] * a[0] + b[6] * a[2] + b[0] * a[3] - b[2] * a[6];
        res[4] = b[4] * a[0] + b[2] * a[1] - b[1] * a[2] + b[7] * a[3] + b[0] * a[4] + b[6] * a[5]
            - b[5] * a[6]
            + b[3] * a[7];
        res[5] = b[5] * a[0] - b[3] * a[1] + b[7] * a[2] + b[1] * a[3] - b[6] * a[4]
            + b[0] * a[5]
            + b[4] * a[6]
            + b[2] * a[7];
        res[6] = b[6] * a[0] + b[3] * a[2] - b[2] * a[3] + b[0] * a[6];
        res[7] = b[7] * a[0]
            + b[6] * a[1]
            + b[5] * a[2]
            + b[4] * a[3]
            + b[3] * a[4]
            + b[2] * a[5]
            + b[1] * a[6]
            + b[0] * a[7];
        res
    }

    pub fn outer_product(self, other: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        let b = other;
        res[0] = b[0] * a[0];
        res[1] = b[1] * a[0] + b[0] * a[1];
        res[2] = b[2] * a[0] + b[0] * a[2];
        res[3] = b[3] * a[0] + b[0] * a[3];
        res[4] = b[4] * a[0] + b[2] * a[1] - b[1] * a[2] + b[0] * a[4];
        res[5] = b[5] * a[0] - b[3] * a[1] + b[1] * a[3] + b[0] * a[5];
        res[6] = b[6] * a[0] + b[3] * a[2] - b[2] * a[3] + b[0] * a[6];
        res[7] = b[7] * a[0]
            + b[6] * a[1]
            + b[5] * a[2]
            + b[4] * a[3]
            + b[3] * a[4]
            + b[2] * a[5]
            + b[1] * a[6]
            + b[0] * a[7];
        res
    }
    pub fn meet(self, other: PGA2D) -> PGA2D {
        PGA2D::outer_product(self, other)
    }
    pub fn wedge(self, other: PGA2D) -> PGA2D {
        PGA2D::outer_product(self, other)
    }

    pub fn regressive_product(self, other: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        let b = other;
        res[7] = 1.0 * (a[7] * b[7]);
        res[6] = 1.0 * (a[6] * b[7] + a[7] * b[6]);
        res[5] = 1.0 * (a[5] * b[7] + a[7] * b[5]);
        res[4] = 1.0 * (a[4] * b[7] + a[7] * b[4]);
        res[3] = 1.0 * (a[3] * b[7] + a[5] * b[6] - a[6] * b[5] + a[7] * b[3]);
        res[2] = 1.0 * (a[2] * b[7] - a[4] * b[6] + a[6] * b[4] + a[7] * b[2]);
        res[1] = 1.0 * (a[1] * b[7] + a[4] * b[5] - a[5] * b[4] + a[7] * b[1]);
        res[0] = 1.0
            * (a[0] * b[7]
                + a[1] * b[6]
                + a[2] * b[5]
                + a[3] * b[4]
                + a[4] * b[3]
                + a[5] * b[2]
                + a[6] * b[1]
                + a[7] * b[0]);
        res
    }
    pub fn join(self, other: PGA2D) -> PGA2D {
        PGA2D::regressive_product(self, other)
    }

    pub fn inner_product(self, other: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        let b = other;
        res[0] = b[0] * a[0] + b[2] * a[2] + b[3] * a[3] - b[6] * a[6];
        res[1] = b[1] * a[0] + b[0] * a[1] - b[4] * a[2] + b[5] * a[3] + b[2] * a[4]
            - b[3] * a[5]
            - b[7] * a[6]
            - b[6] * a[7];
        res[2] = b[2] * a[0] + b[0] * a[2] - b[6] * a[3] + b[3] * a[6];
        res[3] = b[3] * a[0] + b[6] * a[2] + b[0] * a[3] - b[2] * a[6];
        res[4] = b[4] * a[0] + b[7] * a[3] + b[0] * a[4] + b[3] * a[7];
        res[5] = b[5] * a[0] + b[7] * a[2] + b[0] * a[5] + b[2] * a[7];
        res[6] = b[6] * a[0] + b[0] * a[6];
        res[7] = b[7] * a[0] + b[0] * a[7];
        res
    }
    pub fn dot_product(self, other: PGA2D) -> PGA2D {
        PGA2D::inner_product(self, other)
    }
}
//Dual
impl std::ops::Not for PGA2D {
    type Output = PGA2D;
    fn not(self: Self) -> PGA2D {
        self.dual()
    }
}
//Geometric Product (*)
impl std::ops::Mul for PGA2D {
    type Output = PGA2D;
    fn mul(self: PGA2D, other: PGA2D) -> PGA2D {
        PGA2D::geometric_product(self, other)
    }
}
//Wedge (^) / Meet / Outer Product
impl std::ops::BitXor for PGA2D {
    type Output = PGA2D;

    fn bitxor(self: PGA2D, other: PGA2D) -> PGA2D {
        PGA2D::outer_product(self, other)
    }
}
// Vee (&)
// The regressive product. (JOIN)
impl std::ops::BitAnd for PGA2D {
    type Output = PGA2D;
    fn bitand(self: PGA2D, other: PGA2D) -> PGA2D {
        PGA2D::regressive_product(self, other)
    }
}
// Dot (|)
// The inner product.
impl std::ops::BitOr for PGA2D {
    type Output = PGA2D;

    fn bitor(self: PGA2D, other: PGA2D) -> PGA2D {
        PGA2D::inner_product(self, other)
    }
}

// Add
// Multivector addition
impl std::ops::Add for PGA2D {
    type Output = PGA2D;

    fn add(self: PGA2D, b: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a[0] + b[0];
        res[1] = a[1] + b[1];
        res[2] = a[2] + b[2];
        res[3] = a[3] + b[3];
        res[4] = a[4] + b[4];
        res[5] = a[5] + b[5];
        res[6] = a[6] + b[6];
        res[7] = a[7] + b[7];
        res
    }
}

// Sub
// Multivector subtraction
impl std::ops::Sub for PGA2D {
    type Output = PGA2D;

    fn sub(self: PGA2D, b: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a[0] - b[0];
        res[1] = a[1] - b[1];
        res[2] = a[2] - b[2];
        res[3] = a[3] - b[3];
        res[4] = a[4] - b[4];
        res[5] = a[5] - b[5];
        res[6] = a[6] - b[6];
        res[7] = a[7] - b[7];
        res
    }
}

// smul
// one/multivector multiplication
impl std::ops::Mul<PGA2D> for float_t {
    type Output = PGA2D;

    fn mul(self: float_t, b: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a * b[0];
        res[1] = a * b[1];
        res[2] = a * b[2];
        res[3] = a * b[3];
        res[4] = a * b[4];
        res[5] = a * b[5];
        res[6] = a * b[6];
        res[7] = a * b[7];
        res
    }
}

// muls
// multivector/one multiplication
impl std::ops::Mul<float_t> for PGA2D {
    type Output = PGA2D;

    fn mul(self: PGA2D, b: float_t) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a[0] * b;
        res[1] = a[1] * b;
        res[2] = a[2] * b;
        res[3] = a[3] * b;
        res[4] = a[4] * b;
        res[5] = a[5] * b;
        res[6] = a[6] * b;
        res[7] = a[7] * b;
        res
    }
}

// sadd
// one/multivector addition
impl std::ops::Add<PGA2D> for float_t {
    type Output = PGA2D;

    fn add(self: float_t, b: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a + b[0];
        res[1] = b[1];
        res[2] = b[2];
        res[3] = b[3];
        res[4] = b[4];
        res[5] = b[5];
        res[6] = b[6];
        res[7] = b[7];
        res
    }
}

// adds
// multivector/one addition
impl std::ops::Add<float_t> for PGA2D {
    type Output = PGA2D;

    fn add(self: PGA2D, b: float_t) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a[0] + b;
        res[1] = a[1];
        res[2] = a[2];
        res[3] = a[3];
        res[4] = a[4];
        res[5] = a[5];
        res[6] = a[6];
        res[7] = a[7];
        res
    }
}

// ssub
// one/multivector subtraction
impl std::ops::Sub<PGA2D> for float_t {
    type Output = PGA2D;

    fn sub(self: float_t, b: PGA2D) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a - b[0];
        res[1] = -b[1];
        res[2] = -b[2];
        res[3] = -b[3];
        res[4] = -b[4];
        res[5] = -b[5];
        res[6] = -b[6];
        res[7] = -b[7];
        res
    }
}
// subs
// multivector/one subtraction
impl std::ops::Sub<float_t> for PGA2D {
    type Output = PGA2D;

    fn sub(self: PGA2D, b: float_t) -> PGA2D {
        let mut res = PGA2D::zero();
        let a = self;
        res[0] = a[0] - b;
        res[1] = a[1];
        res[2] = a[2];
        res[3] = a[3];
        res[4] = a[4];
        res[5] = a[5];
        res[6] = a[6];
        res[7] = a[7];
        res
    }
}

impl std::ops::Index<usize> for PGA2D {
    type Output = float_t;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.mvec[index]
    }
}
impl std::ops::IndexMut<usize> for PGA2D {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.mvec[index]
    }
}
impl std::ops::Index<Blade> for PGA2D {
    type Output = float_t;
    fn index<'a>(&'a self, index: Blade) -> &'a Self::Output {
        match index {
            Blade::one => &self.mvec[0],
            Blade::e0 => &self.mvec[1],
            Blade::e1 => &self.mvec[2],
            Blade::e2 => &self.mvec[3],
            Blade::e01 => &self.mvec[4],
            Blade::e20 => &self.mvec[5],
            Blade::e12 => &self.mvec[6],
            Blade::e012 => &self.mvec[7],
        }
    }
}
impl std::ops::IndexMut<Blade> for PGA2D {
    fn index_mut<'a>(&'a mut self, index: Blade) -> &'a mut Self::Output {
        match index {
            Blade::one => &mut self.mvec[0],
            Blade::e0 => &mut self.mvec[1],
            Blade::e1 => &mut self.mvec[2],
            Blade::e2 => &mut self.mvec[3],
            Blade::e01 => &mut self.mvec[4],
            Blade::e20 => &mut self.mvec[5],
            Blade::e12 => &mut self.mvec[6],
            Blade::e012 => &mut self.mvec[7],
        }
    }
}
impl std::fmt::Display for PGA2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut n = 0;
        let ret = self
            .mvec
            .iter()
            .enumerate()
            .filter_map(|(i, &coeff)| {
                if coeff > 0.00001 || coeff < -0.00001 {
                    n = 1;
                    Some(format!(
                        "{}{}",
                        format!("{:.*}", 7, coeff)
                            .trim_end_matches('0')
                            .trim_end_matches('.'),
                        if i > 0 {
                            Blade::from_index(i).to_string()
                        } else {
                            "".to_string()
                        }
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");
        if n == 0 {
            write!(f, "0")
        } else {
            write!(f, "{}", ret)
        }
    }
}
