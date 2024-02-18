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
pub struct G2;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct G2Affine;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct G2Compressed;

#[derive(Copy, Clone)]
pub struct G2Uncompressed;


impl Debug for G2Compressed {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G2Compressed {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsRef<[u8]> for G2Compressed {
    fn as_ref(&self) -> &[u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsMut<[u8]> for G2Compressed {
    fn as_mut(&mut self) -> &mut [u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl group::GroupEncoding for G2Affine {
    type Repr = G2Compressed;

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

impl group::GroupEncoding for G2 {
    type Repr = G2Compressed;

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

impl Debug for G2Uncompressed {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G2Uncompressed {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsRef<[u8]> for G2Uncompressed {
    fn as_ref(&self) -> &[u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl AsMut<[u8]> for G2Uncompressed {
    fn as_mut(&mut self) -> &mut [u8] {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for G2Uncompressed {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Eq for G2Uncompressed {}

impl PartialEq for G2Uncompressed {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl group::UncompressedEncoding for G2Affine {
    type Uncompressed = G2Uncompressed;

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

impl G2 {
    pub fn generator() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl G2Affine {
    pub fn generator() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }

    pub fn random(mut _rng: impl RngCore) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G2 {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a> From<&'a G2Affine> for G2 {
    fn from(_value: &'a G2Affine) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl From<G2Affine> for G2 {
    fn from(_value: G2Affine) -> G2 {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl From<G2> for G2Affine {
    fn from(_value: G2) -> G2Affine {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for G2 {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConditionallySelectable for G2 {
    fn conditional_select(_a: &Self, _b: &Self, _choice: Choice) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Group for G2 {
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

impl PrimeCurveAffine for G2Affine {
    type Scalar = GoldilocksField;
    type Curve = G2;

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

impl PrimeGroup for G2 {}

impl Curve for G2 {
    type AffineRepr = G2Affine;

    fn to_affine(&self) -> Self::AffineRepr {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl PrimeCurve for G2 {
    type Affine = G2Affine;
}

impl CurveExt for G2 {
    type ScalarExt = GoldilocksField;
    type Base = GoldilocksField;
    type AffineExt = G2Affine;
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

impl<T> Sum<T> for G2
where
    T: core::borrow::Borrow<G2>,
{
    fn sum<I: Iterator<Item=T>>(_iter: I) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Neg for G2 {
    type Output = G2;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a> Neg for &'a G2 {
    type Output = G2;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G2> for &'a G2 {
    type Output = G2;

    fn add(self, _rhs: &'b G2) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G2Affine> for &'a G2 {
    type Output = G2;

    fn add(self, _rhs: &'b G2Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G2> for &'a G2 {
    type Output = G2;

    fn sub(self, _rhs: &'b G2) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G2Affine> for &'a G2 {
    type Output = G2;

    fn sub(self, _rhs: &'b G2Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Mul<&'b GoldilocksField> for &'a G2 {
    type Output = G2;

    fn mul(self, _rhs: &'b GoldilocksField) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a> Neg for &'a G2Affine {
    type Output = G2Affine;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Neg for G2Affine {
    type Output = G2Affine;

    fn neg(self) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G2> for &'a G2Affine {
    type Output = G2;

    fn add(self, _rhs: &'b G2) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Add<&'b G2Affine> for &'a G2Affine {
    type Output = G2;

    fn add(self, _rhs: &'b G2Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G2> for &'a G2Affine {
    type Output = G2;

    fn sub(self, _rhs: &'b G2) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Sub<&'b G2Affine> for &'a G2Affine {
    type Output = G2;

    fn sub(self, _rhs: &'b G2Affine) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConstantTimeEq for G2Affine {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl<'a, 'b> Mul<&'b GoldilocksField> for &'a G2Affine {
    type Output = G2;

    fn mul(self, _other: &'b GoldilocksField) -> Self::Output {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Default for G2Affine {
    fn default() -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl ConditionallySelectable for G2Affine {
    fn conditional_select(_a: &Self, _b: &Self, _choice: Choice) -> Self {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl CurveAffine for G2Affine {
    type ScalarExt = GoldilocksField;
    type Base = GoldilocksField;
    type CurveExt = G2;

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

impl_binops_additive!(G2, G2);
impl_binops_additive!(G2, G2Affine);
impl_binops_additive_specify_output!(G2Affine, G2Affine, G2);
impl_binops_additive_specify_output!(G2Affine, G2, G2);
impl_binops_multiplicative!(G2, GoldilocksField);
impl_binops_multiplicative_mixed!(G2Affine, GoldilocksField, G2);