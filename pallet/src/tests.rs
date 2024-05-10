use crate::{did::Did, Config};
use crate::{mock::*, Error};
use codec::MaxEncodedLen;
use frame_support::{assert_noop, assert_ok, BoundedVec};
use hex_literal::hex;

pub(crate) const NAME: &[u8] = b"id";
pub(crate) const ATTRIBUTE: &[u8] = b"did:pq:1234567890";

fn expected_deposit() -> Balance {
    (DEPOSIT_PER_BYTE
        * ((BOUNDED_DATA_LEN * 2)
            + Moment::max_encoded_len() as u32
            + AccountId::max_encoded_len() as u32) as Balance)
        + DEPOSIT_BASE
}

#[test]
fn add_attribute_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (origin, did_account, _, _, _) = test_accounts();

        assert_ok!(PeaqDID::add_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap(),
            BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
            None
        ));

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&origin),
            expected_deposit()
        );

        // Test for duplicate entry
        assert_noop!(
            PeaqDID::add_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(NAME.to_vec()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None
            ),
            Error::<Test>::AttributeAlreadyExist
        );

        // Test update did attribute with invalid validity
        assert_noop!(
            PeaqDID::add_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(b"name".to_vec()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                Some(u64::MAX),
            ),
            Error::<Test>::MaxBlockNumberExceeded
        );

        // Test add attibute with invalid name length
        assert_noop!(
            PeaqDID::add_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(vec![0; 70]).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None
            ),
            Error::<Test>::AttributeNameExceedMax64
        );

        // verify deposit didnt change after invalid extrinsics
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&origin),
            expected_deposit()
        );
    });
}

#[test]
fn update_attribute_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (origin, did_account, fake_origin, _, _) = test_accounts();

        assert_ok!(PeaqDID::add_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap(),
            BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
            None
        ));

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&origin),
            expected_deposit()
        );

        // Test update owner did attribute
        assert_ok!(PeaqDID::update_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap(),
            BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
            None,
        ));

        // Test update did attribute with invalid validity
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(NAME.to_vec()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                Some(u64::MAX),
            ),
            Error::<Test>::MaxBlockNumberExceeded
        );

        // Test update another owner did attribute
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(fake_origin),
                did_account,
                BoundedVec::try_from(NAME.to_vec()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None,
            ),
            Error::<Test>::AttributeAuthorizationFailed
        );

        // Test update non-existing attribute
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(b"name".to_vec()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None,
            ),
            Error::<Test>::AttributeNotFound
        );

        // Test update attibute with invalid name length
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(vec![0; 70]).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None
            ),
            Error::<Test>::AttributeNameExceedMax64
        );
    });
}

#[test]
fn read_attribute_test() {
    let (origin, _, _, did_account, did_account1) = test_accounts();

    new_test_ext().execute_with(|| {
        assert_ok!(PeaqDID::add_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap(),
            BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
            None
        ));

        // Test read existing attribute
        assert_ok!(PeaqDID::read_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap()
        ));

        // Test read non-existing attribute
        assert_noop!(
            PeaqDID::read_attribute(
                RuntimeOrigin::signed(origin),
                did_account1,
                BoundedVec::try_from(NAME.to_vec()).unwrap()
            ),
            Error::<Test>::AttributeNotFound
        );
    });
}

#[test]
fn remove_attribute_test() {
    new_test_ext().execute_with(|| {
        let (origin, origin1, _, did_account, _) = test_accounts();

        assert_ok!(PeaqDID::add_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap(),
            BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
            None
        ));

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&origin),
            expected_deposit()
        );

        // Test remove owner did attribute
        assert_ok!(PeaqDID::remove_attribute(
            RuntimeOrigin::signed(origin),
            did_account,
            BoundedVec::try_from(NAME.to_vec()).unwrap()
        ));

        // correct storage deposit was deducted or not
        assert_eq!(<Test as Config>::Currency::reserved_balance(&origin), 0);

        // Test remove another owner did attribute
        assert_noop!(
            PeaqDID::remove_attribute(
                RuntimeOrigin::signed(origin1),
                did_account,
                BoundedVec::try_from(NAME.to_vec()).unwrap()
            ),
            Error::<Test>::AttributeAuthorizationFailed
        );

        // Test remove non-existing attribute
        assert_noop!(
            PeaqDID::remove_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(b"name".to_vec()).unwrap()
            ),
            Error::<Test>::AttributeNotFound
        );
    });
}

#[test]
fn hashed_key_correctness_test() {
    new_test_ext().execute_with(|| {
        let did_account = sp_core::sr25519::Public::from_raw(hex!(
            "6031188a7c447201a20c044b5e93a6857683a0186a2e02c799974c94a6e4331d"
        ));
        let name = b"id";
        let expected_result =
            hex!("01621935bef7de2129d77df46f9fc533054dbc82b03df3f4f0ac1ebeab878919");

        assert_eq!(
            PeaqDID::get_hashed_key_for_attr(&did_account, &name[..]),
            expected_result
        )
    });
}
