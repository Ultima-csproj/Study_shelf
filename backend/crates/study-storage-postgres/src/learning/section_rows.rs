use sqlx::FromRow;
use study_core::catalog::TeacherReference;
use study_core::error::RepositoryResult;
use study_core::ids::{MaterialId, PlacementId, SectionEntryId, TeacherId};
use study_core::learning::{SectionEntry, SectionScope};

pub(crate) struct ScopeSql {
    pub parent: i64,
    pub sections: &'static str,
    pub entries: &'static str,
    pub relation: &'static str,
    pub parent_column: &'static str,
    pub material: &'static str,
    pub placement: &'static str,
    pub person_join: &'static str,
    pub responsible: &'static str,
    pub course: &'static str,
}
impl ScopeSql {
    pub fn new(scope: SectionScope) -> Self {
        match scope {
            SectionScope::Discipline(id) => Self {
                parent: id.0,
                sections: "discipline_sections",
                entries: "discipline_section_entries",
                relation: "JOIN discipline_placements p
                    ON p.id = e.placement_id",
                parent_column: "discipline_id",
                material: "p.material_id",
                placement: "p.id",
                person_join: "LEFT JOIN users u
                    ON u.id = p.responsible_user_id",
                responsible: "p.responsible_user_id",
                course: "p.course",
            },
            SectionScope::Collection(id) => Self {
                parent: id.0,
                sections: "collection_sections",
                entries: "collection_entries",
                relation: "",
                parent_column: "collection_id",
                material: "e.material_id",
                placement: "NULL::bigint",
                person_join: "LEFT JOIN users u
                    ON FALSE",
                responsible: "NULL::bigint",
                course: "NULL::smallint",
            },
        }
    }
    /// All inserted identifiers come from the closed SectionScope above.
    pub fn visible_entries(&self) -> String {
        format!(
            "FROM {} e {}
            JOIN public_materials m
                ON m.id = {} {}
            WHERE e.{} = $1",
            self.entries, self.relation, self.material, self.person_join, self.parent_column
        )
    }
}

#[derive(FromRow)]
pub(super) struct EntryRow {
    pub section_id: i64,
    id: i64,
    material_id: i64,
    placement_id: Option<i64>,
    position: i64,
    responsible_id: Option<i64>,
    responsible_name: Option<String>,
    course: Option<i16>,
}
impl EntryRow {
    pub fn into_entry(self) -> RepositoryResult<SectionEntry> {
        if self.id <= 0
            || self.section_id <= 0
            || self.material_id <= 0
            || self.placement_id.is_some_and(|id| id <= 0)
            || self.position <= 0
        {
            return Err(crate::error::invalid_relation("section entry identity"));
        }
        let responsible = match (self.responsible_id, self.responsible_name) {
            (Some(id), Some(name)) => Some(TeacherReference {
                id: TeacherId(id),
                short_name: name.clone(),
                name,
            }),
            (None, None) => None,
            _ => return Err(crate::error::invalid_relation("section responsible")),
        };
        Ok(SectionEntry {
            id: SectionEntryId(self.id),
            material_id: MaterialId(self.material_id),
            placement_id: self.placement_id.map(PlacementId),
            position: super::count(self.position)?,
            responsible,
            course: self.course.map(super::course).transpose()?,
        })
    }
}
