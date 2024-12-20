pub trait Storage: Send + Sync {
    fn id(&self) -> String;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Storage;

    pub const TEST_STORAGE_ID: &str = "test";

    #[derive(Default)]
    pub struct TestStorage {}

    impl Storage for TestStorage {
        fn id(&self) -> String {
            TEST_STORAGE_ID.to_owned()
        }
    }
}
