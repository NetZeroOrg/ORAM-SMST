use kimchi::{loc, snarky::api::SnarkyCircuit, FieldVar};
use mina_curves::pasta::Pallas;
use poly_commitment::ipa::OpeningProof;

use crate::ScalarField;

struct RangeCheckCircuit {}

impl SnarkyCircuit for RangeCheckCircuit {
    type Curve = Pallas;

    type PrivateInput = ScalarField;
    type PublicInput = ();
    type PublicOutput = ();
    type Proof = OpeningProof<Self::Curve>;

    fn circuit(
        &self,
        sys: &mut kimchi::RunState<ScalarField>,
        _: Self::PublicInput,
        private_input: Option<&Self::PrivateInput>,
    ) -> kimchi::SnarkyResult<Self::PublicOutput> {
        let a: FieldVar<ScalarField> = sys.compute(loc!(), |_| *(private_input.unwrap()))?;
        sys.range_check(loc!(), a.clone(), a.clone(), a)?;
        Ok(())
    }
}
