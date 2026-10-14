use crate::api::catalog::dto::{material::MaterialSummaryDto, references::TeacherDto};
use serde::Serialize;
use study_core::catalog::MaterialSummary;
use study_core::learning::{LearningPage, LearningSection, SectionEntry, SectionPage};
use study_core::timestamp::TimestampError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PageDto<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_count: u32,
}
impl<T> PageDto<T> {
    pub fn map<U>(
        value: LearningPage<U>,
        mut map: impl FnMut(U) -> Result<T, TimestampError>,
    ) -> Result<Self, TimestampError> {
        Ok(Self {
            items: value
                .items
                .into_iter()
                .map(&mut map)
                .collect::<Result<_, _>>()?,
            total: value.total,
            page: value.page,
            page_count: value.page_count,
        })
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EntryDto {
    id: i64,
    material_id: i64,
    placement_id: Option<i64>,
    position: u64,
    responsible: Option<TeacherDto>,
    course: Option<u8>,
}
impl From<SectionEntry> for EntryDto {
    fn from(value: SectionEntry) -> Self {
        Self {
            id: value.id.0,
            material_id: value.material_id.0,
            placement_id: value.placement_id.map(|id| id.0),
            position: value.position,
            responsible: value.responsible.map(Into::into),
            course: value.course,
        }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SectionDto {
    id: i64,
    title: String,
    description: String,
    position: u64,
    step_count: u64,
    items: Vec<EntryDto>,
}
impl From<LearningSection> for SectionDto {
    fn from(value: LearningSection) -> Self {
        Self {
            id: value.id.0,
            title: value.title,
            description: value.description,
            position: value.position,
            step_count: value.step_count,
            items: value.items.into_iter().map(Into::into).collect(),
        }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DetailDto<T> {
    #[serde(flatten)]
    pub summary: T,
    pub sections: Vec<SectionDto>,
    pub materials: Vec<MaterialSummaryDto>,
    pub initial_page_size: u32,
}
impl<T> DetailDto<T> {
    pub fn new(
        summary: T,
        sections: Vec<LearningSection>,
        materials: Vec<MaterialSummary>,
    ) -> Result<Self, TimestampError> {
        Ok(Self {
            summary,
            sections: sections.into_iter().map(Into::into).collect(),
            materials: materials
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            initial_page_size: 10,
        })
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SectionPageDto {
    #[serde(flatten)]
    page: PageDto<EntryDto>,
    materials: Vec<MaterialSummaryDto>,
}
impl TryFrom<SectionPage> for SectionPageDto {
    type Error = TimestampError;
    fn try_from(value: SectionPage) -> Result<Self, Self::Error> {
        Ok(Self {
            page: PageDto::map(value.items, |item| Ok(item.into()))?,
            materials: value
                .materials
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}
