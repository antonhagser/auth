#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("database error")]
    DatabaseError(#[from] prisma_client_rust::QueryError),

    #[error("not found")]
    NotFound,

    #[error("missing field in builder")]
    MissingField(String),
}
