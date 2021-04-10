use dg2core::{InnerMessage, MessagePart, OuterMessage, Store};

/// Test the usual workflow:
/// - init a store (incl. key pairs)
/// - create a group
/// - init another store
/// - simulate send+recv of a DM with the test group key, without fs roundtrip
#[test]
fn simple() {
    let mut store = Store::new();

    let test_group = dg2core::gm::gen_key();
    store.groupkeys.insert("test".to_string(), test_group.clone());

    let mut store2 = Store::new();

    // DM {test_group}
    let mut msg = InnerMessage::new();
    msg.insert("test.gk".to_string(), MessagePart::GroupKey(test_group));

    let mut msg = OuterMessage::try_new(&msg).expect("unable to serialize InnerMessage");
    msg.attach_signature(&store.pubkey.signs, &store.sgnkey);

    let blob =
        dg2core::dm::encode_message(&msg, &store2.pubkey.msgs).expect("unable to encode message");

    // imagine send+recv here

    let msg = dg2core::dm::decode_message(&blob, &store2.pubkey.msgs, &store2.msgkey)
        .expect("unable to decode message");

    msg.verify().for_each(|i| assert!(i.1));

    let msg = msg.inner().expect("unable to deserialize InnerMessage");

    if let MessagePart::GroupKey(gk) = &msg["test.gk"] {
        assert_eq!(gk, &store.groupkeys["test"]);
        store2.groupkeys.insert("test".to_string(), gk.clone());
    } else {
        panic!("message part test.gk not found");
    }
}
