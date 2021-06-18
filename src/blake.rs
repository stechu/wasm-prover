use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::{
    prf::{blake2s::constraints::Blake2sGadget, Blake2s, PRFGadget, PRF},
    SNARK,
};
use ark_ed_on_bls12_381::Fq;
use ark_ff::ToConstraintField;
use ark_groth16::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use ark_r1cs_std::{alloc::AllocVar, prelude::*};

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub type PRFSeed = [u8; 32];
pub type PRFInput = [u8; 32];
pub type PRFOutput = [u8; 32];

#[derive(Clone)]
pub struct PRFCircuit {
    pub seed: PRFSeed,
    pub input: PRFInput,
    pub output: PRFOutput,
}

impl ConstraintSynthesizer<Fq> for PRFCircuit {
    /// Input a circuit, build the constraint system and add it to `cs`
    fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<(), SynthesisError> {
        prf_circuit_helper(&self.seed, &self.input, &self.output, cs)?;
        Ok(())
    }
}

/// generate CRS given parameter of pedersen hash
#[allow(dead_code)]
pub fn groth_param_gen() -> <Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey {
    let sk = [1u8; 32];
    let input = [2u8; 32];
    let out = <Blake2s as PRF>::evaluate(&sk, &input).unwrap();

    let circuit = PRFCircuit {
        seed: sk,
        input: input,
        output: out,
    };

    let mut rng = rand::thread_rng();

    generate_random_parameters::<Bls12_381, _, _>(circuit, &mut rng).unwrap()
}

#[allow(dead_code)]
pub fn groth_proof_gen(
    param: &<Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey,
    circuit: PRFCircuit,
    seed: &[u8; 32],
) -> <Groth16<Bls12_381> as SNARK<Fq>>::Proof {
    let mut rng = ChaCha20Rng::from_seed(*seed);
    create_random_proof(circuit, &param, &mut rng).unwrap()
}

#[allow(dead_code)]
pub fn groth_verify(
    param: &<Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey,
    proof: &<Groth16<Bls12_381> as SNARK<Fq>>::Proof,
    output: &[u8; 32],
) -> bool {
    let pvk = prepare_verifying_key(&param.vk);
    let output_fq: Vec<Fq> = ToConstraintField::<Fq>::to_field_elements(output.as_ref()).unwrap();
    verify_proof(&pvk, &proof, &output_fq).unwrap()
}

// =============================
// A helper function to generate the prf circuit
//     pk = PRF(sk, [0u8;32])
//     void_number = PRF(sk, rho)
// the output pk is hidden, while sn can be public
// =============================
pub(crate) fn prf_circuit_helper(
    seed: &[u8; 32],
    input: &[u8; 32],
    output: &[u8; 32],
    cs: ConstraintSystemRef<Fq>,
) -> Result<(), SynthesisError> {
    // step 1. Allocate seed
    let seed_var = Blake2sGadget::new_seed(cs.clone(), &seed);

    // step 2. Allocate inputs
    let input_var = UInt8::new_witness_vec(ark_relations::ns!(cs, "declare_input"), input)?;

    // step 3. Allocate evaluated output
    let output_var = Blake2sGadget::evaluate(&seed_var, &input_var)?;

    // step 4. Actual output
    let actual_out_var = <Blake2sGadget as PRFGadget<_, Fq>>::OutputVar::new_input(
        ark_relations::ns!(cs, "declare_output"),
        || Ok(output),
    )?;

    // step 5. compare the outputs
    output_var.enforce_equal(&actual_out_var)?;

    Ok(())
}
