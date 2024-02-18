use crate::goldilocks::{G1, G1Affine, G2Affine, G2, Gt};
use crate::goldilocks::Fp as GoldilocksField;
use pairing::{Engine, PairingCurveAffine};

#[derive(Default, Clone, Debug)]
pub struct DummyGoldilocksEngine;

impl PairingCurveAffine for G1Affine {
    type Pair = G2Affine;
    type PairingResult = Gt;

    fn pairing_with(&self, _other: &Self::Pair) -> Self::PairingResult {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl PairingCurveAffine for G2Affine {
    type Pair = G1Affine;
    type PairingResult = Gt;

    fn pairing_with(&self, _other: &Self::Pair) -> Self::PairingResult {
        unimplemented!("non-existent goldilocks curve")
    }
}

impl Engine for DummyGoldilocksEngine {
    type Fr = GoldilocksField;
    type G1 = G1;
    type G1Affine = G1Affine;
    type G2 = G2;
    type G2Affine = G2Affine;
    type Gt = Gt;

    fn pairing(_p: &Self::G1Affine, _q: &Self::G2Affine) -> Self::Gt {
        unimplemented!("non-existent goldilocks curve")
    }
}
