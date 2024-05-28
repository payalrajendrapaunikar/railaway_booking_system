
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult
};


use crate::instatiate_contract::instantiate_msg;

use crate::msg::{InstatiateMsg,ExcuteMsg,QueryMsg};

use crate::execute_contract::execute_msg;

use crate::query_contract::query_msg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstatiateMsg,
) -> StdResult<Response> {
   
   instantiate_msg(deps, env, info, msg)
   
}

#[entry_point]
pub fn execute(
    deps:DepsMut,
    env:Env,
    info:MessageInfo,
    msg:ExcuteMsg,
)-> Result<Response,StdError>{

    execute_msg(deps, env, info, msg)
}

#[entry_point]
pub fn query(
    deps:Deps,
    env : Env,
    msg : QueryMsg
)-> StdResult<Binary>{

    query_msg(deps, env, msg)
}