pub mod contract;

#[cfg(target_arch = "wasm32")]
mod entry_points {
    use super::contract;
    use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Binary, Deps};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: contract::InstantiateMsg,
    ) -> StdResult<Response> {
        contract::instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: contract::ExecuteMsg,
    ) -> StdResult<Response> {
        contract::execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(
        deps: Deps,
        env: Env,
        msg: contract::QueryMsg,
    ) -> StdResult<Binary> {
        contract::query(deps, env, msg)
    }
}
