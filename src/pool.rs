use cosmwasm_std::CustomQuery;
use cosmwasm_std::{CosmosMsg, Deps, MessageInfo};
use cw_asset::Asset;
use serde::{de::DeserializeOwned, Serialize};

use crate::CwDexError;

pub trait Pool<Q: CustomQuery, A = Asset>: Clone + Serialize + DeserializeOwned {
    fn provide_liquidity(
        &self,
        deps: Deps<Q>,
        info: &MessageInfo,
        assets: Vec<A>,
    ) -> Result<CosmosMsg, CwDexError>;
    fn withdraw_liquidity(
        &self,
        deps: Deps<Q>,
        info: &MessageInfo,
        asset: A,
        asset_to_withdraw: Option<A>,
    ) -> Result<CosmosMsg, CwDexError>;
    fn swap(&self, info: &MessageInfo, offer: A, ask: A) -> Result<CosmosMsg, CwDexError>;
    fn get_pool_assets(&self) -> Result<Vec<A>, CwDexError>;
    fn simulate_provide_liquidity(&self, deps: Deps<Q>, asset: Vec<A>) -> Result<A, CwDexError>;
    fn simulate_withdraw_liquidity(
        &self,
        deps: Deps<Q>,
        asset: A,
        asset_to_withdraw: Option<A>,
    ) -> Result<Vec<A>, CwDexError>;
}
