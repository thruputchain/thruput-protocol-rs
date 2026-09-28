// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e671a3694d7312d78364456bb86ee8a18de1502d

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::io::{Read, Write};
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};


pub type Milliseconds = i64;

pub type Seconds = i64;

// ValidatorConfig holds the network configuration of a validator.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorConfig {
    pub private_info: ValidatorPrivateInfo,
    pub known_validators: VecM<ValidatorPublicInfo>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DiscoveryConfig {
    pub advertise_interval_msec: Milliseconds,
    pub advertise_par: u32,
    pub snapshot_ttl_msec: Milliseconds,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RouterConfig {
    pub tree_fan_out: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodConfig {
    // navigator
    pub route_count: DogwoodInt32,
    pub route_time_to_live_msec: Milliseconds,
    pub route_max_latency_msec: Milliseconds,
    // endpoint caller
    pub endpoint_max_pending: DogwoodInt32,
    pub endpoint_num_clients: DogwoodInt32,
    pub endpoint_idle_ttl_msec: Milliseconds,
    pub endpoint_dial_timeout_msec: Milliseconds,
    pub endpoint_redial_delay_msec: Milliseconds,
    pub endpoint_call_timeout_msec: Milliseconds,
    // stream processing
    pub stream_receiver_worker_count: DogwoodInt32,
    pub stream_sender_worker_count: DogwoodInt32,
    // transmission processing
    pub transmission_receiver_decode_worker_count: DogwoodInt32,
    pub transmission_sender_worker_count: DogwoodInt32,
    //
    pub transmission_partial_max_ttl_msec: Milliseconds,
    pub transmission_partial_idle_ttl_msec: Milliseconds,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct MempoolConfig {
    pub transaction_ttl_msec: Milliseconds,
    pub submission_lot_size: i64,
    pub submission_lot_wait_msec: Milliseconds,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CatchupConfig {
    pub catchup_block_ttl_sec: Seconds,
    pub catchup_block_age: BlockHeight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BatonConfig {
    pub skew_msec: Milliseconds,
    pub heartbeat_msec: Milliseconds,
    pub patience_msec: Milliseconds,
    pub max_catchup_height: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RepairConfig {
    pub base_delay_msec: Milliseconds,
    pub max_delay_msec: Milliseconds,
    pub multiplier_nom: u32,
    pub multiplier_denom: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct StorageConfig {
    pub reclaim_interval_msec: Milliseconds,
    pub max_catchup_blocks: i32,
    pub max_scp_slots: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Config {
    pub validator_config: Option<ValidatorConfig>,
    pub discovery_config: Option<DiscoveryConfig>,
    pub router_config: Option<RouterConfig>,
    pub dogwood_config: Option<DogwoodConfig>,
    pub mempool_config: Option<MempoolConfig>,
    pub catchup_config: Option<CatchupConfig>,
    pub scp_config: Option<SCPConfig>,
    pub baton_config: Option<BatonConfig>,
    pub repair_config: Option<RepairConfig>,
    pub storage_config: Option<StorageConfig>,
}


//
// XDR internals below this point
//


