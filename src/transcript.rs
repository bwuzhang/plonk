use algebra::fields::PrimeField;
use algebra::{to_bytes, PairingEngine, ToBytes};
use merlin::Transcript;
use poly_commit::kzg10::Commitment;

pub trait TranscriptProtocol<E: PairingEngine> {
    /// Append a `commitment` with the given `label`.
    fn append_commitment(&mut self, label: &'static [u8], comm: &Commitment<E>);

    /// Append `commitments` with the given `label`.
    fn append_commitments(&mut self, labels: Vec<&'static [u8]>, comm: Vec<&Commitment<E>>);

    /// Compute a `label`ed challenge variable.
    fn challenge_scalar(&mut self, label: &'static [u8]) -> E::Fr;
}

impl<E: PairingEngine> TranscriptProtocol<E> for Transcript {
    fn append_commitment(&mut self, label: &'static [u8], comm: &Commitment<E>) {
        self.append_message(label, &to_bytes![comm].unwrap());
    }

    fn append_commitments(&mut self, labels: Vec<&'static [u8]>, comms: Vec<&Commitment<E>>) {
        for (comm, label) in comms.iter().zip(labels.iter()) {
            self.append_message(label, &to_bytes![comm].unwrap());
        }
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> E::Fr {
        let mut buf = [0u8; 64];
        self.challenge_bytes(label, &mut buf);

        let mut scalar = E::Fr::from_random_bytes(&buf);
        while scalar == None {
            scalar = E::Fr::from_random_bytes(&buf)
        }

        scalar.unwrap()
    }
}