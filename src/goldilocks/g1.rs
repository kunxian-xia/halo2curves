use crate::goldilocks::Fp as GoldilocksField;
use crate::{
    impl_add_binop_specify_output, impl_binops_additive, impl_binops_additive_specify_output,
    impl_binops_multiplicative, impl_binops_multiplicative_mixed, impl_sub_binop_specify_output,
};
use group::{Curve, Group};
use pasta_curves::arithmetic::{Coordinates, CurveAffine, CurveExt};
use rand_core::RngCore;
use std::fmt::{Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, Mul, Neg, Sub};
use group::prime::{PrimeCurve, PrimeCurveAffine, PrimeGroup};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

/// dummy implementation of non-existent curve whose scalar field is goldilocks field.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct G1;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct G1Affine;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct G1Compressed;

#[derive(Copy, Clone)]
pub struct G1Uncompressed;

impl Debug for G1Compressed {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G1Compressed {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsRef<[u8]> for G1Compressed {
    fn as_ref(&self) -> &[u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsMut<[u8]> for G1Compressed {
    fn as_mut(&mut self) -> &mut [u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl group::GroupEncoding for G1Affine {
    type Repr = G1Compressed;

    fn from_bytes(_bytes: &Self::Repr) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn from_bytes_unchecked(_bytes: &Self::Repr) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn to_bytes(&self) -> Self::Repr {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl group::GroupEncoding for G1 {
    type Repr = G1Compressed;

    fn from_bytes(_bytes: &Self::Repr) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn from_bytes_unchecked(_bytes: &Self::Repr) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn to_bytes(&self) -> Self::Repr {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Debug for G1Uncompressed {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G1Uncompressed {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsRef<[u8]> for G1Uncompressed {
    fn as_ref(&self) -> &[u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsMut<[u8]> for G1Uncompressed {
    fn as_mut(&mut self) -> &mut [u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for G1Uncompressed {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Eq for G1Uncompressed {}

impl PartialEq for G1Uncompressed {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl group::UncompressedEncoding for G1Affine {
    type Uncompressed = G1Uncompressed;

    fn from_uncompressed(_bytes: &Self::Uncompressed) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn from_uncompressed_unchecked(_bytes: &Self::Uncompressed) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn to_uncompressed(&self) -> Self::Uncompressed {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl G1 {
    pub fn generator() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl G1Affine {
    pub fn generator() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    pub fn random(mut _rng: impl RngCore) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G1 {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a> From<&'a G1Affine> for G1 {
    fn from(_value: &'a G1Affine) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl From<G1Affine> for G1 {
    fn from(_value: G1Affine) -> G1 {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl From<G1> for G1Affine {
    fn from(_value: G1) -> G1Affine {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for G1 {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConditionallySelectable for G1 {
    fn conditional_select(_a: &Self, _b: &Self, _choice: Choice) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Group for G1 {
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

impl PrimeCurveAffine for G1Affine {
    type Scalar = GoldilocksField;
    type Curve = G1;

    fn identity() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    fn generator() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    fn is_identity(&self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }

    fn to_curve(&self) -> Self::Curve {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl PrimeGroup for G1 {}

impl Curve for G1 {
    type AffineRepr = G1Affine;

    fn to_affine(&self) -> Self::AffineRepr {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl PrimeCurve for G1 {
    type Affine = G1Affine;
}

impl CurveExt for G1 {
    type ScalarExt = GoldilocksField;
    type Base = GoldilocksField;
    type AffineExt = G1Affine;
    const CURVE_ID: &'static str = "non-existent goldilocks curve";

    fn endo(&self) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    fn jacobian_coordinates(&self) -> (Self::Base, Self::Base, Self::Base) {
        unimplemented!("non-existent goldilocks curve")
    }

    fn hash_to_curve<'a>(_domain_prefix: &'a str) -> Box<dyn Fn(&[u8]) -> Self + 'a> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn is_on_curve(&self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }

    fn a() -> Self::Base {
        unimplemented!("non-existent goldilocks curve")
    }

    fn b() -> Self::Base {
        unimplemented!("non-existent goldilocks curve")
    }

    fn new_jacobian(_x: Self::Base, _y: Self::Base, _z: Self::Base) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<T> Sum<T> for G1
where
    T: core::borrow::Borrow<G1>,
{
    fn sum<I: Iterator<Item=T>>(_iter: I) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Neg for G1 {
    type Output = G1;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a> Neg for &'a G1 {
    type Output = G1;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G1> for &'a G1 {
    type Output = G1;

    fn add(self, _rhs: &'b G1) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G1Affine> for &'a G1 {
    type Output = G1;

    fn add(self, _rhs: &'b G1Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G1> for &'a G1 {
    type Output = G1;

    fn sub(self, _rhs: &'b G1) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G1Affine> for &'a G1 {
    type Output = G1;

    fn sub(self, _rhs: &'b G1Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Mul<&'b GoldilocksField> for &'a G1 {
    type Output = G1;

    fn mul(self, _rhs: &'b GoldilocksField) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a> Neg for &'a G1Affine {
    type Output = G1Affine;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Neg for G1Affine {
    type Output = G1Affine;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G1> for &'a G1Affine {
    type Output = G1;

    fn add(self, _rhs: &'b G1) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G1Affine> for &'a G1Affine {
    type Output = G1;

    fn add(self, _rhs: &'b G1Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G1> for &'a G1Affine {
    type Output = G1;

    fn sub(self, _rhs: &'b G1) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G1Affine> for &'a G1Affine {
    type Output = G1;

    fn sub(self, _rhs: &'b G1Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for G1Affine {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Mul<&'b GoldilocksField> for &'a G1Affine {
    type Output = G1;

    fn mul(self, _other: &'b GoldilocksField) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G1Affine {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConditionallySelectable for G1Affine {
    fn conditional_select(_a: &Self, _b: &Self, _choice: Choice) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl CurveAffine for G1Affine {
    type ScalarExt = GoldilocksField;
    type Base = GoldilocksField;
    type CurveExt = G1;

    fn coordinates(&self) -> CtOption<Coordinates<Self>> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn from_xy(_x: Self::Base, _y: Self::Base) -> CtOption<Self> {
        unimplemented!("non-existent goldilocks curve")
    }

    fn is_on_curve(&self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }

    fn a() -> Self::Base {
        unimplemented!("non-existent goldilocks curve")
    }

    fn b() -> Self::Base {
        unimplemented!("non-existent goldilocks curve")
    }
}



impl_binops_additive!(G1, G1);
impl_binops_additive!(G1, G1Affine);
impl_binops_additive_specify_output!(G1Affine, G1Affine, G1);
impl_binops_additive_specify_output!(G1Affine, G1, G1);
impl_binops_multiplicative!(G1, GoldilocksField);
impl_binops_multiplicative_mixed!(G1Affine, GoldilocksField, G1);