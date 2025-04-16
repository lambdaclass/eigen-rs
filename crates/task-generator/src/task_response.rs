use alloy::primitives::B256;
use eigen_types::avs::TaskIndex;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TaskResponse<OUTPUT> {
    #[allow(missing_docs)]
    pub referenceTaskIndex: u32,
    #[allow(missing_docs)]
    pub respose: OUTPUT,
}
