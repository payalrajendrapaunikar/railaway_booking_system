
use chrono::{NaiveDate, NaiveDateTime};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};

use crate::{msg::ExcuteMsg, state::Train};



pub fn execute_msg(
  deps:DepsMut,
  env:Env,
  info:MessageInfo,
  msg:ExcuteMsg,
)->Result<Response,StdError>{

      use ExcuteMsg::*;
      
      match msg {

        AddTrain { train_name, train_no, 
          total_no_seat, origin_stop,
           destination_stop ,
           origin_stop_departure_time_with_basic_date_eg,
           destination_stop_arrival_time_with_basic_date_eg,
           total_distance_in_km,
           service_days,
           ticket_type_with_total_seat_for_ticket_type,
           
        }
           => execute_msg_impl::add_train(deps, info, train_name, train_no, total_no_seat,
             origin_stop, destination_stop,origin_stop_departure_time_with_basic_date_eg,
             destination_stop_arrival_time_with_basic_date_eg,total_distance_in_km,
             service_days,ticket_type_with_total_seat_for_ticket_type
            ),
         AddTicketType { ticket_type, extra_charges } 
         => execute_msg_impl::add_ticket_type(deps, ticket_type, extra_charges),
         RemoveTicketType { ticket_type_id }
         => execute_msg_impl::remove_ticket_type(deps, ticket_type_id),
         BookedTicket { first_name, last_name,
           mobile_no, age, date_of_birth, 
           nationality, 
             train_id, 
            travelling_date,ticket_type_id}  
           => execute_msg_impl::booked_ticket(deps, info, env,first_name, last_name, mobile_no, age, date_of_birth, nationality, train_id, 
            travelling_date,ticket_type_id)
      }
}

mod execute_msg_impl{

    use cosmwasm_std::{Empty, Event, StdResult};
    use cw721_base::{Cw721Contract, ExecuteMsg};

    use crate::state::{BookingTicket, SeatsWithBookingStatus, ServicesDays, TicketType, TicketTypeWithTotalSeat, TicketTypeWithTotalSeatState, Train, TrainSeats, TrainSeatsWithBokkingStatus, BOOKED_TRAIN_SEAT, TICKETS_STATE, TICKET_TYPES, TICKET_TYPES_COUNT, TRAIN_COUNT, TRAIN_SEATS, TRAIN_STATE};

    use super::*;


