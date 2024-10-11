use std::fmt::Debug;

use alloy::primitives::{Address, U256, U64};
use alloy::providers::fillers::{
    ChainIdFiller, GasFiller, JoinFill, NonceFiller, RecommendedFillers,
};
use alloy::providers::{Provider, ProviderCall};
use alloy::rpc::client::NoParams;
use alloy::transports::{BoxTransport, Transport};
use serde::{Deserialize, Serialize};

use crate::network::Zksync;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    pub gas_limit: U256,
    pub gas_per_pubdata_limit: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait ZksyncProvider<T = BoxTransport>: Provider<T, Zksync>
where
    T: Transport + Clone,
{
    /// Gets the address of the main ZKsync contract on L1.
    fn get_main_contract(&self) -> ProviderCall<T, NoParams, Address> {
        self.client().request_noparams("zks_getMainContract").into()
    }

    /// Gets the address of the testnet paymaster ZKsync contract on L2, if it's present on the network.
    fn get_testnet_paymaster(&self) -> ProviderCall<T, NoParams, Address> {
        self.client()
            .request_noparams("zks_getTestnetPaymaster")
            .into()
    }

    /// Gets the L1 Chain ID
    fn get_l1_chain_id(&self) -> ProviderCall<T, NoParams, U64> {
        self.client().request_noparams("zks_L1ChainId").into()
    }

    /// Gets the L1 batch number.
    fn get_l1_batch_number(&self) -> ProviderCall<T, NoParams, U64> {
        self.client().request_noparams("zks_L1BatchNumber").into()
    }

    /// Estimate transaction gas.
    fn estimate_fee<R: Serialize + Clone + Debug + Send + Sync + Unpin>(
        &self,
        tx: R,
    ) -> ProviderCall<T, (R,), Option<Fee>> {
        self.client().request("zks_estimateFee", (tx,)).into()
    }
}

impl<P, T> ZksyncProvider<T> for P
where
    T: Transport + Clone,
    P: Provider<T, Zksync>,
{
}

impl RecommendedFillers for Zksync {
    type RecomendedFillters = JoinFill<GasFiller, JoinFill<NonceFiller, ChainIdFiller>>;

    fn recommended_fillers() -> Self::RecomendedFillters {
        JoinFill::new(
            GasFiller,
            JoinFill::new(NonceFiller::default(), ChainIdFiller::default()),
        )
    }
}
