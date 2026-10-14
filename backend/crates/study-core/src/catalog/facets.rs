use super::{DisciplineReference, MaterialType, TeacherReference};

#[derive(Debug, Clone)]
pub struct TeacherFacet {
    pub teacher: TeacherReference,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct DisciplineFacet {
    pub discipline: DisciplineReference,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct ValueFacet {
    pub value: i64,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct TypeFacet {
    pub material_type: MaterialType,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct CatalogFacets {
    pub types: Vec<TypeFacet>,
    pub disciplines: Vec<DisciplineFacet>,
    pub teachers: Vec<TeacherFacet>,
    pub courses: Vec<ValueFacet>,
    pub years: Vec<ValueFacet>,
}