   pub fn add_train(
    deps:DepsMut,
    _info:MessageInfo,
    train_name : String,
     train_no:u64, 
    total_no_seat : u64, 
    origin_stop:String,
     destination_stop :String,
     origin_stop_departure_time_with_basic_date_eg:String,
     destination_stop_arrival_time_with_basic_date_eg:String,
     total_distance_in_km : u64,
     service_days : Vec<ServicesDays>,
     ticket_type_with_total_seat : Vec<TicketTypeWithTotalSeat>,
    
  )-> Result<Response,StdError>{

      let train_count = TRAIN_COUNT.load(deps.storage)?;


      let mut ticket_types_state : Vec<TicketTypeWithTotalSeatState> = Vec::new();

      let ticket_types = TICKET_TYPES.load(deps.storage)?;

      println!("ticket type available is : {:#?}",ticket_types);

      for ticket_type_with_occupy_seat in ticket_type_with_total_seat {

        for ticket_type in ticket_types.clone() {
            
            if ticket_type.ticket_type_id == ticket_type_with_occupy_seat.ticket_type_id {

              println!("ticket type match is : {:#?}",ticket_type);

               let ticket_type_seats = TicketTypeWithTotalSeatState{
                ticket_type_id : ticket_type.ticket_type_id,
                ticket_type_name : ticket_type.ticket_type_name,
                extra_charge : ticket_type.extra_charges,
                total_seat_for_ticket_type : ticket_type_with_occupy_seat.total_seat_for_ticket_type,
               };


               ticket_types_state.push(ticket_type_seats);
               break;

            }
        }
 
       }

       println!("ticket type with total seat occupy by ticket type : {:#?}",ticket_types_state);

      if train_count == 0{


        let new_train = Train{
          total_no_seat : total_no_seat.clone(),
          train_id : train_count + 1,
          train_name : train_name.clone(),
          train_no,
          origin_stop,
          destination_stop,
          origin_stop_departure_time_with_basic_date_eg,
          destination_stop_arrival_time_with_basic_date_eg,
          total_distance_in_km,
          service_days,
          ticket_type_with_total_seat_for_ticket_type : ticket_types_state
        };

          let trains = vec![new_train];

          TRAIN_STATE.save(deps.storage, &trains)?;

          let train_count = train_count + 1;

          TRAIN_COUNT.save(deps.storage, &train_count)?;


           let mut seats_with_booking_status : Vec<String> = Vec::new();

          for train_seat in 1..total_no_seat+1 {
              
              let seat_with_status = format!("A{}",train_seat);
                

              seats_with_booking_status.push(seat_with_status);
          }

          println!("setas for train is {:#?}",seats_with_booking_status);

          let train_seats = TrainSeats{
            train_id : train_count,
            seats_no : seats_with_booking_status
          };

           TRAIN_SEATS.save(deps.storage, train_count, &train_seats)?;
      
          let event = Event::new("add_new_train")
                        .add_attribute("train_count",train_count.to_string())
                        .add_attribute("train_name", train_name);

          let resp = Response::new()
                  .add_event(event);

        return Ok(resp);                  

      } 

       let mut existing_trains = TRAIN_STATE.load(deps.storage)?;
 
       let train_count = TRAIN_COUNT.load(deps.storage)?;

       let new_train = Train{
        train_id : train_count +1,
        total_no_seat,
        train_name: train_name.clone(),
        train_no,
        origin_stop,
        destination_stop,
        origin_stop_departure_time_with_basic_date_eg,
        destination_stop_arrival_time_with_basic_date_eg,
        total_distance_in_km,
        service_days,
        ticket_type_with_total_seat_for_ticket_type : ticket_types_state
       };

      existing_trains.push(new_train);

      TRAIN_STATE.save(deps.storage,&existing_trains)?;

      let train_count = train_count+1;

      TRAIN_COUNT.save(deps.storage, &train_count)?;


      let mut seats_with_booking_status : Vec<String> = Vec::new();

      for train_seat in 1..total_no_seat+1 {
          
          let seat_with_status = format!("A{}",train_seat);
           

          seats_with_booking_status.push(seat_with_status);
      }

      println!("seats for train is {:#?}",seats_with_booking_status);

      let train_seats = TrainSeats{
        train_id  : train_count,
        seats_no : seats_with_booking_status
      };

     TRAIN_SEATS.save(deps.storage, train_count, &train_seats)?;

     
      let event = Event::new("add_new_train_in_existing")
      .add_attribute("train_count",train_count.to_string())
      .add_attribute("train_name", train_name);

      let resp = Response::new()
             .add_event(event);

           Ok(resp)                  


   }

   pub fn add_ticket_type(deps:DepsMut,ticket_type_name:String,extra_charges:u64)->Result<Response,StdError>{

       let mut ticket_types = TICKET_TYPES.load(deps.storage)?;

       println!("ticket type is : {:#?}",ticket_types);

       let ticket_type_count = TICKET_TYPES_COUNT.load(deps.storage)?;

        println!("ticket type count : {}",ticket_type_count);

       let new_ticket_type = TicketType{
        ticket_type_id : ticket_type_count+1,
        ticket_type_name : ticket_type_name.clone(),
        extra_charges : extra_charges.clone()
       };

       ticket_types.push(new_ticket_type);

       TICKET_TYPES.save(deps.storage, &ticket_types)?;

       let ticket_type_count = ticket_type_count + 1;

       TICKET_TYPES_COUNT.save(deps.storage, &ticket_type_count)?;

       let event = Event::new("add_ticket_type")
       .add_attribute("ticket_type", ticket_type_name)
       .add_attribute("extra_charges", extra_charges.to_string());

      let resp = Response::new()
        .add_event(event);

     return Ok(resp);  


   }


