use std::fs::File;
use std::io::BufReader;
use ark_circom::{CircomBuilder, CircomConfig};
use ark_std::rand::thread_rng;
use color_eyre::Result;

use ark_bn254::Bn254;
use ark_groth16::{
    create_random_proof as prove, generate_random_parameters, prepare_verifying_key, verify_proof,
};

#[test]
fn parse_input_prove() -> Result<()> {
    let cfg = CircomConfig::<Bn254>::new(
        "./test-vectors/mycircuit.wasm",
        "./test-vectors/mycircuit.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    let input_file = File::open("./test-vectors/mycircuit-input1.json")?;
    builder.push_inputs(BufReader::new(input_file));

    // create an empty instance for setting it up
    let circom = builder.setup();

    let mut rng = thread_rng();
    let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    let circom = builder.build()?;

    let inputs = circom.get_public_inputs().unwrap();

    let proof = prove(circom, &params, &mut rng)?;

    let pvk = prepare_verifying_key(&params.vk);

    let verified = verify_proof(&pvk, &proof, &inputs)?;

    assert!(verified);

    Ok(())
}

#[test]
fn parse_wrong_input() {
    for input in vec![
        "mycircuit-input-wrong1",
        "mycircuit-input-wrong2",
        // "mycircuit-input-wrong3" // TODO: builder currently does not fail if an input is not provided
    ] {
        let cfg = CircomConfig::<Bn254>::new(
            "./test-vectors/mycircuit.wasm",
            "./test-vectors/mycircuit.r1cs",
        ).unwrap();

        let mut builder = CircomBuilder::new(cfg);
        // `foo' isn't a public input to the circuit, should fail
        // let filename = ;
        // let path = Path::new(filename.as_str());
        let input_file = File::open(format!("./test-vectors/{}.json", input)).unwrap();
        builder.push_inputs(BufReader::new(input_file));

        // create an empty instance for setting it up
        let circom = builder.setup();

        let mut rng = thread_rng();
        let _params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng).unwrap();

        let _ = builder.build().unwrap_err();
    }
}

#[test]
#[cfg(feature = "circom-2")]
fn groth16_proof_circom2() -> Result<()> {
    let cfg = CircomConfig::<Bn254>::new(
        "./test-vectors/circom2_multiplier2.wasm",
        "./test-vectors/circom2_multiplier2.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 11);

    // create an empty instance for setting it up
    let circom = builder.setup();

    let mut rng = thread_rng();
    let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    let circom = builder.build()?;

    let inputs = circom.get_public_inputs().unwrap();

    let proof = prove(circom, &params, &mut rng)?;

    let pvk = prepare_verifying_key(&params.vk);

    let verified = verify_proof(&pvk, &proof, &inputs)?;

    assert!(verified);

    Ok(())
}

#[test]
#[cfg(feature = "circom-2")]
fn witness_generation_circom2() -> Result<()> {
    let cfg = CircomConfig::<Bn254>::new(
        "./test-vectors/circom2_multiplier2.wasm",
        "./test-vectors/circom2_multiplier2.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 0x100000000u64 - 1);

    assert!(builder.build().is_ok());

    Ok(())
}
