use bytes::Bytes;

use crate::websocket::Client;

use super::{Request, Service, ServiceId, ServiceMap, ServiceSchema};

fn handler(_: Client, _: Request) -> Result<Bytes, &'static str> {
    Err("")
}

fn make_service(name: &str, id: u32) -> Service {
    Service::builder(name, ServiceSchema::new("schema"))
        .with_id(ServiceId::new(id))
        .sync_handler_fn(handler)
}

#[test]
fn test_service_map() {
    let s1 = make_service("s1", 1);
    let s2 = make_service("s2", 2);
    let s3 = make_service("s3", 3);

    // init
    let mut map = ServiceMap::default();
    assert_eq!(map.values().count(), 0);
    assert!(!map.contains_name("s1"));
    assert!(!map.contains_id(ServiceId::new(1)));
    assert!(map.get_by_id(ServiceId::new(1)).is_none());

    // insert
    map.insert(s1);
    assert_eq!(map.values().count(), 1);
    assert!(map.contains_name("s1"));
    assert!(map.contains_id(ServiceId::new(1)));
    assert!(map.get_by_id(ServiceId::new(1)).is_some());
    assert!(!map.contains_name("s2"));
    assert!(!map.contains_id(ServiceId::new(2)));

    // remove
    assert!(map.remove_by_name("s1").is_some());
    assert!(!map.contains_name("s1"));
    assert!(!map.contains_id(ServiceId::new(1)));

    // insert multiple
    map.insert(s2);
    map.insert(s3);
    assert_eq!(map.values().count(), 2);
    assert!(map.get_by_id(ServiceId::new(1)).is_none());
    assert!(map.get_by_id(ServiceId::new(2)).is_some());
    assert!(map.get_by_id(ServiceId::new(3)).is_some());
}