   pub fn remove_ticket_type(deps:DepsMut,ticket_type_id : u64)-> Result<Response,StdError>{

       TICKET_TYPES.update(deps.storage, move|ticket_types|->StdResult<_>{

        let ticket_types = ticket_types
        .into_iter()
        .filter(|ticket_type|  ticket_type.ticket_type_id != ticket_type_id)
        .collect();

       Ok(ticket_types)
       })?;

       let event = Event::new("remove_ticket_type")
             .add_attribute("remove_ticket_type_id", ticket_type_id.to_string());

         let resp = Response::new()
                .add_event(event);

         Ok(resp)        


   }


   pub fn booked_ticket(
    deps:DepsMut,
    info : MessageInfo,
    env : Env,
    first_name : String,
    last_name : String,
    mobile_no : u64,
    age : u16,
    date_of_birth : String, //ISO 8601 format : "YYYY-MM-DD"
    nationality : String,
    train_id : u64,
    travelling_date : String,
    ticket_type_id : u64
   )-> Result<Response,StdError>{

     let trains = TRAIN_STATE.load(deps.storage)?;

     let mut train : Option<Train> = None;

     for get_train in trains {
         
         if get_train.train_id == train_id {
              train = Some(get_train);
             break;
         }
     }  

   
      match train {
          
          Some(get_train)=> {

            let booked_train_seat_key = format!("{}{}",travelling_date.clone(),get_train.train_id);

            let train_booking_status = BOOKED_TRAIN_SEAT.load(deps.storage, booked_train_seat_key.clone());

             match train_booking_status {
                 
                 Ok(mut train_seats_booking_status)=>{

                  println!("may be all seats are booked on travelling date");

                  let train_seats = TRAIN_SEATS.load(deps.storage, train_id)?;

                  println!("total seat in train is : {}",train_seats.seats_no.len().clone());

                  println!("total booked seat : {}",train_seats_booking_status.seats_with_booking_status.len().clone());

                  if train_seats.seats_no.len() == train_seats_booking_status.seats_with_booking_status.len().clone() {
                      
                     return  Err(StdError::generic_err("all seats are booked"));
                  }

                  let mut booked_ticket_state = TICKETS_STATE.load(deps.storage,booked_train_seat_key.clone())?;

                  println!("booked ticket state : {:#?}",booked_ticket_state);

                  let get_ticket_type = &get_train
                                           .ticket_type_with_total_seat_for_ticket_type
                                            .iter()
                                            .find(|ticket_type|ticket_type.ticket_type_id == ticket_type_id)
                                            .unwrap();

                  let filter_ticket_by_using_ticket_type : Vec<BookingTicket>= booked_ticket_state.clone()
                                                 .into_iter()
                                                 .filter(|booked_ticket|booked_ticket.ticket_type == get_ticket_type.ticket_type_name.clone())
                                                 .collect();
                      
                      println!("the {} category have total seat in {} : {}, {:#?}",get_ticket_type.ticket_type_name,get_train.train_name,get_ticket_type.total_seat_for_ticket_type,get_ticket_type);

                      println!("{} category booked tickit in {} : {}, {:#?}",get_ticket_type.ticket_type_name,get_train.train_name,filter_ticket_by_using_ticket_type.len(),filter_ticket_by_using_ticket_type);


                       if get_ticket_type.total_seat_for_ticket_type as usize == filter_ticket_by_using_ticket_type.len() {
                           
                          return  Err(StdError::generic_err("this ticket type all seats are booked"));
                       } 


                       


                   let (origin_destination_travel_time_date,final_destional_travel_time_date)= adjust_origin_and_destination_travel_date_and_time(&get_train, travelling_date.clone())?;                            


                   let booked_seat = SeatsWithBookingStatus{
                    seat_no : train_seats.seats_no[train_seats_booking_status.seats_with_booking_status.len()].clone(),
                    booking_status : true,
                   };

                   train_seats_booking_status.seats_with_booking_status.push(booked_seat.clone());

                   println!("booked train seat : {:#?}",train_seats_booking_status);

                    BOOKED_TRAIN_SEAT.save(deps.storage, booked_train_seat_key.clone(), &train_seats_booking_status)?;


                    let ticket_id = format!("{} {} {}",train_id,booked_seat.seat_no.clone(),travelling_date.clone());                             

                   let booked_ticket = BookingTicket{
                     train_id,
                     owner_addr : info.sender.clone(),
                     first_name,
                     last_name,
                     mobile_no,
                     age,
                     date_of_birth,
                     nationality,
                     price : info.funds,
                     seat_no : booked_seat.seat_no,
                     date_of_travelling : travelling_date.clone(),
                     train_name : get_train.train_name.clone(),
                     train_no : get_train.train_no,
                     ticket_type : get_ticket_type.ticket_type_name.to_string(),
                     origin_destination : get_train.origin_stop.clone(),
                     final_destination : get_train.destination_stop.clone(),
                     total_distance_in_km : get_train.total_distance_in_km,
                     ticket_id : ticket_id.clone(),
                     origin_destination_departure_time_with_date : origin_destination_travel_time_date,
                     final_destination_arrival_time_with_date : final_destional_travel_time_date,
                   };
                   

                   booked_ticket_state.push(booked_ticket.clone());

                   println!("booked ticket are : {:#?}",booked_ticket_state);

                   TICKETS_STATE.save(deps.storage, booked_train_seat_key.clone(), &booked_ticket_state)?;

                
                    //construct mint message 
                 let mint_message = ExecuteMsg::<BookingTicket,Empty>::Mint { 
                  token_id: ticket_id.clone(),
                   owner: info.sender.clone().to_string(), //the owner of minted nft (person who booked ticket)
                    token_uri: None,
                     extension: booked_ticket
                     };


                    // Execute the minting
    let contract: Cw721Contract<BookingTicket, Empty, Empty, Empty> = Cw721Contract::default();
    let mint_response = contract.execute(deps, env.clone(), MessageInfo { 
      sender: env.contract.address.clone(),
       funds: vec![]},mint_message)
          .map_err(|err| StdError::generic_err(format!("failed to excute min msg {}",err)))?;


          

          println!("mint resonse : {:#?}",mint_response);
          

                   let event = Event::new("booked_ticket")
                   .add_attribute("ticket_id", ticket_id)
                   .add_attributes(mint_response.attributes);

                   let resp = Response::new()
                                .add_event(event);

                          Ok(resp)   
                 }
                 Err(_)=>{

                  println!("0 seats are booke on travelling date");

                  let (origin_destination_travel_time_date,final_destional_travel_time_date)= adjust_origin_and_destination_travel_date_and_time(&get_train, travelling_date.clone())?;                            



                  let train_seats = TRAIN_SEATS.load(deps.storage, train_id)?;

                   let booked_seat = SeatsWithBookingStatus{
                    seat_no : train_seats.seats_no[0].clone(),
                    booking_status : true
                   };

                  
                   let booked_train_seat = TrainSeatsWithBokkingStatus{
                    train_id,
                    booking_date : travelling_date.clone(),
                    seats_with_booking_status : vec![booked_seat.clone()],
                   };


                   let booked_train_seat_key = format!("{}{}",travelling_date,train_id);

                   println!("booked train seat is : {:#?}",booked_train_seat);

                   BOOKED_TRAIN_SEAT.save(deps.storage, booked_train_seat_key.clone(), &booked_train_seat)?;

                   let get_train_ticket_type = get_train.ticket_type_with_total_seat_for_ticket_type
                                                                             .iter()
                                                                             .find(|ticket_type|ticket_type.ticket_type_id == ticket_type_id)
                                                                             .unwrap();

                let ticket_id = format!("{} {} {}",train_id,booked_seat.seat_no.clone(),travelling_date.clone());                                                                

                   let booked_ticket = BookingTicket{
                    train_id,
                    owner_addr : info.sender.clone(),
                    first_name,
                    last_name,
                    mobile_no,
                    age,
                    date_of_birth,
                    nationality,
                    price : info.funds,
                    seat_no : booked_seat.seat_no.clone(),
                    date_of_travelling : travelling_date.clone(),
                    train_name : get_train.train_name.clone(),
                    train_no : get_train.train_no,
                    ticket_type : get_train_ticket_type.ticket_type_name.to_string(),
                    origin_destination : get_train.origin_stop.clone(),
                    final_destination : get_train.destination_stop.clone(),
                    total_distance_in_km : get_train.total_distance_in_km,
                    ticket_id : ticket_id.clone(),
                    origin_destination_departure_time_with_date : origin_destination_travel_time_date,
                    final_destination_arrival_time_with_date : final_destional_travel_time_date,
                  };

                  let booked_ticket_state :Vec<BookingTicket>  = vec![booked_ticket.clone()];

        
                   TICKETS_STATE.save(deps.storage, booked_train_seat_key.clone(), &booked_ticket_state)?;
                   
                    //construct mint message 
                 let mint_message = ExecuteMsg::<BookingTicket,Empty>::Mint { 
                  token_id: ticket_id.clone(),
                   owner: info.sender.clone().to_string(), //the owner of minted nft (person who booked ticket)
                    token_uri: None,
                     extension: booked_ticket
                     };
                    
                    // Execute the minting
    let contract: Cw721Contract<BookingTicket, Empty, Empty, Empty> = Cw721Contract::default();
    let mint_response = contract.execute(deps, env.clone(), MessageInfo { 
      sender: env.contract.address.clone(),
       funds: vec![]},mint_message)
          .map_err(|err| StdError::generic_err(format!("failed to excute min msg {}",err)))?;


          println!("mint resonse : {:#?}",mint_response);

                   let event = Event::new("booked_ticket")
                          .add_attribute("ticket_id", ticket_id)
                          .add_attributes(mint_response.attributes);

                   let resp = Response::new()
                                .add_event(event);
                    

                          Ok(resp)    
                 }

             }

          },
          None => Err(StdError::generic_err("train is not found"))
      }
        
      }

   }

