use primitive_types::{U256, U512};

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
}

