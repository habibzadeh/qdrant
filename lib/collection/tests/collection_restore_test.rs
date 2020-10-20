mod common;

use crate::common::{simple_collection_fixture, load_collection_fixture};
use tempdir::TempDir;
use collection::operations::CollectionUpdateOperations;
use collection::operations::point_ops::PointOps;

#[test]
fn test_collection_reloading() {
    let collection_dir = TempDir::new("collection").unwrap();

    {
        let (_rt, _collection) = simple_collection_fixture(collection_dir.path());
    }

    for _i in 0..10 {
        let (_rt, collection) = load_collection_fixture(collection_dir.path());
        let insert_points = CollectionUpdateOperations::PointOperation(
            PointOps::UpsertPoints {
                ids: vec![0, 1],
                vectors: vec![
                    vec![1.0, 0.0, 1.0, 1.0],
                    vec![1.0, 0.0, 1.0, 0.0],
                ],
            }
        );
        collection.update(insert_points, true).unwrap();
    }

    let (_rt, collection) = load_collection_fixture(collection_dir.path());

    for (_idx, segment) in collection.segments.read().iter() {
        eprintln!("segment.get().read().info() = {:?}", segment.get().read().info());
    }

    eprintln!("collection.info() = {:#?}", collection.info());
}