use ark_bls12_381::g1::Parameters as G1_Parameters;
use ark_bls12_381::g2::Parameters as G2_Parameters;
use ark_ec::short_weierstrass_jacobian::GroupProjective;
use ark_ec::ProjectiveCurve;

pub mod commitment;
pub mod srs;

type G1 = ark_bls12_381::G1Projective;
type G2 = ark_bls12_381::G2Projective;
//type Gt = ark_bls12_381::Fq12;
type Scalar = ark_bls12_381::Fr;

fn g1_base() -> GroupProjective<G1_Parameters> {
    G1::prime_subgroup_generator()
}

fn g2_base() -> GroupProjective<G2_Parameters> {
    G2::prime_subgroup_generator()
}
