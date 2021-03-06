use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::msgs::Msg;
use crate::ics07_tendermint::consensus_state::ConsensusState;
use crate::ics07_tendermint::header::Header;
use crate::ics23_commitment::CommitmentRoot;
use crate::ics24_host::client::ClientId;

use serde_derive::{Deserialize, Serialize};
use tendermint::account::Id as AccountId;

pub const TYPE_MSG_CREATE_CLIENT: &str = "create_client";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgCreateClient {
    client_id: ClientId,
    header: Header,
    trusting_period: std::time::Duration,
    bonding_period: std::time::Duration,
    signer: AccountId,
}

impl MsgCreateClient {
    pub fn new(
        client_id: ClientId,
        header: Header,
        trusting_period: std::time::Duration,
        bonding_period: std::time::Duration,
        signer: AccountId,
    ) -> Self {
        Self {
            client_id,
            header,
            trusting_period,
            bonding_period,
            signer,
        }
    }
}

impl Msg for MsgCreateClient {
    type ValidationError = crate::ics24_host::error::ValidationError;
    type Header = Header;

    fn route(&self) -> String {
        crate::keys::ROUTER_KEY.to_string()
    }

    fn get_type(&self) -> String {
        TYPE_MSG_CREATE_CLIENT.to_string()
    }

    fn validate_basic(&self) -> Result<(), Self::ValidationError> {
        // Nothing to validate since both ClientId and AccountId perform validation on creation.
        Ok(())
    }

    fn get_sign_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn get_signers(&self) -> Vec<ClientId> {
        vec![self.client_id.clone()]
    }

    fn get_client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn get_header(&self) -> &Self::Header {
        &self.header
    }
}

impl crate::ics02_client::msgs::MsgCreateClient for MsgCreateClient {
    type ConsensusState = ConsensusState;

    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn header(&self) -> &Self::Header {
        &self.header
    }

    fn consensus_state(&self) -> Self::ConsensusState {
        let root = CommitmentRoot; // TODO
        let header = &self.header.signed_header.header;

        ConsensusState::new(
            root,
            header.height.into(),
            header.time,
            self.header.validator_set.clone(),
        )
    }

    fn client_type(&self) -> ClientType {
        ClientType::Tendermint
    }
}
