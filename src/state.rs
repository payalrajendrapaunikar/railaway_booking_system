
use core::fmt;

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub enum ServicesDays{

   Monday,
   Tuesday,
   Wednesday,
   Thursday,
   Friday,
   Saturday,
   Sunday
}

impl fmt::Display for ServicesDays {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{:?}", self) // Use Debug formatting for simplicity (or customize formatting)
   }
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct ServicesDaysInContract{

    pub service_days : Vec<ServicesDays>
}


#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct Train{
   pub train_id :u64,
   pub train_name : String ,
   pub  train_no : u64 ,
   pub total_no_seat : u64,
   pub origin_stop:String,
   pub destination_stop : String,
   pub origin_stop_departure_time_with_basic_date_eg : String,
   pub destination_stop_arrival_time_with_basic_date_eg: String,

   pub total_distance_in_km : u64,
   pub service_days : Vec<ServicesDays>,
   pub ticket_type_with_total_seat_for_ticket_type : Vec<TicketTypeWithTotalSeatState>
}


// here ticket_id is => "train_id seat_no travelling_date"
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct BookingTicket{

   pub ticket_id : String, 
   pub owner_addr : Addr,
   pub first_name : String,
   pub last_name : String,
   pub mobile_no : u64,
   pub age : u16,
   pub date_of_birth : String, //ISO 8601 format : "YYYY-MM-DD"
   pub nationality : String,
   pub price : Vec<Coin>,
   pub seat_no : String,
   pub date_of_travelling : String,
   pub train_id : u64,
   pub train_name : String ,
   pub  train_no : u64 ,
   pub ticket_type : String,
   pub origin_destination : String,
   pub final_destination : String,
   pub origin_destination_departure_time_with_date : String,
   pub final_destination_arrival_time_with_date : String,
   pub total_distance_in_km : u64,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct TrainSeatsWithBokkingStatus{

    pub booking_date : String,
    pub seats_with_booking_status : Vec<SeatsWithBookingStatus>,
    pub train_id : u64,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct AvailableTicketForBooking{
  
      pub ticket_type_name : String,
      pub available_ticket : u64,

}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct SeatsWithBookingStatus{

   pub seat_no : String,
   pub booking_status : bool
}


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub struct TicketTypeWithTotalSeat{

   pub ticket_type_id : u64,
   pub total_seat_for_ticket_type : u64
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub struct TicketTypeWithTotalSeatState{

   pub ticket_type_id : u64,
   pub ticket_type_name : String,
   pub extra_charge : u64,
   pub total_seat_for_ticket_type : u64
}


#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct TicketType{
    pub ticket_type_id : u64,
    pub ticket_type_name : String,
    pub extra_charges : u64,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct TrainSeats{

   pub train_id : u64,
   pub seats_no : Vec<String>
}



//storage items
//using this we generated the train id 
pub const TRAIN_COUNT : Item<u64> = Item::new("train_count");

pub const TRAIN_STATE : Item<Vec<Train>> = Item::new("train_state");
pub const PRICE_PER_KM : Item<Coin> = Item::new("price_per_km");
pub const SERVICE_DAYS :Item<ServicesDaysInContract> = Item::new("service_days");
pub const TICKET_TYPES : Item<Vec<TicketType>> = Item::new("ticket_type");

//using this we generated ticket type id
pub const TICKET_TYPES_COUNT : Item<u64> = Item::new("ticket_type_count");


//booked train seats with travelling date , train_id  as key
pub const BOOKED_TRAIN_SEAT : Map<String,TrainSeatsWithBokkingStatus> = Map::new("train_seats_with_booking_status");

//ticket get by using travelling date ,train_id as key 
pub const TICKETS_STATE : Map<String,Vec<BookingTicket>> = Map::new("booked_ticket");


// get train seats using key that is train id
pub const TRAIN_SEATS : Map<u64,TrainSeats> = Map::new("train seats");