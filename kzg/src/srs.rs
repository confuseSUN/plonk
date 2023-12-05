use ark_bls12_381::{Fr, G1Projective, G2Projective};
use ark_ec::Group;
use ark_ff::{One, UniformRand};
use ark_std::rand::Rng;
use std::ops::{Mul, MulAssign};

/// Structured Reference String, defined over BLS12-381 curve.
#[derive(Debug, Clone)]
pub struct SRS {
    pub g1: Vec<G1Projective>,
    pub g2: Vec<G2Projective>,
}

impl SRS {
    pub fn new<R: Rng>(max_degree: usize, rng: &mut R) -> SRS {
        let r = Fr::rand(rng);

        let mut g1 = Vec::new();
        g1.push(G1Projective::generator());
        for i in 0..max_degree {
            let ele = g1[i].mul(&r);
            g1.push(ele)
        }

        let g2 = vec![G2Projective::generator(), G2Projective::generator().mul(r)];

        SRS { g1, g2 }
    }

    /// Update SRS.
    pub fn update<R: Rng>(&mut self, rng: &mut R) {
        assert!(self.g2.len() == 2);
        let r = Fr::rand(rng);

        let mut r_pow = Fr::one();
        for x in self.g1.iter_mut().skip(1) {
            r_pow.mul_assign(&r);
            x.mul_assign(r_pow);
        }

        self.g2[1].mul_assign(r);
    }

    // /// The public setup parameters come from https://github.com/findora-crypto/export-setup-parameters.
    // pub fn load_from_public_setup_parameters(max_degree: usize) -> SRS {
    //     let g1 = pub_srs::export_g1_from_public_setup_parameters(max_degree);
    //     let g2 = pub_srs::export_g2_from_public_setup_parameters();

    //     SRS { g1, g2 }
    // }
}
