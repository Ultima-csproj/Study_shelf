use serde::Serialize;

use crate::catalog::CatalogView;

use super::dto::facets::CatalogFacetsDto;
use super::dto::material::CatalogMaterialDto;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogResponse {
    items: Vec<CatalogMaterialDto>,
    total: u64,
    page: u32,
    page_count: u32,
    facets: CatalogFacetsDto,
}

impl From<CatalogView> for CatalogResponse {
    fn from(value: CatalogView) -> Self {
        Self {
            items: value.items.into_iter().map(Into::into).collect(),
            total: value.total,
            page: value.page,
            page_count: value.page_count,
            facets: value.facets.into(),
        }
    }
}
