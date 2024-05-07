//! Benchmarking setup for did

use super::*;

use crate::structs::Attribute;
#[allow(unused)]
use crate::Pallet as DID;
use frame_benchmarking::v1::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::{Pallet as System, RawOrigin};
use num_traits::bounds::UpperBounded;
use sp_std::vec;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    System::<T>::assert_last_event(generic_event.into());
}

const MAX_ATTRIBUTE_BYTES_LEN: u32 = 4096;
const CALLER_ACCOUNT_STR: &str = "Iredia1";
const DID_ACCOUNT_STR: &str = "Iredia2";
const ATTRITUBE_BYTES: &[u8; MAX_ATTRIBUTE_BYTES_LEN as usize] = &[0; MAX_ATTRIBUTE_BYTES_LEN as usize];

benchmarks! {
    add_attribute {
        let i in 0 .. MAX_ATTRIBUTE_BYTES_LEN;
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let name = vec![1; 64];
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.clone(), ATTRITUBE_BYTES.to_vec(), None)
    verify {
        assert_last_event::<T>(Event::<T>::AttributeAdded(
            caller,
            did_account,
            name,
            ATTRITUBE_BYTES.to_vec(),
            None,
        ).into());
    }

    update_attribute {
        let i in 0 .. MAX_ATTRIBUTE_BYTES_LEN;
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let new_attribute = &[1; MAX_ATTRIBUTE_BYTES_LEN as usize];
        let name = vec![1; 64];
        <DID<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.clone(),
            ATTRITUBE_BYTES.to_vec(),
            None)?;
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.clone(), new_attribute.to_vec(), None)
    verify {
        assert_last_event::<T>(Event::<T>::AttributeUpdated(
            caller.clone(),
            did_account.clone(),
            name,
            new_attribute.to_vec(),
            None,
        ).into());
    }

    read_attribute {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let name = vec![1; 64];
        <DID<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.clone(),
            ATTRITUBE_BYTES.to_vec(),
            None)?;
    }: _(RawOrigin::Signed(caller.clone()), did_account, name.clone())
    verify {
        let read_attr = Attribute::<T::BlockNumber, <<T as Config>::Time as MomentTime>::Moment> {
            name: name,
            value: ATTRITUBE_BYTES.to_vec(),
            validity: T::BlockNumber::max_value(),
            created: T::Time::now(),
        };
        assert_last_event::<T>(Event::<T>::AttributeRead(read_attr).into());
    }

    remove_attribute {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let name = vec![0; 64];
        <DID<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.clone(),
            ATTRITUBE_BYTES.to_vec(),
            None)?;
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.clone())
    verify {
        assert_last_event::<T>(Event::<T>::AttributeRemoved(
            caller.clone(),
            did_account,
            name,
        ).into());
    }

}

#[cfg(test)]
mod tests {
    use crate::mock;
    use frame_support::sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::new_test_ext()
    }
}

impl_benchmark_test_suite!(
    DID,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::Test,
);
