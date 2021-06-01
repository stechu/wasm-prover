use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::{commitment::pedersen::Randomness, SNARK};
use ark_ed_on_bls12_381::Fq;
use ark_ed_on_bls12_381::Fr;
use ark_ff::UniformRand;
use ark_groth16::*;
use ark_serialize::CanonicalDeserialize;
use crate::groth_api::*;
use manta_crypto::CommitmentParam;
use manta_crypto::MantaSerDes;
use crate::pedersen::*;
use crate::pedersen_params::COMMIT_PARAM;
use crate::r1cs::*;
use crate::zk_params::ZK_PARAM;

pub fn prover() {
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
    groth_proof_gen(&zk_param, circuit.clone(), &[0u8; 32]);

}
