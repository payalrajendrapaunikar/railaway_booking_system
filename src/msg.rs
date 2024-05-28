
use serde::{Deserialize, Serialize};

use crate::state::{ServicesDays, TicketTypeWithTotalSeat};




#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct InstatiateMsg{
    
    pub nft_name : String,
    pub nft_symbol : String,
    
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub enum ExcuteMsg{
    AddTrain{
         train_name : String ,
         train_no : u64 ,
         total_no_seat : u64,
         origin_stop:String,
         destination_stop:String,
         origin_stop_departure_time_with_basic_date_eg:String,
         destination_stop_arrival_time_with_basic_date_eg:String,
         total_distance_in_km: u64,
         service_days : Vec<ServicesDays>,
         ticket_type_with_total_seat_for_ticket_type : Vec<TicketTypeWithTotalSeat>,
       
    },
     AddTicketType{
        ticket_type : String,
        extra_charges : u64,
     },
     RemoveTicketType{
        ticket_type_id : u64,
     },
    BookedTicket{
       first_name : String,
       last_name : String,
       mobile_no : u64,
       age : u16,
       date_of_birth : String, //ISO 8601 format : "YYYY-MM-DD"
       nationality : String,

       train_id : u64,
       travelling_date : String,
       ticket_type_id : u64,
    }
}




#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub enum  QueryMsg{

    GetAllTrain{},
    GetServiceDays{},
    GetTrainsUsingOriginAndFinalDestinationWithDateFortravelling{
        origin_destination:String,
        final_destination : String,
         travelling_date : String
    },
    GetAllTicketTypes{},
    GetTicketPrice{ 
        ticket_type_id : u64, 
        total_diatance_in_km : u64 },
    GetBookedTicket{
        train_id : u64, 
        travelling_date : String,
        ticket_id : String
    }     
    
    
}