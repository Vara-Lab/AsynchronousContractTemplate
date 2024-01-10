
#![no_std]

use gstd::{ prelude::*, ActorId };
use gmeta::{In,Out,InOut,Metadata};



// 1. Create your own Actions
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    
    // Add Actions
    FirstAction,
    SecondAction,
    ThirdAction,
    
}


// 2. Create your own Events
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {
    
    // Add Events(Example)
    FirstEvent,
    SecondEvent,
    ThirdEvent,
}


// 3. Create your own Struct
#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct CustomStruct {
    firstfield: String,
    secondfield: u128,
    thirdfield: ActorId,
   
}

// 4. Create your init Struct
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitStruct {
   
    // Example:
    pub ft_program_id: ActorId,
}


pub struct ContractMetadata;

// 5. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
     type Init = In<InitStruct>;
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Out<CustomStruct>;

}