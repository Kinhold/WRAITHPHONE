pub const SQLITE_SCHEMA_V1: &str = "CREATE TABLE IF NOT EXISTS manifests (id TEXT PRIMARY KEY);";

pub struct PersistentManifestStore;
pub struct TombstoneRecord;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_storage_scaffold() {
        assert_eq!(SQLITE_SCHEMA_V1.is_empty(), false);
    }
}
