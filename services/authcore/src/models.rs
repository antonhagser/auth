#[rustfmt::skip]
#[allow(unused_imports, dead_code, clippy::all)]
mod prisma;

#[rustfmt::skip]
#[allow(unused_imports, dead_code, clippy::all)]
pub use prisma::PrismaClient;

pub mod application;
pub mod error;
pub mod user;
