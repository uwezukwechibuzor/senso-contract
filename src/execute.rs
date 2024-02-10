use cw_ownable::OwnershipError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{CustomMsg, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use cw721::ContractInfoResponse;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::Cw721Contract;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:senso-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<C>, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract_info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        self.contract_info.save(deps.storage, &contract_info)?;

        let owner = match msg.minter {
            Some(owner) => deps.api.addr_validate(&owner)?,
            None => info.sender,
        };
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner.as_ref()))?;

        // set mint amount
        self.mint_amount.save(deps.storage, &msg.mint_amount)?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        //msg: ExecuteMsg<T, E>,
    ) -> Result<Response<C>, ContractError> {
        //unimplemented()
    }
}
