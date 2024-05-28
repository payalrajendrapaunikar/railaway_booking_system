


use chrono::{Datelike, NaiveDate};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError, StdResult};

use crate::msg::QueryMsg;

pub fn query_msg(
    deps:Deps,
    _env : Env,
    msg : QueryMsg
)-> StdResult<Binary>{

    use QueryMsg::*;

     match msg {
         
         GetAllTrain {  }=> to_json_binary(&query_msg_impl::get_all_train(deps)?),
         GetServiceDays {  } => to_json_binary(&query_msg_impl::get_service_days(deps)?),
         GetTrainsUsingOriginAndFinalDestinationWithDateFortravelling { 
            origin_destination, 
            final_destination, 
            travelling_date }
            => to_json_binary(&query_msg_impl
                ::get_train_by_orgin_to_final_destination(
                    deps, 
                    origin_destination,
                     final_destination, travelling_date)?),
            GetAllTicketTypes {  }
            => to_json_binary(&query_msg_impl::get_all_ticket_type(deps)?), 
            GetTicketPrice { ticket_type_id,total_diatance_in_km }
            => to_json_binary(&query_msg_impl::calculate_ticket_price(ticket_type_id, total_diatance_in_km, deps)?),
            GetBookedTicket { train_id, travelling_date, ticket_id }
            => to_json_binary(&query_msg_impl::get_booked_ticket(deps, train_id, travelling_date, ticket_id)?)        
     }
}


mod query_msg_impl{
  

    use cosmwasm_std::{ Coin, Deps, StdError, StdResult, Uint128};

    use crate::state::{BookingTicket, ServicesDaysInContract, TicketType, Train, TRAIN_STATE, PRICE_PER_KM, SERVICE_DAYS, TICKET_TYPES, TICKETS_STATE};

    use super::convert_date_into_day;


    pub fn get_all_train(deps:Deps)->StdResult<Vec<Train>>{

       let trains = TRAIN_STATE.load(deps.storage)?;

       Ok(trains)
    }

    pub fn get_train_by_orgin_to_final_destination(
        deps:Deps,
        origin_destination:String,
        final_destination : String,
        date_for_travelling:String
    )-> StdResult<Vec<Train>>{


        let trains = TRAIN_STATE.load(deps.storage)?;

        let travelling_day = convert_date_into_day(date_for_travelling)?;

        println!("exiting trains are : {:#?}",trains);

        let mut our_destinations_trains : Vec<Train> = Vec::new();

        for get_train in trains  {
            
            if get_train.origin_stop == origin_destination && get_train.destination_stop == final_destination {
                    println!("origin and final stops match");
                    for day in get_train.service_days.clone(){

                        println!("{} service day is {}",get_train.train_id,day);
             
                        if day.to_string() == travelling_day {
                            

                            println!("the train travel on this day {}",travelling_day);  
                            our_destinations_trains.push(get_train.clone());
                            break;
                        }
                     }               
            } 

               
        }

       Ok(our_destinations_trains)



    }

    pub fn get_service_days(deps:Deps)->StdResult<ServicesDaysInContract>{

        println!("inside the get services days query");

        let service_days = match SERVICE_DAYS.load(deps.storage) {
            Ok(service_days) => Ok(service_days),
            Err(err) => Err(StdError::generic_err(format!("Error loading services: {}", err))),
        };

       // println!("service days is {:#?}",service_days);

       service_days
      
    }


    pub fn get_all_ticket_type(deps:Deps)-> StdResult<Vec<TicketType>>{

        let ticket_types = TICKET_TYPES.load(deps.storage)?;

        Ok(ticket_types)
    }

     pub fn calculate_ticket_price(ticket_type_id : u64 , 
        total_diatance_in_km : u64, deps:Deps)->StdResult<Coin>{

        let ticket_types = TICKET_TYPES.load(deps.storage)?;

         let get_ticket_type = ticket_types
                                .iter()
                                .find(|ticket_type|ticket_type.ticket_type_id == ticket_type_id)
                                .unwrap();


          let get_price_per_km = PRICE_PER_KM.load(deps.storage)?;

          let total_distance = Uint128::from(total_diatance_in_km);

          let total_km_price = get_price_per_km.amount.checked_mul(total_distance)?;

          let extra_charge = Uint128::from(get_ticket_type.extra_charges);

          let total_ticket_price = total_km_price.checked_add(extra_charge)?;


          println!("ticket price is : {}",total_ticket_price);

          Ok(Coin { 
            denom: get_price_per_km.denom,
             amount: total_ticket_price })
                
                            
     }


     pub fn get_booked_ticket(
        deps:Deps,
        train_id : u64, 
        travelling_date : String,
        ticket_id : String
    )->StdResult<BookingTicket>{

        let booked_ticket_key = format!("{}{}",travelling_date.clone(),train_id.clone());

        let booked_ticket = TICKETS_STATE.load(deps.storage, booked_ticket_key)?;

        println!("get booked ticket at query booked ticket : {:#?}",booked_ticket);

        let ticket =  booked_ticket
                             .iter()
                            .find(|ticket|ticket.ticket_id == ticket_id);                

        match ticket {

            Some(get_ticket)=>Ok(get_ticket.to_owned()),
            None => Err(StdError::generic_err("ticket details is not correct"))
            
        }                 
                            



     }

    
}



pub fn convert_date_into_day( travelling_date :String)->Result<String, StdError>{

    println!("our travelling date is : {}",travelling_date.clone());

    let parsed_date = match NaiveDate::parse_from_str(&travelling_date, "%d/%m/%Y") {
        Ok(parsed_date) => parsed_date,
        Err(err) => {
            // Handle the error here (e.g., log it, return a specific error message)
            println!("Error parsing date: {}", err);
            return Err(StdError::generic_err("Error during date parsing")); // Or return a custom error
        }
    };

    println!("our parsed date is : {}",parsed_date.clone());

    let day_of_week = parsed_date.weekday().num_days_from_monday();

    let day_names = ["Monday".to_owned(), "Tuesday".to_owned(),
     "Wednesday".to_owned(), "Thursday".to_owned(), "Friday".to_owned(), 
     "Saturday".to_owned(), "Sunday".to_owned()];

     println!("the value index of day of week is : {}",day_of_week.clone());

     let day_name = day_names[day_of_week as usize].clone();

     Ok(day_name)

}





