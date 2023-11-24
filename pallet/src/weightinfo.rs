//! The trait definition for the weights of extrinsics.

use frame_support::weights::Weight;

pub trait WeightInfo {
	fn add_attribute() -> Weight;
	fn update_attribute() -> Weight;
	fn read_attribute() -> Weight;
	fn remove_attribute() -> Weight;
}
