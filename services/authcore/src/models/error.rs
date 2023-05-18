#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("database error")]
    DatabaseError(#[from] prisma_client_rust::QueryError),
    #[error("not found")]
    RecordNotFound,
}
