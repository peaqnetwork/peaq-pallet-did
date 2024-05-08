//! The trait definition for the weights of extrinsics.

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn add_attribute(value: u32) -> Weight;
    fn update_attribute(value: u32) -> Weight;
    fn read_attribute() -> Weight;
    fn remove_attribute() -> Weight;
}
