use super::dto::facets::CatalogFacetsDto;
use super::dto::material::MaterialSummaryDto;
use crate::catalog::CatalogView;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogResponse {
    items: Vec<MaterialSummaryDto>,
    total: u64,
    page: u32,
    page_count: u32,
    facets: CatalogFacetsDto,
}
impl TryFrom<CatalogView> for CatalogResponse {
    type Error = study_core::timestamp::TimestampError;
    fn try_from(value: CatalogView) -> Result<Self, Self::Error> {
        Ok(Self {
            items: value
                .items
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            total: value.total,
            page: value.page,
            page_count: value.page_count,
            facets: value.facets.into(),
        })
    }
}
