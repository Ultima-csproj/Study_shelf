use std::collections::BTreeMap;

use serde::Serialize;
use study_storage_sqlite::catalog::{CatalogFacets, ValueFacet};

use super::references::{DisciplineFacetDto, TeacherFacetDto};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueFacetDto {
    value: i64,
    material_count: i64,
}

impl From<ValueFacet> for ValueFacetDto {
    fn from(value: ValueFacet) -> Self {
        Self {
            value: value.value,
            material_count: value.material_count,
        }
    }
}

#[derive(Serialize)]
pub struct CatalogFacetsDto {
    types: BTreeMap<String, i64>,
    disciplines: Vec<DisciplineFacetDto>,
    teachers: Vec<TeacherFacetDto>,
    courses: Vec<ValueFacetDto>,
    years: Vec<ValueFacetDto>,
}

impl From<CatalogFacets> for CatalogFacetsDto {
    fn from(value: CatalogFacets) -> Self {
        let types = value
            .types
            .into_iter()
            .map(|facet| {
                (
                    facet.material_type.as_db_value().to_owned(),
                    facet.material_count,
                )
            })
            .collect();

        Self {
            types,
            disciplines: value.disciplines.into_iter().map(Into::into).collect(),
            teachers: value.teachers.into_iter().map(Into::into).collect(),
            courses: value.courses.into_iter().map(Into::into).collect(),
            years: value.years.into_iter().map(Into::into).collect(),
        }
    }
}
