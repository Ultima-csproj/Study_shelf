use std::error::Error;
use std::io;

use study_core::error::RepositoryErrorKind;

use super::map_sqlx_error;

#[test]
fn closed_and_timed_out_pools_are_unavailable_and_preserve_source() {
    for source in [sqlx::Error::PoolClosed, sqlx::Error::PoolTimedOut] {
        let error = map_sqlx_error(source);
        assert_eq!(error.kind(), RepositoryErrorKind::Unavailable);
        assert!(
            error
                .source()
                .unwrap()
                .downcast_ref::<sqlx::Error>()
                .is_some()
        );
    }
}

#[test]
fn decoding_errors_are_invalid_data_and_preserve_nested_source() {
    let error = map_sqlx_error(sqlx::Error::ColumnDecode {
        index: "course".to_owned(),
        source: Box::new(io::Error::new(io::ErrorKind::InvalidData, "invalid value")),
    });
    assert_eq!(error.kind(), RepositoryErrorKind::InvalidData);
    assert_eq!(
        error.source().unwrap().source().unwrap().to_string(),
        "invalid value"
    );
}

#[test]
fn query_and_worker_errors_are_unexpected() {
    for source in [
        sqlx::Error::ColumnNotFound("missing".to_owned()),
        sqlx::Error::WorkerCrashed,
    ] {
        let error = map_sqlx_error(source);
        assert_eq!(error.kind(), RepositoryErrorKind::Unexpected);
        assert!(error.source().is_some());
    }
}
