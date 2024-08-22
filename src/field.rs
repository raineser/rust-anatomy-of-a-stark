use primitive_types::{U256, U512};
use std::ops::{Add, Sub, Mul, Div, Neg, BitXor};
use std::fmt::{write, Display};

const P:u128 =  1 + 407 * ( 1 << 119 );

pub fn xgd(x: U256, y: U256) -> (U256, U256, U256) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (U256::from(1), U256::from(0));
    let (mut old_t, mut t) = (U256::from(0), U256::from(1));

    while r != U256::from(0) {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    (old_s, old_t, old_r)
}

#[derive(Debug, PartialEq)]
pub struct FieldElement {
    value: U256,
    p: U256
}

// functions that create new FieldElements
impl FieldElement {
    
    fn new(value: U256) -> Self {
        Self{value: value, p: U256::from(P)}
    }

    fn zero() -> Self {
        FieldElement::new(U256::from(0))
    }

    fn one() -> Self {
        FieldElement::new(U256::from(1))
    }

    fn multiply(left: &FieldElement, right: &FieldElement) -> Self {
        FieldElement::new((left.value * right.value) % left.p)
    }

    fn add(left: &FieldElement, right: &FieldElement) -> Self{
        FieldElement::new((left.value + right.value) % left.p)
    }

    fn subtract(left: &FieldElement, right: &FieldElement) -> Self {
        let result = ((U512::from(left.p) + U512::from(left.value)) - U512::from(right.value)) % U512::from(left.p);
        //should never fail since result is mod p
        FieldElement::new(U256::try_from(result).unwrap())
    }

    fn negate(operand: &FieldElement) -> Self {
        FieldElement::new((operand.p - operand.value) % operand.p)
    }

    fn inverse(operand: &FieldElement) -> Self {
        let (a, _, _) = xgd(operand.value, operand.p);
        FieldElement::new(a)
    }

    fn divide(left: &FieldElement, right: &FieldElement) -> Self {
        assert!(!right.is_zero());
        let (a, _, _) = xgd(right.value, right.p);
        FieldElement::new((left.value * a) % left.p)

    }

    fn is_zero(&self) -> bool {
        self.value == U256::from(0)
    }

    fn  generator(&self) -> Self {
        FieldElement::new(U256::from(85408008396924667383611388730472331217u128))
    }

    fn primitive_nth_root(&self, n:u128 ) -> Self {
        assert!(n <= 1 << 119 && (n & (n-1)) == 0);
        let mut root = FieldElement::new(U256::from(85408008396924667383611388730472331217u128));
        root
    }
}


impl<'a, 'b> Add<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: &FieldElement) -> Self::Output {
        FieldElement::add(self, rhs)
    }
}

impl<'a, 'b> Sub<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: &FieldElement) -> Self::Output {
        FieldElement::subtract(self, rhs)
    }
}

impl<'a, 'b> Mul<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: &FieldElement) -> Self::Output {
        FieldElement::multiply(self, rhs)
    }
}

impl<'a, 'b> Div<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;
    fn div(self, rhs: &FieldElement) -> Self::Output {
        FieldElement::divide(self, rhs)
    }
}

impl<'b> Neg for &'b FieldElement {
    type Output = FieldElement;
    fn neg(self) -> Self::Output {
        FieldElement::negate(self)
    }
}

impl<'b> BitXor<u128> for &'b FieldElement {
    type Output = FieldElement;
    fn bitxor(self, exponenet: u128) -> Self::Output {
        let mut acc = FieldElement::one();
        let mut val = FieldElement::new(self.value);
        for i in (0.. format!("{exponenet:b}").to_string().chars().count()).rev() {
            acc = &acc * &acc;
            if (1 << i) & exponenet != 0 {
                acc = &acc * &val;
            }
        }
        acc
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let one = FieldElement::one();
        let two = FieldElement::new(U256::from(2));

        assert_eq!(&one + &two, FieldElement::new(U256::from(3)));

        let p_minus_one = FieldElement::new(U256::from(P) - U256::from(1));

        assert_eq!(&one + &p_minus_one, FieldElement::zero());
    }
}