
use std::ops::{Add, Sub, Mul, Div, Neg, BitXor};
use crate::field::FieldElement;
use std::cmp;



#[derive(Clone)]
struct Univariate {
    coefficients: Vec<FieldElement>
}

impl Univariate {

    fn new(coefficients: Vec<FieldElement>) -> Self {
        Univariate{coefficients: coefficients}
    }

    fn degree(&self) -> i32 {
        if self.coefficients.len() == 0 {
            return -1;
        }
        let zero = FieldElement::zero();
        let mut maxindex = 0;
        for i in 0..self.coefficients.len() {
            if self.coefficients[i] != zero {
                maxindex = i as i32;
            }
        }
        if maxindex == 0 {
            -1
        } else {
            maxindex
        }
    }

    fn is_zero(&self) -> bool {
        if self.degree() == -1 {
            true
        } else {
            false
        }
    }

}

impl<'a> Neg for &'a Univariate {
    type Output = Univariate;
    fn neg(self) -> Self::Output {
        let mut neg_coefficients = Vec::with_capacity(self.coefficients.len());
        for coef in self.coefficients.iter() {
            neg_coefficients.push(-coef);
        }
        Univariate::new(neg_coefficients)
    }
}

impl<'a, 'b> Add<&'a Univariate> for &'b Univariate {
    type Output = Univariate;
    fn add(self, other: &Univariate) -> Self::Output {
        if self.degree() == -1 {
            return other.clone();
        }
        if other.degree() == -1 {
            return self.clone()
        }
        let mut coeffs = vec![FieldElement::zero(); cmp::max(self.coefficients.len(), other.coefficients.len())];
        for i in 0..self.coefficients.len() {
            coeffs[i] = &coeffs[i] + &self.coefficients[i];
        }
        for i in 0..other.coefficients.len() {
            coeffs[i] = &coeffs[i] + &other.coefficients[i];
        }

        Univariate::new(coeffs)
    }
}

impl<'a, 'b> Sub<&'a Univariate> for &'b Univariate {
    type Output = Univariate;
    fn sub(self, other: &Univariate) -> Self::Output {
       let neg = -other;
       self + &neg
    }
}  

impl<'a, 'b> Mul<&'a Univariate> for &'b Univariate {
    type Output = Univariate;
    fn mul(self, other: &Univariate) -> Self::Output {
        if self.coefficients.len() == 0 || other.coefficients.len() == 0 {
            return  Univariate::new(vec![]);
        }
        let mut buf = vec![FieldElement::zero(); self.coefficients.len() + other.coefficients.len()];
        for i in 0..self.coefficients.len() {
            if self.coefficients[i].is_zero() {
                continue;
            }
            for j in 0..other.coefficients.len() {
                buf[i+j] = &buf[i+j] + &(&self.coefficients[i] * &other.coefficients[j]);
            }
        }
        Univariate::new(buf)
    }
}

impl PartialEq for Univariate {
    fn eq(&self, other: &Univariate) -> bool {
        if self.degree() != other.degree() {
            return false
        } 
        if self.degree() == - 1{
            return true;
        }
        
        self.coefficients.iter().zip(other.coefficients.iter()).fold(true, |mut eq, (left, right)| {
            eq &= left == right;
            eq
        })

    }
}
