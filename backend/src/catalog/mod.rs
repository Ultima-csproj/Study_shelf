mod models;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::{Context, Result};

use study_core::catalog::{CatalogQuery, CatalogRepository, Material};

use study_core::ids::{DisciplineId, MaterialId, TeacherId};

pub use models::{CatalogItem, CatalogView};

pub struct CatalogService {
    repository: Arc<dyn CatalogRepository>,
}

impl CatalogService {
    pub fn new<R>(repository: R) -> Self
    where
        R: CatalogRepository + 'static,
    {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn list(&self, query: CatalogQuery) -> Result<CatalogView> {
        let page = self.repository.search(query).await?;
        let items = self.enrich(page.items).await?;
        let facets = self.repository.facets().await?;

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

        let mut items = self.enrich(vec![material]).await?;

        Ok(items.pop())
    }

    async fn enrich(&self, materials: Vec<Material>) -> Result<Vec<CatalogItem>> {
        if materials.is_empty() {
            return Ok(Vec::new());
        }

        let discipline_ids: Vec<DisciplineId> = materials
            .iter()
            .map(|material| material.discipline_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let teacher_ids: Vec<TeacherId> = materials
            .iter()
            .map(|material| material.teacher_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let disciplines = self.repository.disciplines(&discipline_ids).await?;

        let teachers = self.repository.teachers(&teacher_ids).await?;

        let disciplines: HashMap<_, _> = disciplines
            .into_iter()
            .map(|discipline| (discipline.id, discipline))
            .collect();

        let teachers: HashMap<_, _> = teachers
            .into_iter()
            .map(|teacher| (teacher.id, teacher))
            .collect();

        let mut items = Vec::with_capacity(materials.len());

        for material in materials {
            let discipline = disciplines
                .get(&material.discipline_id)
                .with_context(|| format!("discipline {} is missing", material.discipline_id.0,))?
                .clone();

            let teacher = teachers
                .get(&material.teacher_id)
                .with_context(|| format!("teacher {} is missing", material.teacher_id.0,))?
                .clone();

            items.push(CatalogItem {
                material,
                discipline,
                teacher,
            });
        }

        Ok(items)
    }
}
