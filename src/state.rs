use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

use cosmwasm_std::CustomMsg;

use cw721::{ContractInfoResponse, Cw721};
use cw_storage_plus::Item;

pub struct Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomMsg,
    E: CustomMsg,
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub mint_amount: Item<'a, u64>,

    pub(crate) _custom_response: PhantomData<C>,
    pub(crate) _custom_query: PhantomData<Q>,
    pub(crate) _custom_execute: PhantomData<E>,
}

// This is a signal, the implementations are in other files
impl<'a, T, C, E, Q> Cw721<T, C> for Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
}

impl<T, C, E, Q> Default for Cw721Contract<'static, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn default() -> Self {
        Self::new("nft_info", "mint_amount")
    }
}

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn new(contract_key: &'a str, mint_amount_key: &'a u64) -> Self {
        Self {
            contract_info: Item::new(contract_key),
            mint_amount: Item::new(mint_amount_key),
            _custom_response: PhantomData,
            _custom_execute: PhantomData,
            _custom_query: PhantomData,
        }
    }
}
