#![cfg(feature = "eks")]

extern crate rusoto_core;
extern crate rusoto_eks;

use rusoto_eks::{Eks, EksClient, ListClustersRequest};
use rusoto_core::Region;

#[test]
fn should_list_clusters() {
    let client = EksClient::new(Region::UsEast1);
    let request = ListClustersRequest::default();

    let result = client.list_clusters(request).sync().unwrap();
    println!("{:#?}", result);
}
