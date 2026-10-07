use std::sync::LazyLock;
use study_core::learning::LearningSort;
const SUMMARY: &str = include_str!("sql/summary.sql");
pub(super) const FILTER: &str = include_str!("sql/filter.sql");
pub(super) static COUNT: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
        SELECT
          count(*)
        FROM
          collections c {FILTER}
    "#
    )
});
pub(super) static ONE: LazyLock<String> =
    LazyLock::new(|| format!("{SUMMARY} WHERE c.id=$1 AND c.visibility='public'"));
static NEWEST: LazyLock<String> = LazyLock::new(|| {
    format!("{SUMMARY} {FILTER} ORDER BY c.created_at DESC,c.id LIMIT $5 OFFSET $6")
});
static OLDEST: LazyLock<String> =
    LazyLock::new(|| format!("{SUMMARY} {FILTER} ORDER BY c.created_at,c.id LIMIT $5 OFFSET $6"));
static TITLE: LazyLock<String> =
    LazyLock::new(|| format!("{SUMMARY} {FILTER} ORDER BY c.title,c.id LIMIT $5 OFFSET $6"));
pub(super) fn select(sort: LearningSort) -> &'static str {
    match sort {
        LearningSort::Newest => &NEWEST,
        LearningSort::Oldest => &OLDEST,
        LearningSort::Title => &TITLE,
    }
}
