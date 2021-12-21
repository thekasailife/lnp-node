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

pub(self) mod automata;
#[cfg(feature = "server")]
mod opts;
mod runtime;
mod state;
pub(self) mod storage;

pub use automata::Error;
#[cfg(feature = "server")]
pub use opts::Opts;
pub use runtime::run;
pub(self) use state::ChannelState;
