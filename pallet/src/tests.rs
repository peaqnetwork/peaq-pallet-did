use crate::{did::Did, Config, MAX_VALUE_SIZE};
use crate::{mock::*, Error};
use codec::MaxEncodedLen;
use frame_support::{assert_noop, assert_ok, BoundedVec};
use hex_literal::hex;

pub(crate) const NAME: &[u8] = b"id";
pub(crate) const ATTRIBUTE: &[u8] = b"did:pq:1234567890";

fn expected_deposit() -> Balance {
    (DEPOSIT_PER_BYTE
        * ((MAX_VALUE_SIZE * 2) as u32
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

        // verify deposit didn't change after invalid extrinsics
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
            Error::<Test>::AttributeAuthorizationFailed
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
            Error::<Test>::AttributeAuthorizationFailed
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

// This test was made in order to make sure that even after adding a new attribute for the same account you cannot modify other's people attributes
#[test]
fn override_did_attribute() {
    new_test_ext().execute_with(|| {
        let (alice_key, bob_key, _, did_account_key, _) = test_accounts();

        let name = b"id";
        let attribute = b"did:pq:true";

        let name2 = b"random";
        let attribute2 = b"did:pq:random";

        let attribute_overriden = b"did:pq:false";

        // Alice creates a DID attribute for did_account
        assert_ok!(PeaqDID::add_attribute(
            RuntimeOrigin::signed(alice_key),
            did_account_key,
            BoundedVec::try_from(name.to_vec()).unwrap(),
            BoundedVec::try_from(attribute.to_vec()).unwrap(),
            None
        ));

        // Bob cannot edit Alice's attribute for did_account
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(bob_key),
                did_account_key,
                BoundedVec::try_from(name.to_vec()).unwrap(),
                BoundedVec::try_from(attribute.to_vec()).unwrap(),
                None,
            ),
            Error::<Test>::AttributeAuthorizationFailed
        );

        // Bob adds a new attribute to did_account
        assert_ok!(PeaqDID::add_attribute(
            RuntimeOrigin::signed(bob_key),
            did_account_key,
            BoundedVec::try_from(name2.to_vec()).unwrap(),
            BoundedVec::try_from(attribute2.to_vec()).unwrap(),
            None
        ));

        // Bob cannot override the attribute Alice created
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(bob_key),
                did_account_key,
                BoundedVec::try_from(name.to_vec()).unwrap(),
                BoundedVec::try_from(attribute_overriden.to_vec()).unwrap(),
                None,
            ),
            Error::<Test>::AttributeAuthorizationFailed
        );

        // Bob cannot remove the attribute Alice created.
        assert_noop!(
            PeaqDID::remove_attribute(
                RuntimeOrigin::signed(bob_key),
                did_account_key,
                BoundedVec::try_from(name.to_vec()).unwrap(),
            ),
            Error::<Test>::AttributeAuthorizationFailed
        );
    });
}

#[test]
fn add_attribute_with_non_ascii_test() {
    new_test_ext().execute_with(|| {
        let (origin, did_account, _, _, _) = test_accounts();
        let non_ascii_name = vec![128, 129, 130];
        let non_ascii_attribute = vec![131, 132, 133];

        // Test add attribute with non-ASCII name
        assert_noop!(
            PeaqDID::add_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(non_ascii_name.clone()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None
            ),
            Error::<Test>::AttributeNonAsciiProperty
        );

        // Test add attribute with non-ASCII attribute
        assert_noop!(
            PeaqDID::add_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(NAME.to_vec()).unwrap(),
                BoundedVec::try_from(non_ascii_attribute.clone()).unwrap(),
                None
            ),
            Error::<Test>::AttributeNonAsciiProperty
        );
    });
}

#[test]
fn update_attribute_with_non_ascii_test() {
    new_test_ext().execute_with(|| {
        let (origin, did_account, _, _, _) = test_accounts();
        let non_ascii_name = vec![128, 129, 130];
        let non_ascii_attribute = vec![131, 132, 133];

        // Test update attribute with non-ASCII name
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(non_ascii_name.clone()).unwrap(),
                BoundedVec::try_from(ATTRIBUTE.to_vec()).unwrap(),
                None
            ),
            Error::<Test>::AttributeNonAsciiProperty
        );

        // Test update attribute with non-ASCII attribute
        assert_noop!(
            PeaqDID::update_attribute(
                RuntimeOrigin::signed(origin),
                did_account,
                BoundedVec::try_from(NAME.to_vec()).unwrap(),
                BoundedVec::try_from(non_ascii_attribute.clone()).unwrap(),
                None
            ),
            Error::<Test>::AttributeNonAsciiProperty
        );
    });
}