pub fn adjust_origin_and_destination_travel_date_and_time(
  train : &Train,user_travel_date : String)-> StdResult<(String,String)>{


   // Parse user travel date
   let user_parse_travel_date = NaiveDate::parse_from_str(&user_travel_date, "%d/%m/%Y")
   .map_err(|_| StdError::generic_err("Invalid travel date format"))?;

   println!("user parse travel date : {:#?},",user_parse_travel_date);


    // Parse the original train times
    let origin_stop_departure_time = NaiveDateTime::parse_from_str(&train.origin_stop_departure_time_with_basic_date_eg, "%d/%m/%Y %H:%M:%S")
        .map_err(|e| StdError::generic_err(format!("Invalid origin departure time format: {}", e)))?;


        println!("parse origin departure time : {:#?}",origin_stop_departure_time);

     //parse departure train time
    let destination_stop_arrival_time = NaiveDateTime::parse_from_str(&train.destination_stop_arrival_time_with_basic_date_eg, "%d/%m/%Y %H:%M:%S")
          .map_err(|_| StdError::generic_err("Invalid destinaton arrival time"))?;

         println!("destination arrival time : {:#?}",destination_stop_arrival_time);

      // calculate duration of train journey 
      let duration = destination_stop_arrival_time - origin_stop_departure_time;

      println!("duratin will be : {:#?}",duration); 

      // adjust orgin travel time based on user travel date
      let origin_travel_date_with_time_str = format!(
        "{} {}",user_parse_travel_date.format("%d/%m/%Y"),
        origin_stop_departure_time.format("%H:%M:%S")
      );

      println!("origin travel date : {:#?}",origin_travel_date_with_time_str);

      //parse the user travel date
      let parse_origin_date_and_time = NaiveDateTime::parse_from_str(&origin_travel_date_with_time_str, "%d/%m/%Y %H:%M:%S")
                 .map_err(|_| StdError::generic_err("Invalid origin travel date"))?;

      //calculate destination time 
      let destination_travel_date_time = parse_origin_date_and_time + duration;

      println!("destion travel time : {:#?}",destination_travel_date_time);

      let parse_destination_travel_time_and_date_str = format!(
        "{} {}",
        destination_travel_date_time.format("%d/%m/%Y"),
        destination_travel_date_time.format("%H:%M:%S")
      );

      Ok((origin_travel_date_with_time_str,parse_destination_travel_time_and_date_str))          

}