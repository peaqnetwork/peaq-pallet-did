//! Benchmarking setup for did

use super::*;

use crate::structs::Attribute;
#[allow(unused)]
use crate::Pallet as DID;
use frame_benchmarking::v1::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::{Pallet as System, RawOrigin};
use num_traits::bounds::UpperBounded;
use sp_runtime::BoundedVec;
use sp_std::vec;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    System::<T>::assert_last_event(generic_event.into());
}

const MAX_ATTRIBUTE_BYTES_LEN: u32 = 2560;
const CALLER_ACCOUNT_STR: &str = "Iredia1";
const DID_ACCOUNT_STR: &str = "Iredia2";
const ATTRITUBE_BYTES: &[u8; MAX_ATTRIBUTE_BYTES_LEN as usize] = &[0; MAX_ATTRIBUTE_BYTES_LEN as usize];

benchmarks! {
    add_attribute {
        let i in 0 .. MAX_ATTRIBUTE_BYTES_LEN;
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let new_attribute = BoundedVec::try_from([1; MAX_ATTRIBUTE_BYTES_LEN as usize].to_vec()).expect("Length is within bounds");
        let name = BoundedVec::try_from(vec![1; 64]).unwrap();
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.clone(), new_attribute, None)
    verify {
        assert_last_event::<T>(Event::<T>::AttributeAdded(
            caller,
            did_account,
            name.clone(),
            BoundedVec::try_from(ATTRITUBE_BYTES.to_vec()).unwrap(),
            None,
        ).into());
    }

    update_attribute {
        let i in 0 .. MAX_ATTRIBUTE_BYTES_LEN;
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let new_attribute = BoundedVec::<u8, T::BoundedDataLen>::try_from([1; MAX_ATTRIBUTE_BYTES_LEN as usize].to_vec()).unwrap();
        let name = BoundedVec::try_from(vec![1; 64]).unwrap();
        <DID<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.clone(),
            BoundedVec::try_from(ATTRITUBE_BYTES.to_vec()).unwrap(),
            None)?;
    }: _(RawOrigin::Signed(caller.clone()), did_account.clone(), name.clone(), new_attribute.clone(), None)
    verify {
        assert_last_event::<T>(Event::<T>::AttributeUpdated(
            caller.clone(),
            did_account.clone(),
            name,
            new_attribute,
            None,
        ).into());
    }

    read_attribute {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let name = BoundedVec::try_from(vec![1; 64]).unwrap();
        <DID<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.clone(),
            BoundedVec::try_from(ATTRITUBE_BYTES.to_vec()).unwrap(),
            None)?;
    }: _(RawOrigin::Signed(caller.clone()), did_account, name.clone())
    verify {
        let read_attr = Attribute::<T::BlockNumber, <<T as Config>::Time as MomentTime>::Moment> {
            name: vec![1; 64],
            value: ATTRITUBE_BYTES.to_vec(),
            validity: T::BlockNumber::max_value(),
            created: T::Time::now(),
        };
        assert_last_event::<T>(Event::<T>::AttributeRead(read_attr).into());
    }

    remove_attribute {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
        let did_account : T::AccountId = account(DID_ACCOUNT_STR, 0, 0);
        let name = BoundedVec::try_from(vec![1; 64]).unwrap();
        
        <DID<T>>::add_attribute(
            RawOrigin::Signed(caller.clone()).into(),
            did_account.clone(),
            name.clone(),
            BoundedVec::try_from(ATTRITUBE_BYTES.to_vec()).unwrap(),
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
