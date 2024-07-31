use alloy_consensus::{TxEip1559, TxLegacy};
use alloy_network::{Ethereum, EthereumWallet, TxSigner};
use alloy_primitives::Address;
use alloy_provider::{PendingTransactionBuilder, Provider, RootProvider};
use alloy_rpc_types_eth::TransactionReceipt;
use alloy_signer_local::PrivateKeySigner;
use eigen_signer::signer::Config;
use thiserror::Error;

pub type Transport = alloy_transport_http::Http<reqwest::Client>;

/// Possible errors raised in Tx Manager
#[derive(Error, Debug)]
pub enum TxManagerError {
    #[error("signer error")]
    SignerError,
    #[error("send error")]
    SendTxError,
    #[error("wait_for_receipt error")]
    WaitForReceiptError,
}

pub struct SimpleTxManager {
    wallet: EthereumWallet,
    //    client: eth::Client,
    //log: logging::Logger,
    sender: Address,
    gas_limit_multiplier: f64,
    private_key: String,
    provider: RootProvider<Transport>,
}

impl SimpleTxManager {
    /*
    pub fn new(
        wallet: wallet::Wallet,
        client: eth::Client,
        log: logging::Logger,
        sender: common::Address,
        gas_limit_multiplier: f64,
    ) -> SimpleTxManager {
        SimpleTxManager {
            wallet,
            client,
            log,
            sender,
            gas_limit_multiplier,
        }
    }
    */

    pub fn with_gas_limit_multiplier(&mut self, multiplier: f64) {
        self.gas_limit_multiplier = multiplier;
    }

    fn create_local_signer(&self) -> Result<PrivateKeySigner, TxManagerError> {
        let config = Config::PrivateKey(self.private_key.clone());
        Config::signer_from_config(config).map_err(|_| TxManagerError::SignerError)
    }

    /// Sends a EIP1559 transaction.
    ///
    /// Send is used to send a transaction to the Ethereum node. It takes an unsigned/signed transaction
    /// and then sends it to the Ethereum node.
    /// If you pass in a signed transaction it will ignore the signature
    /// and resign the transaction after adding the nonce and gas limit.
    ///
    /// # Arguments
    ///
    /// - `tx`: The transaction to be sent.
    ///
    /// # Returns
    ///
    /// - The transaction receipt.
    pub async fn send_eip1559_tx(
        &self,
        tx: &mut TxEip1559,
    ) -> Result<TransactionReceipt, TxManagerError> {
        let signer = self.create_local_signer()?;
        let _signed_tx = signer
            .sign_transaction(tx)
            .await
            .map_err(|_| TxManagerError::SignerError)?;

        // send transaction and get receipt
        let pending_tx = self
            .provider
            .send_transaction(tx.clone().into())
            .await
            .map_err(|_| TxManagerError::SendTxError)?;

        // wait for the transaction to be mined
        SimpleTxManager::wait_for_receipt(pending_tx).await
    }

    /// Send is used to send a transaction to the Ethereum node. It takes an unsigned/signed transaction
    /// and then sends it to the Ethereum node.
    /// If you pass in a signed transaction it will ignore the signature
    /// and resign the transaction after adding the nonce and gas limit.
    ///
    /// # Arguments
    ///
    /// - `tx`: The transaction to be sent.
    ///
    /// # Returns
    ///
    /// - The transaction receipt.
    pub async fn send_legacy_tx(
        &self,
        tx: &mut TxLegacy,
    ) -> Result<TransactionReceipt, TxManagerError> {
        // TODO: It also takes care of gas estimation and adds a buffer to the gas limit
        // TODO: Estimating gas and nonce
        //m.log.Debug("Estimating gas and nonce")
        //tx, err := m.estimateGasAndNonce(ctx, tx)
        let signer = self.create_local_signer()?;
        let _signed_tx = signer
            .sign_transaction(tx)
            .await
            .map_err(|_| TxManagerError::SignerError)?;

        // send transaction and get receipt
        let pending_tx = self
            .provider
            .send_transaction(tx.clone().into())
            .await
            .map_err(|_| TxManagerError::SendTxError)?;

        // wait for the transaction to be mined
        SimpleTxManager::wait_for_receipt(pending_tx).await
    }

    /// Waits for the transaction receipt.
    ///
    /// This is a wrapper around `PendingTransactionBuilder::get_receipt`.
    ///
    /// # Arguments
    ///
    /// - `pending_tx`: The pending transaction builder we want to wait for.
    ///
    /// # Returns
    ///
    /// - The block number in which the transaction was included.
    /// - `None` if the transaction was not included in a block or an error ocurred.
    pub async fn wait_for_receipt(
        pending_tx: PendingTransactionBuilder<'_, Transport, Ethereum>,
    ) -> Result<TransactionReceipt, TxManagerError> {
        pending_tx
            .get_receipt()
            .await
            .map_err(|_| TxManagerError::WaitForReceiptError)
    }

    /*
    // GetNoSendTxOpts This generates a noSend TransactOpts so that we can use
    // this to generate the transaction without actually sending it
    func (m *SimpleTxManager) GetNoSendTxOpts() (*bind.TransactOpts, error) {
    }

    func (m *SimpleTxManager) queryReceipt(ctx context.Context, txID wallet.TxID) *types.Receipt {
    }

    // estimateGasAndNonce we are explicitly implementing this because
    // * We want to support legacy transactions (i.e. not dynamic fee)
    // * We want to support gas management, i.e. add buffer to gas limit
    func (m *SimpleTxManager) estimateGasAndNonce(ctx context.Context, tx *types.Transaction) (*types.Transaction, error) {
    }
    */
}

#[cfg(test)]
mod tests {
    use super::SimpleTxManager;
    use alloy_consensus::TxLegacy;
    use alloy_network::TxSigner;
    use alloy_node_bindings::Anvil;
    use alloy_primitives::{bytes, TxKind::Call, U256};
    use alloy_provider::{Provider, ProviderBuilder};
    use eigen_signer::signer::Config;
    use tokio;

    const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

    #[tokio::test]
    async fn test_send_signed_transaction() {
        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        let anvil = Anvil::new().try_spawn().unwrap();

        // Create a provider.
        let rpc_url = anvil.endpoint().parse().unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url);

        // Create two users, Alice and Bob.
        let _alice = anvil.addresses()[0];
        let bob = anvil.addresses()[1];

        let config = Config::PrivateKey(PRIVATE_KEY.into());
        let signer = Config::signer_from_config(config).unwrap();

        let mut tx = TxLegacy {
            to: Call(bob),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };
        let _signed_tx = signer.sign_transaction(&mut tx).await.unwrap();

        // send transaction and get receipt
        let pending_tx = provider.send_transaction(tx.into()).await.unwrap();

        // wait for the transaction to be mined
        let receipt = SimpleTxManager::wait_for_receipt(pending_tx).await.unwrap();
        let block_number = receipt.block_number.unwrap();
        println!("Transaction mined in block: {}", block_number);
        assert!(block_number > 0);
        assert_eq!(receipt.to, Some(bob));
    }
}
