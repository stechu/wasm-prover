//#![feature(wasm_simd)]
//#![feature(wasm_target_feature)]
//#![feature(target_feature_11)]

use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::{commitment::pedersen::Randomness, SNARK};
use ark_ed_on_bls12_381::Fq;
use ark_ed_on_bls12_381::Fr;
use ark_ff::UniformRand;
use ark_groth16::*;
use ark_serialize::CanonicalDeserialize;
use manta_crypto::CommitmentParam;
use manta_crypto::MantaSerDes;
use crate::pedersen::*;
use crate::pedersen_params::COMMIT_PARAM;
use crate::r1cs::*;
use crate::zk_params::ZK_PARAM;
use crate::blake_params::BLAKE_PARAM;
use crate::blake::{PRFCircuit};
use ark_crypto_primitives::prf::{Blake2s, PRF};

//#[cfg(target_arch = "wasm32")]
//#[target_feature(enable = "simd128")]
pub fn pedersen_prover() {
    let mut rng = rand::thread_rng();
    let len = 128;

    let pedersen_param = CommitmentParam::deserialize(COMMIT_PARAM.data);

    let input = vec![2u8; len];
    let open = Randomness::<JubJub>(Fr::rand(&mut rng));
    let commit = pedersen_commit(&input, &pedersen_param, &open);

    let circuit = PedersenComCircuit {
        param: pedersen_param.clone(),
        input,
        open,
        commit,
    };

    // TODO: remove this and use pre-computed zk_param
    let zk_param =
        <Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey::deserialize_unchecked(ZK_PARAM.data)
            .unwrap();
    //let zk_param = groth_param_gen(pedersen_param);

    // This is the part that we want to benchmark:
    crate::groth_api::groth_proof_gen(&zk_param, circuit.clone(), &[0u8; 32]);
    //groth_proof_gen(&zk_param, circuit.clone(), &[2u8; 32]);

}

pub fn blake_prover() {
    //  fix a secret key
    let sk = [4u8; 32];

    // pk = PRF(sk, 0); which is also the address
    let pk = <Blake2s as PRF>::evaluate(&sk, &[0u8; 32]).unwrap();

    // build the circuit
    let circuit = PRFCircuit {
        seed: sk,
        input: [0u8; 32],
        output: pk,
    };

    // generate a zkp parameters
    //let zk_param = groth_param_gen();

    //let mut g_buf = vec![];
    //CanonicalSerialize::serialize_uncompressed(&zk_param, &mut g_buf).unwrap();
    //println!("const ZK_PARAM_BYTES: [u8; {:?}] = ", g_buf.len());
    //println!("{:?}", g_buf);

    let zk_param =
        <Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey::deserialize_unchecked(BLAKE_PARAM.data)
            .unwrap();

    crate::blake::groth_proof_gen(&zk_param, circuit, &[0u8; 32]);

    //assert!(crate::blake::groth_verify(&zk_param, &proof, &pk));
}