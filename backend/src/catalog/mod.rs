use std::collections::HashMap;

use anyhow::{Context, Result};
use study_core::catalog::{CatalogQuery, Material, MaterialRepository};
use study_core::ids::MaterialId;
use study_storage_sqlite::catalog::{
    CatalogFacets, DisciplineFacet, SqliteCatalogRepository, TeacherFacet,
};

pub struct CatalogItem {
    pub material: Material,
    pub discipline: DisciplineFacet,
    pub teacher: TeacherFacet,
}

pub struct CatalogView {
    pub items: Vec<CatalogItem>,
    pub total: u64,
    pub page: u32,
    pub page_count: u32,
    pub facets: CatalogFacets,
}

pub struct CatalogService {
    repository: SqliteCatalogRepository,
}

impl CatalogService {
    pub fn new(repository: SqliteCatalogRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self, query: CatalogQuery) -> Result<CatalogView> {
        let page = self.repository.search(query).await?;
        let facets = self.repository.facets().await?;
        let disciplines: HashMap<i64, DisciplineFacet> = facets
            .disciplines
            .iter()
            .map(|discipline| (discipline.id, discipline.clone()))
            .collect();
        let teachers: HashMap<i64, TeacherFacet> = facets
            .teachers
            .iter()
            .map(|teacher| (teacher.id, teacher.clone()))
            .collect();
        let items = page
            .items
            .into_iter()
            .map(|material| -> Result<CatalogItem> {
                let discipline = disciplines
                    .get(&material.discipline_id.0)
                    .with_context(|| format!("discipline {} is missing", material.discipline_id.0))?
                    .clone();
                let teacher = teachers
                    .get(&material.teacher_id.0)
                    .with_context(|| format!("teacher {} is missing", material.teacher_id.0))?
                    .clone();
                Ok(CatalogItem {
                    material,
                    discipline,
                    teacher,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(CatalogView {
            items,
            total: page.total,
            page: page.page,
            page_count: page.page_count,
            facets,
        })
    }

    pub async fn get_by_id(&self, id: MaterialId) -> Result<Option<CatalogItem>> {
        let Some(material) = self.repository.get_by_id(id).await? else {
            return Ok(None);
        };

        let discipline = self
            .repository
            .discipline(material.discipline_id)
            .await?
            .with_context(|| format!("discipline {} is missing", material.discipline_id.0))?;

        let teacher = self
            .repository
            .teacher(material.teacher_id)
            .await?
            .with_context(|| format!("teacher {} is missing", material.teacher_id.0))?;

        Ok(Some(CatalogItem {
            material,
            discipline,
            teacher,
        }))
    }
}
