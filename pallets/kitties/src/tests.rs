use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchResultWithPostInfo};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let name = *b"12345678";
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);

		let _ = Balances::force_set_balance(RuntimeOrigin::root(), account_id, 10_000_000);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id), name),
			Error::<Test>::InvalidKittyId
		);
		// 判断是否启用了KittyCreated事件
		System::assert_has_event(
			crate::Event::KittyCreated {
				who: account_id,
				kitty_id,
				kitty: KittiesModule::kitties(kitty_id).unwrap(),
			}
			.into(),
		);

		// 判断最后一个事件是否是KittyCreated事件
		System::assert_last_event(
			crate::Event::KittyCreated {
				who: account_id,
				kitty_id,
				kitty: KittiesModule::kitties(kitty_id).unwrap(),
			}
			.into(),
		);
	});
}

#[test]
fn it_works_for_sale() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let name = *b"12345678";
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), account_id, 10_000_000);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_some(), false);
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));
		assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_some(), true);
		System::assert_has_event(crate::Event::KittyOnSale { who: account_id, kitty_id }.into());
		System::assert_last_event(crate::Event::KittyOnSale { who: account_id, kitty_id }.into());
	});
}

#[test]
fn it_works_for_buy() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let saler_id = 0;
		let buyer_id = 1;
		let name = *b"12345678";
		let _saler_id = Balances::force_set_balance(RuntimeOrigin::root(), saler_id, 10_000_000);
		let _buyer_id = Balances::force_set_balance(RuntimeOrigin::root(), buyer_id, 10_000_000);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(saler_id), name));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(saler_id));
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(saler_id), kitty_id));
		assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_some(), true);
		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(buyer_id), kitty_id));
		assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_some(), false);
		System::assert_has_event(Event::KittyBought { who: buyer_id, kitty_id }.into());
		System::assert_last_event(Event::KittyBought { who: buyer_id, kitty_id }.into());
	})
}
