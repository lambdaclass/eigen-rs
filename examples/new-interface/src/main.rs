#![allow(missing_docs)]

use alloy::primitives::{address, U256};
use bindings::iincrediblesquaringtaskmanager::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{Task, TaskResponse},
    IncredibleSquaringTaskManager::{self, IncredibleSquaringTaskManagerInstance},
    BN254::{G1Point, G2Point},
};
use eigen_common::get_provider;
use eigen_task_generator::{
    indexing_task_processor::IndexingTaskProcessor, task_manager_contract::TaskManagerContract,
};

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;

type ContractInstance = IncredibleSquaringTaskManagerInstance<
    (),
    alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy::providers::RootProvider,
    >,
>;

struct ContractImpl {
    contract: ContractInstance,
}

impl ContractImpl {
    fn new(contract: ContractInstance) -> Self {
        Self { contract }
    }
}

impl TaskManagerContract<Task, TaskResponse, NonSignerStakesAndSignature> for ContractImpl {
    fn create_new_task(&self, _task: Task) {
        todo!();
    }

    fn respond_to_task(
        &self,
        task: Task,
        response: TaskResponse,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) {
        self.contract
            .respondToTask(task, response, non_signer_stakes_and_signature);
    }
}

#[tokio::main]
async fn main() {
    let http_rpc_url = "http://localhost:8545".to_string();
    let contract = IncredibleSquaringTaskManager::new(
        address!("0x7bc06c482dead17c0e297afbc32f6e63d3846650"),
        get_provider(&http_rpc_url),
    );

    let contract_impl = ContractImpl::new(contract);
    let task = Task {
        numberToBeSquared: U256::from(10),
        quorumNumbers: vec![1].into(),
        quorumThresholdPercentage: 0,
        taskCreatedBlock: 0,
    };
    let response = TaskResponse {
        numberSquared: U256::from(100),
        referenceTaskIndex: 1,
    };
    let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
        nonSignerQuorumBitmapIndices: vec![1],
        nonSignerPubkeys: vec![G1Point {
            X: U256::from(1),
            Y: U256::from(1),
        }],
        quorumApks: vec![G1Point {
            X: U256::from(1),
            Y: U256::from(1),
        }],
        apkG2: G2Point {
            X: [U256::from(1), U256::from(1)],
            Y: [U256::from(1), U256::from(1)],
        },
        sigma: G1Point {
            X: U256::from(1),
            Y: U256::from(1),
        },
        quorumApkIndices: vec![1],
        totalStakeIndices: vec![1],
        nonSignerStakeIndices: vec![vec![1]],
    };

    let indexing = IndexingTaskProcessor::<
        Task,
        TaskResponse,
        NonSignerStakesAndSignature,
        U256,
        U256,
        ContractImpl,
    >::new(contract_impl);
}
