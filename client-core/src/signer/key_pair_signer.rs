//! A signer that sign message using the provided key pair
use chain_core::common::Proof;
use chain_core::tx::data::address::ExtendedAddr;
use chain_core::tx::witness::tree::RawXOnlyPubkey;
use chain_core::tx::witness::{TxInWitness, TxWitness};
use client_common::{
    Error, ErrorKind, MultiSigAddress, PrivateKey, PrivateKeyAction, PublicKey, Result, ResultExt,
    Transaction,
};

use crate::{SelectedUnspentTransactions, SignCondition, Signer};

/// Signer using key pair
pub struct KeyPairSigner {
    extended_addr: ExtendedAddr,
    proof: Proof<RawXOnlyPubkey>,
    private_key: PrivateKey,
}

impl KeyPairSigner {
    /// Create a new signer using the provided key pair
    #[inline]
    pub fn new(private_key: PrivateKey, public_key: PublicKey) -> Result<Self> {
        let (extended_addr, proof) = generate_extended_addr_and_proof(public_key)?;
        Ok(KeyPairSigner {
            extended_addr,
            proof,
            private_key,
        })
    }

    // Sign a message using the key pair
    pub fn schnorr_sign_message(&self, message: &[u8], signing_addr: &ExtendedAddr) -> Result<TxInWitness> {
        if *signing_addr != self.extended_addr {
            return Err(Error::new(
                ErrorKind::MultiSigError,
                "Signing address does not belong to the key pair",
            ));
        }

        Ok(TxInWitness::TreeSig(
            self.private_key.schnorr_sign_message(message)?,
            self.proof.clone(),
        ))
    }
}

fn generate_extended_addr_and_proof(
    public_key: PublicKey,
) -> Result<(ExtendedAddr, Proof<RawXOnlyPubkey>)> {
    let require_signers = 1;
    let multi_sig_address = MultiSigAddress::new(
        vec![public_key.clone()],
        public_key.clone(),
        require_signers,
    )?;
    let proof = multi_sig_address
        .generate_proof(vec![public_key])?
        .chain(|| (ErrorKind::InvalidInput, "Unable to generate merkle proof"))?;
    let extended_addr = ExtendedAddr::from(multi_sig_address);

    Ok((extended_addr, proof))
}

impl Signer for KeyPairSigner {
    fn schnorr_sign_transaction(
        &self,
        tx: &Transaction,
        selected_unspent_transactions: &SelectedUnspentTransactions<'_>,
    ) -> Result<TxWitness> {
        selected_unspent_transactions
            .iter()
            .map(|(_, output)| self.schnorr_sign(tx, &output.address))
            .collect::<Result<Vec<TxInWitness>>>()
            .map(Into::into)
    }

    fn schnorr_sign_condition(&self, signing_addr: &ExtendedAddr) -> Result<SignCondition> {
        if *signing_addr == self.extended_addr {
            Ok(SignCondition::SingleSignUnlock)
        } else {
            Ok(SignCondition::Impossible)
        }
    }

    fn schnorr_sign(&self, tx: &Transaction, signing_addr: &ExtendedAddr) -> Result<TxInWitness> {
        if *signing_addr != self.extended_addr {
            return Err(Error::new(
                ErrorKind::MultiSigError,
                "Signing address does not belong to the key pair",
            ));
        }

        Ok(TxInWitness::TreeSig(
            self.private_key.schnorr_sign(tx)?,
            self.proof.clone(),
        ))
    }
}
