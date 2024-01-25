use std::ops::Mul;
use std::ops::Add;
use std::fmt;
#[derive(Copy, Clone)]
pub struct Complex{
    pub real : f64,
    pub imag : f64
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex{
            real : r,
            imag : i
        }
    }

    pub fn abs(&self) -> f64{
        return (self.real*self.real + self.imag*self.imag).abs();
    }
}


impl Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, other : Complex) -> Self {
        Complex::new(self.real*other.real-self.imag*other.imag, self.real*other.imag+self.imag*other.real)
    }
}

impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, fother : f64 ) -> Self{
        Complex::new(self.real * fother, self.imag * fother)
    }
}

impl Add<Complex> for Complex {
    type Output = Self;
    fn add(self, other : Complex) -> Self {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, other : f64) -> Self {
        Complex::new(self.real + other, self.imag + other)
    }
}

impl Add<f32> for Complex {
    type Output = Self;
    fn add(self, other : f32) -> Self {
        Complex::new(self.real + (other as f64), self.imag + (other as f64))
    }
}

impl Add<i64> for Complex {
    type Output = Self;
    fn add(self, other : i64) -> Self {
        Complex::new(self.real + (other as f64), self.imag + (other as f64))
    }
}

impl Add<i32> for Complex {
    type Output = Self;
    fn add(self, other : i32) -> Self {
        Complex::new(self.real + (other as f64), self.imag + (other as f64))
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, i: {})", self.real, self.imag )
    }
}