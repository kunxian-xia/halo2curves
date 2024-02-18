use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, Mul, Neg, Sub};
use group::Group;
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use crate::goldilocks::Fp as GoldilocksField;
use crate::{
    impl_add_binop_specify_output, impl_binops_additive, impl_binops_additive_specify_output,
    impl_binops_multiplicative, impl_binops_multiplicative_mixed, impl_sub_binop_specify_output,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Gt;

impl Debug for Gt {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for Gt {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConditionallySelectable for Gt {
    fn conditional_select(_a: &Self, _b: &Self, _choice: Choice) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Neg for Gt {
    type Output = Gt;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Group for Gt {
    type Scalar = GoldilocksField;

    fn random(_rng: impl RngCore) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    fn identity() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    fn generator() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    fn is_identity(&self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }

    fn double(&self) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b Gt> for &'a Gt {
    type Output = Gt;

    fn add(self, _rhs: &'b Gt) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b Gt> for &'a Gt {
    type Output = Gt;

    fn sub(self, _rhs: &'b Gt) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Mul<&'b GoldilocksField> for &'a Gt {
    type Output = Gt;

    fn mul(self, _rhs: &'b GoldilocksField) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<T> Sum<T> for Gt
where
    T: Borrow<Gt>
{
    fn sum<I: Iterator<Item=T>>(_iter: I) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl_binops_additive!(Gt, Gt);
impl_binops_multiplicative!(Gt, GoldilocksField);