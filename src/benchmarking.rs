//! Benchmarking setup for did

use super::*;

#[allow(unused)]
use crate::Pallet as DID;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, account};
use frame_system::{Pallet as System, RawOrigin};

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    add_attribute {
        let caller : T::AccountId = account("Iredia1", 0, 0);

        let did_account : T::AccountId = account("Iredia2", 0, 0);
        let name = b"id";
        let attribute = b"did:pq:1234567890";
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.to_vec(), attribute.to_vec(), None)
    verify {
        assert_last_event::<T>(Event::<T>::AttributeAdded(
            caller.into(),
            did_account.clone(),
            name.to_vec(),
            attribute.to_vec(),
            None,
        ).into());
    }

    update_attribute {
        let caller : T::AccountId = account("Iredia1", 0, 0);

        let did_account : T::AccountId = account("Iredia2", 0, 0);
        let name = b"id";
        let attribute = b"did:pq:1234567890";
        let new_attribute = b"did:pq:0987654321";
        <Pallet<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.to_vec(),
            attribute.to_vec(),
            None)?;
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.to_vec(), new_attribute.to_vec(), None)
    verify {
        assert_last_event::<T>(Event::<T>::AttributeUpdated(
            caller.clone(),
            did_account.clone(),
            name.to_vec(),
            new_attribute.to_vec(),
            None,
        ).into());
    }

}

#[cfg(test)]
mod tests {
    use crate::mock;
    use frame_support::sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::ExternalityBuilder::build()
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::TestRuntime,
);
