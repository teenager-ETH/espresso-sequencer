// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

//! # Node Validator Service
//!
//! The Node Validator Service is a general purpose relay service that watches
//! data flow from the Hot Shot protocol via the CDN pub sub service. It
//! maintains a local state of the network map and is able to relay the
//! stored details to any client that requests it. In addition it is also
//! able to provide individual state change updates to any client that
//! subscribes to that particular event stream.  In order to be able to
//! provide identity information to the clients, this identity information
//! must be volunteered by the nodes in the network.  This requires the
//! nodes to be able to receive and respond to these requests, and relay
//! to anyone who desires it, the identity information of the node.
//!
//! ## Storage
//!
//! In order for this service to be effective and efficient it needs to be
//! able to store the state of the network in an efficient manner.  The
//! storage should be fast and efficient.  We are not expecting a lot of
//! data to be stored within this storage, but as things tend to grow and
//! change it may be necessary to have more robust storage mechanisms in
//! place, or even to have the ability to introduce new storage mechanisms.
//! In order to effectively store the data that we need to store, we need
//! to ask a fundamental question:
//!
//! What states do we need to track?
//! 1. Node Information
//!   a. Node Identity Information
//!   b. Node State Information (specifically voter participation, latest block
//!      information, and staking information)
//! 2. Network Information
//!   a. Latest Block
//!   b. The most recent N blocks (N assumed to be 50 at the moment)
//!     - Information can be derived from these most recent 50 blocks
//!       that allows us to derive histogram data, producer data, and
//!       the most recent block information.  We might be able to get away with
//!       just storing the header information of these blocks, since we don't
//!       need the full block data.
//!   c. The most recent N votes participants
//!   d. The top block producers over the latest N blocks
//!   e. Histogram data for the latest N blocks
//!     - Block Size
//!     - Block Time
//!     - Block Space Used
//!
//! ## Data Streams
//!
//! In order for clients to be able to receive the information from the node
//! validator service, we need to be able to facilitate requests.  We could
//! simply just start streaming data to the clients as soon as they connect,
//! however, this causes potential compatibility issues with the clients
//! in question.  For example, if we want to add a new data stream that
//! can be retrieved for the client, and the client isn't expecting it, they
//! won't know how to handle the data, and it can potentially cause errors.
//! As such, it makes sense to only provide data streams when the client asks
//! for them.  This allows for new features to be added to the data stream
//! without breaking compatibility with the clients, provided that the existing
//! streams don't change in a way that would break the client.
//!
//! Starting out, there doesn't need to be a lot of data that needs to be
//! streamed to to the client.  In fact, we might be able to be a little
//! naive about this, and broadcast general objects in an event stream, as
//! data may be derivable from the objects that are broadcast.  For example,
//! if we start out by sending the latest N block information, the client
//! may be able to derive histogram data from that information, which would
//! prevent us from having to send and store the histogram data.  However,
//! there may be some pieces of data that are lacking from this approach which
//! would require us to send out additional data streams.
//!
//! Ideally, we should strive for a balance between the data we store locally
//! and the data that we stream to the clients. In order to know what we
//! need to store, we need to know what data we are expecting the client to
//! consume, and which data can be derived for these purposes.
//!
//! What Data Streams do we need to provide to clients?
//! 1. Node Information
//!    a. Node Identity Information
//!      - Should be able to be sent in an initial batch
//!      - Should be able to send individual updates as they occur
//!    b. Node State Information
//!      - Should be able to be sent in an initial batch
//!      - Should be able to send individual updates as they occur
//!    c. Block Information
//!      - Should be able to be sent in an initial batch
//!      - Should be able to send individual updates as they occur

pub mod api;
pub mod service;

#[cfg(test)]
pub mod test;

/// Storage is a general purpose trait that allows for the storage of
/// arbitrary data.  This trait allows for the specification of the
/// Get result to be different than that of the Set result.  This should
/// allow for a larger degree of flexibility when it comes to storing things.
pub trait Storage {
    type Get;
    type Set;
    fn get(&self) -> Self::Get;
    fn set(&mut self, value: Self::Set);
}

/// KeyValueStorage is a general purpose trait that allows for the storage
/// of key value pairs.  This trait allows for the specification of the
/// Key and Value types to be different.  This should allow for a larger
/// degree of flexibility when it comes to storing things.
pub trait KeyValueStorage {
    type Key: Eq;
    type Value: Clone;
    fn get(&self, key: &Self::Key) -> &Self::Value;
    fn set(&mut self, key: &Self::Key, value: Self::Value);
}

pub struct NodeInformation {}
