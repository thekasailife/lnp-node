// LNP Node: node running lightning network protocol and generalized lightning
// channels.
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use amplify::{Slice32, Wrapper};
use lnp::bolt::Lifecycle;
use lnp::p2p::legacy::{ActiveChannelId, ChannelId, Messages, TempChannelId};
use lnp::Extension;

use super::Error;
use crate::channeld::runtime::Runtime;
use crate::service::LogStyle;
use crate::state_machine::{Event, StateMachine};
use crate::{rpc, ServiceId};

/// Channel proposal workflow
#[derive(Debug, Display)]
pub enum ChannelPropose {
    /// asked remote peer to accept a new channel
    #[display("PROPOSED")]
    Proposed,

    /// remote peer accepted our channel proposal
    #[display("ACCEPTED")]
    Accepted,

    /// sent funding txid and commitment signature to the remote peer
    #[display("FUNDING")]
    Funding,

    /// received signed commitment from the remote peer
    #[display("SIGNED")]
    Signed,

    /// awaiting funding transaction to be mined
    #[display("FUNDED")]
    Funded,

    /// funding transaction is mined, awaiting for the other peer confirmation of this fact
    #[display("LOCKED")]
    Locked,
}

impl StateMachine<rpc::Request, Runtime> for ChannelPropose {
    type Error = Error;

    fn next(
        self,
        event: Event<rpc::Request>,
        runtime: &mut Runtime,
    ) -> Result<Option<Self>, Self::Error> {
        let channel_id = runtime.channel.active_channel_id();
        debug!("ChannelPropose {} received {} event", channel_id, event.message);
        let state = match self {
            ChannelPropose::Proposed => finish_proposed(event, runtime),
            ChannelPropose::Accepted => finish_accepted(event, runtime),
            ChannelPropose::Funding => finish_funding(event, runtime),
            ChannelPropose::Signed => finish_signed(event, runtime),
            ChannelPropose::Funded => finish_funded(event, runtime),
            ChannelPropose::Locked => {
                finish_locked(event, runtime)?;
                info!("ChannelPropose {} has completed its work", channel_id);
                return Ok(None);
            }
        }?;
        info!("ChannelPropose {} switched to {} state", channel_id, state);
        Ok(Some(state))
    }
}

impl ChannelPropose {
    /// Computes channel lifecycle stage for the current channel proposal workflow stage
    pub fn lifecycle(&self) -> Lifecycle {
        match self {
            ChannelPropose::Proposed => Lifecycle::Proposed,
            ChannelPropose::Accepted => Lifecycle::Accepted,
            ChannelPropose::Funding => Lifecycle::Funding,
            ChannelPropose::Signed => Lifecycle::Signed,
            ChannelPropose::Funded => Lifecycle::Funded,
            ChannelPropose::Locked => Lifecycle::Locked,
        }
    }
}

// State transitions:

impl ChannelPropose {
    /// Constructs channel proposal state machine
    pub fn with(
        event: Event<rpc::Request>,
        runtime: &mut Runtime,
    ) -> Result<ChannelPropose, Error> {
        let request = match event.message {
            rpc::Request::OpenChannelWith(ref request) => request,
            msg => {
                panic!("channel_propose workflow inconsistency: starting workflow with {}", msg)
            }
        };
        let temp_channel_id = request.channel_req.temporary_channel_id;

        let open_channel = Messages::OpenChannel(request.channel_req.clone());
        runtime.channel.update_from_peer(&open_channel)?;

        let peerd = request.peerd.clone();
        event.complete_msg_service(
            ServiceId::Peer(peerd),
            rpc::Request::PeerMessage(open_channel),
        )?;

        Ok(ChannelPropose::Proposed)
    }

    /// Construct information message for error and client reporting
    pub fn info_message(&self, channel_id: ActiveChannelId) -> String {
        match self {
            ChannelPropose::Proposed => {
                format!(
                    "{} remote peer to {} with temp id {:#}",
                    "Proposing".promo(),
                    "open a channel".promo(),
                    channel_id.promoter()
                )
            }
            _ => todo!(),
        }
    }
}

fn finish_proposed(
    event: Event<rpc::Request>,
    runtime: &mut Runtime,
) -> Result<ChannelPropose, Error> {
    todo!()
}

fn finish_accepted(
    event: Event<rpc::Request>,
    runtime: &mut Runtime,
) -> Result<ChannelPropose, Error> {
    todo!()
}

fn finish_funding(
    event: Event<rpc::Request>,
    runtime: &mut Runtime,
) -> Result<ChannelPropose, Error> {
    todo!()
}

fn finish_signed(
    event: Event<rpc::Request>,
    runtime: &mut Runtime,
) -> Result<ChannelPropose, Error> {
    todo!()
}

fn finish_funded(
    event: Event<rpc::Request>,
    runtime: &mut Runtime,
) -> Result<ChannelPropose, Error> {
    todo!()
}

fn finish_locked(event: Event<rpc::Request>, runtime: &mut Runtime) -> Result<(), Error> { todo!() }