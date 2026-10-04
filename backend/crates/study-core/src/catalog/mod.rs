mod facets;
mod material;
mod page;
mod query;
mod references;
mod repository;
mod text;

pub use facets::{CatalogFacets, DisciplineFacet, TeacherFacet, TypeFacet, ValueFacet};

pub use material::{EducationalPlacement, MaterialSummary, MaterialType};
pub use page::CatalogPage;
pub use query::{CatalogQuery, CatalogQueryError, CatalogSort};

pub use references::{DisciplineReference, TeacherReference};

pub use repository::CatalogRepository;
pub use text::normalize_search_text;
