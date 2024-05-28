

use cosmwasm_std::{
    Coin, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};

use crate::{msg::InstatiateMsg, state::{ServicesDays, ServicesDaysInContract, TicketType, Train, PRICE_PER_KM, SERVICE_DAYS, TICKET_TYPES, TICKET_TYPES_COUNT, TRAIN_COUNT}};

use cw721_base:: {InstantiateMsg as CW721InstantiateMsg,Cw721Contract};


pub fn instantiate_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstatiateMsg,
) -> StdResult<Response> {

  
  let cw721_instatiate_msg = CW721InstantiateMsg{
    name : msg.nft_name,
    symbol : msg.nft_symbol,
    minter : env.contract.address.to_string(),// here i set contract as minter
  };

  let train_count :u64 = 0;
   TRAIN_COUNT.save(deps.storage, &train_count)?;

   let price_per_km = Coin::new(4, "eth");
   PRICE_PER_KM.save(deps.storage, &price_per_km)?;

   let service_days = ServicesDaysInContract{
    
    service_days : vec![
      ServicesDays::Monday,
      ServicesDays::Tuesday,
      ServicesDays::Wednesday,
      ServicesDays::Thursday,
      ServicesDays::Friday,
      ServicesDays::Saturday,
      ServicesDays::Sunday,
    ]
   };

   SERVICE_DAYS.save(deps.storage, &service_days)?;

   let ticket_type : Vec<TicketType>= vec![];   

   TICKET_TYPES.save(deps.storage,&ticket_type)?;

   let ticket_types_count = 0;

    TICKET_TYPES_COUNT.save(deps.storage, &ticket_types_count)?;

     

  // Instantiate the CW721 contract using the cw721_instantiate_msg
  let contract: Cw721Contract<Train, Empty, Empty, Empty> = Cw721Contract::default();
  contract.instantiate(deps, env, info, cw721_instatiate_msg)

    
}


