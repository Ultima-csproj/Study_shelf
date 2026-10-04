use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::{FromRow, PgConnection};
use study_core::catalog::{
    CatalogFacets, DisciplineFacet, DisciplineReference, TeacherFacet, TeacherReference, TypeFacet,
    ValueFacet,
};
use study_core::error::RepositoryResult;
use study_core::ids::{DisciplineId, TeacherId};

#[derive(FromRow)]
struct FacetRow {
    kind: String,
    value: Option<i64>,
    name: Option<String>,
    slug: Option<String>,
    material_count: i64,
}

pub(super) async fn load(conn: &mut PgConnection) -> RepositoryResult<CatalogFacets> {
    let mut result = CatalogFacets {
        types: Vec::with_capacity(5),
        disciplines: Vec::new(),
        teachers: Vec::new(),
        courses: Vec::new(),
        years: Vec::new(),
    };
    let mut rows = sqlx::query_as::<_, FacetRow>(include_str!("sql/facets.sql")).fetch(conn);
    while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
        let count = row.material_count;
        match row.kind.as_str() {
            "type" => result.types.push(TypeFacet {
                material_type: crate::material::mapping::type_from_db(
                    row.name
                        .as_deref()
                        .ok_or_else(|| crate::error::invalid_relation("type facet"))?,
                )?,
                material_count: count,
            }),
            "discipline" => result.disciplines.push(DisciplineFacet {
                discipline: DisciplineReference {
                    id: DisciplineId(
                        row.value
                            .ok_or_else(|| crate::error::invalid_relation("discipline facet"))?,
                    ),
                    name: row
                        .name
                        .ok_or_else(|| crate::error::invalid_relation("discipline name"))?,
                    slug: row
                        .slug
                        .ok_or_else(|| crate::error::invalid_relation("discipline slug"))?,
                },
                material_count: count,
            }),
            "teacher" => {
                let name = row
                    .name
                    .ok_or_else(|| crate::error::invalid_relation("responsible facet"))?;
                result.teachers.push(TeacherFacet {
                    teacher: TeacherReference {
                        id: TeacherId(
                            row.value
                                .ok_or_else(|| crate::error::invalid_relation("responsible id"))?,
                        ),
                        short_name: name.clone(),
                        name,
                    },
                    material_count: count,
                });
            }
            "course" | "year" => {
                let facet = ValueFacet {
                    value: row
                        .value
                        .ok_or_else(|| crate::error::invalid_relation("numeric facet"))?,
                    material_count: count,
                };
                if row.kind == "course" {
                    result.courses.push(facet);
                } else {
                    result.years.push(facet);
                }
            }
            _ => return Err(crate::error::unknown("facet kind", &row.kind)),
        }
    }
    Ok(result)
}
