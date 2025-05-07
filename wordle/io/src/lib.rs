#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

pub struct WordleMetadata;

impl Metadata for WordleMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<WordleAction, WordleEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<WordleStateQuery, WordleStateReply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WordleAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WordleEvent {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WordleStateQuery {
    All,
    Player(ActorId),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WordleStateReply {
    All(Vec<ActorId>),
    Game(WordleGame),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct WordleGame {
    pub word: String,
    pub attempts: u8,
    pub max_attempts: u8,
}
