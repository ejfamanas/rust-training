use std::ops::{Add, AddAssign, Neg};

// deriving will implement element-wise props
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im}
    }
}

// here the generic is specified to ensure that the
// arg supports the add operation
impl<T> Add for Complex<T> where T: Add<Output = T>{
    // associated type. needed to specify the return tupe of
    // the implemented function
    type Output = Complex<T>;

    // implementation to support a + b
    // small self = ref
    // large Self = complex type of generic
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

// extends the += operator
// do not need output because of the self
// constraint where self = T and Self : T
impl<T> AddAssign for Complex <T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// negation
impl<T> Neg for Complex<T> where T: Neg<Output = T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}


// partial eq
// full eq: x = x
// NAN = not a number 0/0 inf/inf
// NAN == NAN -> false
// partial equality implementation
impl<T> PartialEq for Complex<T> where T: PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

impl<T: Eq> Eq for Complex<T> where T: Eq {}


fn main() {
    println!("Hello, world!");
    let mut a = Complex::new(1,2);
    let b = Complex::new(3,4);
    // println!("{:?}", a + b);
    // a += b;
    // println!("{:?}", -a);
    println!("{}", a == b);
    println!("minished!");
}
