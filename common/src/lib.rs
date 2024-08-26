#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLists;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListDetails {
    pub list_id: [u8; 4],
    pub title: String,
    pub update_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lists(pub Vec<ListDetails>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
