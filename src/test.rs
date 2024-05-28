

#[cfg(test)]
mod tests{
    

    use cosmwasm_std::{coins, Addr, Coin, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::{contract::{execute, instantiate}, msg::{ExcuteMsg, InstatiateMsg, QueryMsg}, query_contract:: query_msg, state::{BookingTicket, ServicesDays, ServicesDaysInContract, TicketType, TicketTypeWithTotalSeat, TicketTypeWithTotalSeatState, Train}};



    #[test]
    pub fn add_train(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

      let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

     let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap(); 

          let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "AC".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();                           
                  

        let excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr,
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 123440,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                   destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/04/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/04/2024 3:00:20".to_string(),
                                   total_distance_in_km : 738,
                                   service_days: vec![
                                    ServicesDays::Monday,
                                    ServicesDays:: Wednesday,
                                    ServicesDays::Friday,
                                    ServicesDays::Saturday
                                  ],
                                  ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                  ],
                                 
                                  }, 
                                   &[]).unwrap();   

               let event = excute_add_train_msg
                            .events
                             .iter()
                             .find(|ev|ev.ty == "wasm-add_new_train")
                             .expect("event not found at the excute msg for add train");

                let attr = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_count")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr.value,
                      "1"
                    );   

               let attr2 = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_name")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr2.value,
                      "Satabadi Express"
                    );                     
    }


    #[test]
    pub fn add_multiple_train(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

      let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

     let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap(); 

          let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "AC".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();       

        let excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(),
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 12340,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                   destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/04/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/04/2024 3:00:20".to_string(),
                                   total_distance_in_km: 738,
                                   service_days : vec![
                                     ServicesDays::Monday,
                                     ServicesDays::Tuesday,
                                     ServicesDays::Wednesday
                                   ],
                                   ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                   ],
                                  }, 
                                   &[]).unwrap();   

               let event = excute_add_train_msg
                            .events
                             .iter()
                             .find(|ev|ev.ty == "wasm-add_new_train")
                             .expect("event not found at the excute msg for add train");

                let attr = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_count")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr.value,
                      "1"
                    );   

               let attr2 = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_name")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr2.value,
                      "Satabadi Express"
                    ); 

     let excute_add_train_msg = app
                    .execute_contract(
                     Addr::unchecked("user1"),
                      addr,
                       &ExcuteMsg::AddTrain { 
                         train_name: "Chennai Express".to_string(), 
                         train_no: 123540,
                          total_no_seat: 10,
                           origin_stop: "Nagpur".to_string(),
                            destination_stop: "Chennai".to_string(),
                            origin_stop_departure_time_with_basic_date_eg:"10/04/2024 02:00:00".to_string(),
                            destination_stop_arrival_time_with_basic_date_eg:"12/04/2024 3:00:20".to_string(),
                            total_distance_in_km:1024,
                            service_days: vec![
                              ServicesDays::Monday,
                              ServicesDays:: Wednesday,
                              ServicesDays::Friday,
                              ServicesDays::Saturday
                            ],
                            ticket_type_with_total_seat_for_ticket_type:vec![
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 1,
                                total_seat_for_ticket_type : 5,
                              },
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 2,
                                total_seat_for_ticket_type : 5,
                              },
                            ],
                           }, 
                            &[]).unwrap();   

        let event = excute_add_train_msg
                     .events
                      .iter()
                      .find(|ev|ev.ty == "wasm-add_new_train_in_existing")
                      .expect("event not found at the excute msg for add train");

         let attr = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_count")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr.value,
               "2"
             );   

        let attr2 = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_name")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr2.value,
               "Chennai Express"
             ); 

               

    }

    #[test]
    pub fn get_all_train(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 


      let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

     let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 20 }, 
                        &[]).unwrap(); 

          let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "AC".to_string(),
                         extra_charges: 60 }, 
                        &[]).unwrap();         

        let excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(),
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 123440,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                   destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                                   total_distance_in_km:738,
                                   service_days: vec![
                                    ServicesDays::Monday,
                                    ServicesDays:: Wednesday,
                                    ServicesDays::Friday,
                                    ServicesDays::Saturday
                                  ],
                                  ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                  ],
                                  }, 
                                   &[]).unwrap();   

               let event = excute_add_train_msg
                            .events
                             .iter()
                             .find(|ev|ev.ty == "wasm-add_new_train")
                             .expect("event not found at the excute msg for add train");

                let attr = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_count")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr.value,
                      "1"
                    );   

               let attr2 = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_name")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr2.value,
                      "Satabadi Express"
                    ); 

     let excute_add_train_msg = app
                    .execute_contract(
                     Addr::unchecked("user1"),
                      addr.clone(),
                       &ExcuteMsg::AddTrain { 
                         train_name: "Chennai Express".to_string(), 
                         train_no: 123540,
                          total_no_seat: 10,
                           origin_stop: "Nagpur".to_string(),
                            destination_stop: "Chennai".to_string(),
                            origin_stop_departure_time_with_basic_date_eg:"10/02/2024 02:00:00".to_string(),
                            destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 4:00:20".to_string(),
                            total_distance_in_km:1121,
                            service_days: vec![
                              ServicesDays::Monday,
                              ServicesDays:: Wednesday,
                              ServicesDays::Friday,
                              ServicesDays::Saturday
                            ],
                            ticket_type_with_total_seat_for_ticket_type:vec![
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 1,
                                total_seat_for_ticket_type : 5,
                              },
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 2,
                                total_seat_for_ticket_type : 3,
                              },
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 3,
                                total_seat_for_ticket_type : 2
                              }
                            ],
                           }, 
                            &[]).unwrap();   

        let event = excute_add_train_msg
                     .events
                      .iter()
                      .find(|ev|ev.ty == "wasm-add_new_train_in_existing")
                      .expect("event not found at the excute msg for add train");

         let attr = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_count")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr.value,
               "2"
             );   

        let attr2 = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_name")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr2.value,
               "Chennai Express"
             ); 

          let query_msg_get_all_train : Vec<Train>= app
                             .wrap()
                              .query_wasm_smart(
                              addr.clone(),
                               &QueryMsg::GetAllTrain {  }
                              ).unwrap();
                 
                 assert_eq!(
                  query_msg_get_all_train,
                  vec![
                    Train{
                      train_name: "Satabadi Express".to_string(), 
                      train_no: 123440,
                       total_no_seat: 10,
                        origin_stop: "nagpur".to_string(),
                         destination_stop: "pune".to_string(),
                         train_id : 1,
                         origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                         destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                         total_distance_in_km:738,
                         service_days: vec![
                          ServicesDays::Monday,
                          ServicesDays:: Wednesday,
                          ServicesDays::Friday,
                          ServicesDays::Saturday
                        ],
                        ticket_type_with_total_seat_for_ticket_type:vec![
                           TicketTypeWithTotalSeatState{
                            ticket_type_id : 1,
                            ticket_type_name : "General".to_string(),
                            extra_charge : 40,
                            total_seat_for_ticket_type : 5
                        
                           },
                           TicketTypeWithTotalSeatState{
                            ticket_type_id : 2,
                            ticket_type_name : "Sleeper".to_string(),
                            extra_charge : 20,
                            total_seat_for_ticket_type : 3
                        
                           },
                           TicketTypeWithTotalSeatState{
                            ticket_type_id : 3,
                            ticket_type_name : "AC".to_string(),
                            extra_charge : 60,
                            total_seat_for_ticket_type : 2
                        
                           },

                        ]
                         
                    },
                    Train{
                      train_name: "Chennai Express".to_string(), 
                      train_no: 123540,
                       total_no_seat: 10,
                        origin_stop: "Nagpur".to_string(),
                         destination_stop: "Chennai".to_string(),
                         train_id : 2,
                         origin_stop_departure_time_with_basic_date_eg:"10/02/2024 02:00:00".to_string(),
                         destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 4:00:20".to_string(),
                         total_distance_in_km:1121,
                         service_days: vec![
                          ServicesDays::Monday,
                          ServicesDays:: Wednesday,
                          ServicesDays::Friday,
                          ServicesDays::Saturday
                        ],
                        ticket_type_with_total_seat_for_ticket_type:vec![
                          TicketTypeWithTotalSeatState{
                            ticket_type_id : 1,
                            ticket_type_name : "General".to_string(),
                            extra_charge : 40,
                            total_seat_for_ticket_type : 5
                        
                           },
                           TicketTypeWithTotalSeatState{
                            ticket_type_id : 2,
                            ticket_type_name : "Sleeper".to_string(),
                            extra_charge : 20,
                            total_seat_for_ticket_type : 3
                        
                           },
                           TicketTypeWithTotalSeatState{
                            ticket_type_id : 3,
                            ticket_type_name : "AC".to_string(),
                            extra_charge : 60,
                            total_seat_for_ticket_type : 2
                        
                           },
                        ]
                    }
                  ]
                 )

    }

    #[test]  
    pub fn gettrain_byusing_origin_and_final_destination_withdate_fortravelling(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

        let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

     let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 20 }, 
                        &[]).unwrap(); 

          let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "AC".to_string(),
                         extra_charges: 60 }, 
                        &[]).unwrap();   

        let excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(),
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 12340,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                   destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                                   total_distance_in_km:738,
                                   service_days: vec![
                                    ServicesDays::Monday,
                                    ServicesDays:: Wednesday,
                                    ServicesDays::Friday,
                                    ServicesDays::Saturday
                                  ],
                                  ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                  ],
                                  }, 
                                   &[]).unwrap();   

               let event = excute_add_train_msg
                            .events
                             .iter()
                             .find(|ev|ev.ty == "wasm-add_new_train")
                             .expect("event not found at the excute msg for add train");

                let attr = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_count")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr.value,
                      "1"
                    );   

               let attr2 = event
                          .attributes
                          .iter()
                           .find(|attr|attr.key == "train_name")
                           .expect("attribute not found at excute msg for add train");  

                    assert_eq!(
                      attr2.value,
                      "Satabadi Express"
                    ); 


            let excute_add_train_msg = app
                    .execute_contract(
                     Addr::unchecked("user1"),
                      addr.clone(),
                       &ExcuteMsg::AddTrain { 
                         train_name: "Mahakal Express".to_string(), 
                         train_no: 122256,
                          total_no_seat: 10,
                           origin_stop: "nagpur".to_string(),
                            destination_stop: "pune".to_string(),
                            origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                            destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                            total_distance_in_km:738,
                            service_days: vec![
                             ServicesDays::Monday,
                             ServicesDays:: Wednesday,
                             ServicesDays::Friday,
                             ServicesDays::Saturday
                           ],
                           ticket_type_with_total_seat_for_ticket_type:vec![
                            TicketTypeWithTotalSeat{
                              ticket_type_id : 1,
                              total_seat_for_ticket_type : 5,
                            },
                            TicketTypeWithTotalSeat{
                              ticket_type_id : 2,
                              total_seat_for_ticket_type : 3,
                            },
                            TicketTypeWithTotalSeat{
                              ticket_type_id : 3,
                              total_seat_for_ticket_type : 2
                            },
                            
                          ],
                           }, 
                            &[]).unwrap();   

        let event = excute_add_train_msg
                     .events
                      .iter()
                      .find(|ev|ev.ty == "wasm-add_new_train_in_existing")
                      .expect("event not found at the excute msg for add train");

         let attr = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_count")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr.value,
               "2"
             );   

        let attr2 = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_name")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr2.value,
               "Mahakal Express"
             ); 


             let excute_add_train_msg = app
             .execute_contract(
              Addr::unchecked("user1"),
               addr.clone(),
                &ExcuteMsg::AddTrain { 
                  train_name: "Ram Express".to_string(), 
                  train_no: 13456,
                   total_no_seat: 10,
                    origin_stop: "nagpur".to_string(),
                     destination_stop: "pune".to_string(),
                     origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                     destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                     total_distance_in_km:738,
                     service_days: vec![
                      ServicesDays:: Wednesday,
                      ServicesDays::Friday,
                      ServicesDays::Saturday
                    ],
                    ticket_type_with_total_seat_for_ticket_type:vec![
                      TicketTypeWithTotalSeat{
                        ticket_type_id : 1,
                        total_seat_for_ticket_type : 5,
                      },
                      TicketTypeWithTotalSeat{
                        ticket_type_id : 2,
                        total_seat_for_ticket_type : 3,
                      },
                      TicketTypeWithTotalSeat{
                        ticket_type_id : 3,
                        total_seat_for_ticket_type : 2
                      }
                    ],
                    }, 
                     &[]).unwrap();   

 let event = excute_add_train_msg
              .events
               .iter()
               .find(|ev|ev.ty == "wasm-add_new_train_in_existing")
               .expect("event not found at the excute msg for add train");

  let attr = event
            .attributes
            .iter()
             .find(|attr|attr.key == "train_count")
             .expect("attribute not found at excute msg for add train");  

      assert_eq!(
        attr.value,
        "3"
      );   

 let attr2 = event
            .attributes
            .iter()
             .find(|attr|attr.key == "train_name")
             .expect("attribute not found at excute msg for add train");  

      assert_eq!(
        attr2.value,
        "Ram Express"
      ); 


     let excute_add_train_msg = app
                    .execute_contract(
                     Addr::unchecked("user1"),
                      addr.clone(),
                       &ExcuteMsg::AddTrain { 
                         train_name: "Chennai Express".to_string(), 
                         train_no: 1235640,
                          total_no_seat: 10,
                           origin_stop: "Nagpur".to_string(),
                            destination_stop: "Chennai".to_string(),
                            origin_stop_departure_time_with_basic_date_eg:"10/02/2024 02:00:00".to_string(),
                            destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 4:00:20".to_string(),
                            total_distance_in_km:1121,
                            service_days: vec![
                              ServicesDays::Monday,
                              ServicesDays:: Wednesday,
                              ServicesDays::Friday,
                              ServicesDays::Saturday
                            ],
                            ticket_type_with_total_seat_for_ticket_type:vec![
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 1,
                                total_seat_for_ticket_type : 5,
                              },
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 2,
                                total_seat_for_ticket_type : 3,
                              },
                              TicketTypeWithTotalSeat{
                                ticket_type_id : 3,
                                total_seat_for_ticket_type : 2
                              }
                            ],
                           }, 
                            &[]).unwrap();   

        let event = excute_add_train_msg
                     .events
                      .iter()
                      .find(|ev|ev.ty == "wasm-add_new_train_in_existing")
                      .expect("event not found at the excute msg for add train");

         let attr = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_count")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr.value,
               "4"
             );   

        let attr2 = event
                   .attributes
                   .iter()
                    .find(|attr|attr.key == "train_name")
                    .expect("attribute not found at excute msg for add train");  

             assert_eq!(
               attr2.value,
               "Chennai Express"
             ); 

        
        let query_msg_to_get_train_byusing_destination : Vec<Train> = app
                                       .wrap()
                                       .query_wasm_smart(addr.clone(), 
                                        &QueryMsg::GetTrainsUsingOriginAndFinalDestinationWithDateFortravelling { 
                                          origin_destination: "nagpur".to_string(), 
                                          final_destination: "pune".to_string(), 
                                          travelling_date: "20/05/2024".to_string() 
                                        }).unwrap(); 

                        assert_eq!(
                         query_msg_to_get_train_byusing_destination,
                         vec![
                          Train{
                            train_name: "Satabadi Express".to_string(), 
                            train_no: 12340,
                             total_no_seat: 10,
                              origin_stop: "nagpur".to_string(),
                               destination_stop: "pune".to_string(),
                               origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                               destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                               total_distance_in_km:738,
                               service_days: vec![
                                ServicesDays::Monday,
                                ServicesDays:: Wednesday,
                                ServicesDays::Friday,
                                ServicesDays::Saturday
                              ],
                              train_id:1,
                              ticket_type_with_total_seat_for_ticket_type:vec![
                                TicketTypeWithTotalSeatState{
                                  ticket_type_id : 1,
                                  ticket_type_name : "General".to_string(),
                                  extra_charge : 40,
                                  total_seat_for_ticket_type : 5
                              
                                 },
                                 TicketTypeWithTotalSeatState{
                                  ticket_type_id : 2,
                                  ticket_type_name : "Sleeper".to_string(),
                                  extra_charge : 20,
                                  total_seat_for_ticket_type : 3
                              
                                 },
                                 TicketTypeWithTotalSeatState{
                                  ticket_type_id : 3,
                                  ticket_type_name : "AC".to_string(),
                                  extra_charge : 60,
                                  total_seat_for_ticket_type : 2
                              
                                 },
                              ]
                          },

                          Train{
                            train_name: "Mahakal Express".to_string(), 
                            train_no: 122256,
                             total_no_seat: 10,
                              origin_stop: "nagpur".to_string(),
                               destination_stop: "pune".to_string(),
                               origin_stop_departure_time_with_basic_date_eg:"10/02/2024 12:00:00".to_string(),
                               destination_stop_arrival_time_with_basic_date_eg:"11/02/2024 3:00:20".to_string(),
                               total_distance_in_km:738,
                               service_days: vec![
                                ServicesDays::Monday,
                                ServicesDays:: Wednesday,
                                ServicesDays::Friday,
                                ServicesDays::Saturday
                              ],
                              train_id : 2,
                              ticket_type_with_total_seat_for_ticket_type:vec![
                                TicketTypeWithTotalSeatState{
                                  ticket_type_id : 1,
                                  ticket_type_name : "General".to_string(),
                                  extra_charge : 40,
                                  total_seat_for_ticket_type : 5
                              
                                 },
                                 TicketTypeWithTotalSeatState{
                                  ticket_type_id : 2,
                                  ticket_type_name : "Sleeper".to_string(),
                                  extra_charge : 20,
                                  total_seat_for_ticket_type : 3
                              
                                 },
                                 TicketTypeWithTotalSeatState{
                                  ticket_type_id : 3,
                                  ticket_type_name : "AC".to_string(),
                                  extra_charge : 60,
                                  total_seat_for_ticket_type : 2
                              
                                 },
                              ]
                          }

                         ]
                        )                
    }


    #[test]
    fn get_service_days(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

          let query_msg_to_get_service_days : ServicesDaysInContract= app
                                        .wrap()
                                        .query_wasm_smart(addr.clone(), &QueryMsg::GetServiceDays {  })
                                        .unwrap();


             assert_eq!(
              query_msg_to_get_service_days,
              ServicesDaysInContract{
                service_days:vec![
                  ServicesDays::Monday,
                  ServicesDays::Tuesday,
                  ServicesDays::Wednesday,
                  ServicesDays::Thursday,
                 ServicesDays::Friday,
                 ServicesDays::Saturday,
                 ServicesDays::Sunday,
                ]
              }
             );                           
                               
                                      
    }


  
    #[test]
    fn add_ticket_type(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr,
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "General"
                  );      

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "40"
                  );                                   
    }


    #[test]
    fn add_multiple_ticket_type(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "General"
                  );      

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "40"
                  );                


                   let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr,
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 10 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "Sleeper"
                  );      

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "10"
                  );                                     
    }


    

    #[test]
    fn get_all_ticket_type(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "General"
                  );      

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "40"
                  );                


                   let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 10 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "Sleeper"
                  ); 

     

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "10"
                  );  

                  let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "AC".to_string(),
                         extra_charges: 10 }, 
                        &[]).unwrap();

               let get_all_ticket_type : Vec<TicketType>= app
                                     .wrap()
                                     .query_wasm_smart(addr.clone(), &QueryMsg::GetAllTicketTypes {  })
                                     .unwrap();

                                     assert_eq!(
                                      get_all_ticket_type,
                                       vec![
                                        TicketType{
                                           
                                           ticket_type_id : 1,
                                           ticket_type_name : "General".to_string(),
                                           extra_charges:40
                                        },
                                        TicketType{
                                          ticket_type_id : 2,
                                          ticket_type_name:  "Sleeper".to_string() ,
                                          extra_charges : 10
                                        },
                                        TicketType{
                                          ticket_type_id : 3,
                                          ticket_type_name:  "AC".to_string() ,
                                          extra_charges : 10
                                        }
                                       ]
                                     )

    }



    #[test]
    fn remove_ticket_type(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "General"
                  );      

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "40"
                  );                


                   let add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "Sleeper".to_string(),
                         extra_charges: 10 }, 
                        &[]).unwrap();

          let event = add_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-add_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "ticket_type")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "Sleeper"
                  );      

               let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "extra_charges")
                          .expect("attribute not found for add tickek type msg"); 

                  assert_eq!(
                    attr.value,
                    "10"
                  );  

               let get_all_ticket_type : Vec<TicketType>= app
                                     .wrap()
                                     .query_wasm_smart(addr.clone(), &QueryMsg::GetAllTicketTypes {  })
                                     .unwrap();

                                     assert_eq!(
                                      get_all_ticket_type,
                                       vec![
                                        TicketType{
                                           
                                           ticket_type_id : 1,
                                           ticket_type_name : "General".to_string(),
                                           extra_charges:40
                                        },
                                        TicketType{
                                          ticket_type_id : 2,
                                          ticket_type_name:  "Sleeper".to_string() ,
                                          extra_charges : 10
                                        }
                                       ]
                                     );

              let remove_ticket_type = app
                          .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(), 
                             &ExcuteMsg::RemoveTicketType { 
                              ticket_type_id: 1
                            },
                             &[]).unwrap();  

                let event = remove_ticket_type
                     .events
                     .iter()
                      .find(|ev|ev.ty == "wasm-remove_ticket_type")
                      .expect("event not found for add ticket");                
                                      
          let attr = event
                           .attributes
                           .iter()
                           .find(|attr|attr.key == "remove_ticket_type_id")
                          .expect("attribute not found for add tickek type msg"); 

                 assert_eq!(
                  attr.value,
                  "1"
                 ); 


                   let get_all_ticket_type : Vec<TicketType>= app
                                     .wrap()
                                     .query_wasm_smart(addr.clone(), &QueryMsg::GetAllTicketTypes {  })
                                     .unwrap();

                                     assert_eq!(
                                      get_all_ticket_type,
                                       vec![
                                        TicketType{
                                          ticket_type_id : 2,
                                          ticket_type_name:  "Sleeper".to_string() ,
                                          extra_charges : 10
                                        }
                                       ]
                                     )   
                                  

    }


     #[test]
    fn booked_ticket(){

      let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("payal"), coins(1000000, "eth"))
            .unwrap()
    });

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();


       let _add_ticket_type = app 
                        .execute_contract(
                          Addr::unchecked("user1"),
                           addr.clone(),
                            &ExcuteMsg::AddTicketType { 
                              ticket_type: "Sleeper".to_string(),
                               extra_charges: 20 }, 
                              &[]).unwrap();   

         let _add_ticket_type = app 
                              .execute_contract(
                                Addr::unchecked("user1"),
                                 addr.clone(),
                                  &ExcuteMsg::AddTicketType { 
                                    ticket_type: "AC".to_string(),
                                     extra_charges: 60 }, 
                                    &[]).unwrap(); 

           let _excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(),
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 123440,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                   destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/04/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/04/2024 3:00:20".to_string(),
                                   total_distance_in_km : 738,
                                   service_days: vec![
                                    ServicesDays::Monday,
                                    ServicesDays:: Wednesday,
                                    ServicesDays::Friday,
                                    ServicesDays::Saturday
                                  ],
                                  ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                  ],
                                 
                                  }, 
                                   &[]).unwrap();   
                          
             let _excute_add_train_msg = app
                                   .execute_contract(
                                    Addr::unchecked("user1"),
                                     addr.clone(),
                                      &ExcuteMsg::AddTrain { 
                                        train_name: "Chennai Express".to_string(), 
                                        train_no: 123540,
                                         total_no_seat: 10,
                                          origin_stop: "Nagpur".to_string(),
                                           destination_stop: "Chennai".to_string(),
                                           origin_stop_departure_time_with_basic_date_eg:"10/04/2024 02:00:00".to_string(),
                                           destination_stop_arrival_time_with_basic_date_eg:"12/04/2024 3:00:20".to_string(),
                                           total_distance_in_km:1024,
                                           service_days: vec![
                                             ServicesDays::Monday,
                                             ServicesDays:: Wednesday,
                                             ServicesDays::Friday,
                                             ServicesDays::Saturday
                                           ],
                                           ticket_type_with_total_seat_for_ticket_type:vec![
                                             TicketTypeWithTotalSeat{
                                               ticket_type_id : 1,
                                               total_seat_for_ticket_type : 5,
                                             },
                                             TicketTypeWithTotalSeat{
                                               ticket_type_id : 2,
                                               total_seat_for_ticket_type : 5,
                                             },
                                           ],
                                          }, 
            
                                         &[]).unwrap();   
                         
      let _query_msg_to_get_train_byusing_destination : Vec<Train> = app
                                         .wrap()
                                         .query_wasm_smart(addr.clone(), 
                                          &QueryMsg::GetTrainsUsingOriginAndFinalDestinationWithDateFortravelling { 
                                            origin_destination: "nagpur".to_string(), 
                                            final_destination: "pune".to_string(), 
                                            travelling_date: "20/05/2024".to_string() 
                                          }).unwrap(); 

          let get_ticket_price : Coin= app
                                     .wrap()
                                      .query_wasm_smart(addr.clone(),
                                       &QueryMsg::GetTicketPrice { 
                                        ticket_type_id: 1, total_diatance_in_km: 738 })
                                        .unwrap();                              
                                         
                    assert_eq!(
                      get_ticket_price,
                      Coin{
                        denom : "eth".to_string(),
                        amount : Uint128::new(2992)
                      }
                    );                  

                  let excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Payal".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "10/01/2003".to_string(),
                                           nationality: "Indian".to_string(),
                                               train_id: 1,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 1, 
                                                  },
                                                   &coins(2992, "eth".to_string())
                                                    ).unwrap();

                      let event = excute_booked_ticket
                                        .events
                                        .iter()
                                         .find(|ev|ev.ty == "wasm-booked_ticket")
                                         .expect("event type for booking is not found");

                       let attr = event
                                  .attributes
                                  .iter()
                                  .find(|attr|attr.key == "ticket_id")
                                  .expect("attribute not found for booked ticket");  


                           assert_eq!( 
                            attr.value,
                            "1 A1 30/04/2024"
                          );                                                 

                    let excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Prachi".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "20/07/19976".to_string(),
                                           nationality: "Indian".to_string(),
                                               train_id: 2,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 1
                                                  },
                                                   &coins(4136, "eth".to_string())).unwrap();      

                       let event = excute_booked_ticket
                                                   .events
                                                   .iter()
                                                    .find(|ev|ev.ty == "wasm-booked_ticket")
                                                    .expect("event type for booking is not found");
           
                                  let attr = event
                                             .attributes
                                             .iter()
                                             .find(|attr|attr.key == "ticket_id")
                                             .expect("attribute not found for booked ticket");  
           
           
                                      assert_eq!( 
                                       attr.value,
                                       "2 A1 30/04/2024"
                                     );                                      

                     let excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Shubham".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "20/07/19976".to_string(),
                                           nationality: "Indian".to_string(),
                                            
                                               train_id: 1,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 1
                                                  },
                                                   &coins(2992, "eth".to_string())).unwrap();   

                               let event = excute_booked_ticket
                                        .events
                                        .iter()
                                         .find(|ev|ev.ty == "wasm-booked_ticket")
                                         .expect("event type for booking is not found");

                       let attr = event
                                  .attributes
                                  .iter()
                                  .find(|attr|attr.key == "ticket_id")
                                  .expect("attribute not found for booked ticket");  


                           assert_eq!( 
                            attr.value,
                            "1 A2 30/04/2024"
                          );   

                          let _excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Sangita".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "10/01/2003".to_string(),
                                           nationality: "Indian".to_string(),
                                               train_id: 1,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 2, 
                                                  },
                                                   &coins(2972, "eth".to_string())).unwrap();   

                           
    }

    #[test]
    fn calculate_ticket_price(){

      let mut app = App::default();

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();


       let _add_ticket_type = app 
                        .execute_contract(
                          Addr::unchecked("user1"),
                           addr.clone(),
                            &ExcuteMsg::AddTicketType { 
                              ticket_type: "Sleeper".to_string(),
                               extra_charges: 20 }, 
                              &[]).unwrap();   

         let _add_ticket_type = app 
                              .execute_contract(
                                Addr::unchecked("user1"),
                                 addr.clone(),
                                  &ExcuteMsg::AddTicketType { 
                                    ticket_type: "AC".to_string(),
                                     extra_charges: 60 }, 
                                    &[]).unwrap(); 

           let _excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(),
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 123440,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                  destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/04/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/04/2024 3:00:20".to_string(),
                                   total_distance_in_km : 738,
                                   service_days: vec![
                                    ServicesDays::Monday,
                                    ServicesDays:: Wednesday,
                                    ServicesDays::Friday,
                                    ServicesDays::Saturday
                                  ],
                                  ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                  ],
                                 
                                  }, 
                                   &[]).unwrap();   
                          
             let _excute_add_train_msg = app
                                   .execute_contract(
                                    Addr::unchecked("user1"),
                                     addr.clone(),
                                      &ExcuteMsg::AddTrain { 
                                        train_name: "Chennai Express".to_string(), 
                                        train_no: 123540,
                                         total_no_seat: 10,
                                          origin_stop: "Nagpur".to_string(),
                                           destination_stop: "Chennai".to_string(),
                                           origin_stop_departure_time_with_basic_date_eg:"10/04/2024 02:00:00".to_string(),
                                           destination_stop_arrival_time_with_basic_date_eg:"12/04/2024 3:00:20".to_string(),
                                           total_distance_in_km:1024,
                                           service_days: vec![
                                             ServicesDays::Monday,
                                             ServicesDays:: Wednesday,
                                             ServicesDays::Friday,
                                             ServicesDays::Saturday
                                           ],
                                           ticket_type_with_total_seat_for_ticket_type:vec![
                                             TicketTypeWithTotalSeat{
                                              ticket_type_id : 1,
                                               total_seat_for_ticket_type : 5,
                                             },
                                             TicketTypeWithTotalSeat{
                                               ticket_type_id : 2,
                                               total_seat_for_ticket_type : 5,
                                             },
                                           ],
                                          }, 
            
                                         &[]).unwrap();   
                         
      let _query_msg_to_get_train_byusing_destination : Vec<Train> = app
                                         .wrap()
                                         .query_wasm_smart(addr.clone(), 
                                          &QueryMsg::GetTrainsUsingOriginAndFinalDestinationWithDateFortravelling { 
                                            origin_destination: "nagpur".to_string(), 
                                            final_destination: "pune".to_string(), 
                                            travelling_date: "20/05/2024".to_string() 
                                          }).unwrap(); 

          let get_ticket_price : Coin= app
                                     .wrap()
                                      .query_wasm_smart(addr.clone(),
                                       &QueryMsg::GetTicketPrice { 
                                        ticket_type_id: 1, total_diatance_in_km: 738 })
                                        .unwrap();    


              assert_eq!(
                      get_ticket_price,
                      Coin{
                        denom : "eth".to_string(),
                        amount : Uint128::new(2992)
                      }
                    );                             
    }

    #[test]
    fn get_booked_ticket(){


       
      let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("payal"), coins(1000000, "eth"))
            .unwrap()
    });

      let code = ContractWrapper::new(
        execute,instantiate,query_msg);

      let code_id = app.store_code(Box::new(code)); 

      let addr = app
                .instantiate_contract(
                  code_id,
                  Addr::unchecked("owner"),
                   &InstatiateMsg{
                    nft_name : "Train Ticket NFT".to_string(),
                    nft_symbol:"TTN".to_string()
                   }, 
                   &[],
                    "TrainTicketBookingContract",
                    None
                  ).unwrap(); 

       let _add_ticket_type = app 
                  .execute_contract(
                    Addr::unchecked("user1"),
                     addr.clone(),
                      &ExcuteMsg::AddTicketType { 
                        ticket_type: "General".to_string(),
                         extra_charges: 40 }, 
                        &[]).unwrap();


       let _add_ticket_type = app 
                        .execute_contract(
                          Addr::unchecked("user1"),
                           addr.clone(),
                            &ExcuteMsg::AddTicketType { 
                              ticket_type: "Sleeper".to_string(),
                               extra_charges: 20 }, 
                              &[]).unwrap();   

         let _add_ticket_type = app 
                              .execute_contract(
                                Addr::unchecked("user1"),
                                 addr.clone(),
                                  &ExcuteMsg::AddTicketType { 
                                    ticket_type: "AC".to_string(),
                                     extra_charges: 60 }, 
                                    &[]).unwrap(); 

           let _excute_add_train_msg = app
                           .execute_contract(
                            Addr::unchecked("user1"),
                             addr.clone(),
                              &ExcuteMsg::AddTrain { 
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 123440,
                                 total_no_seat: 10,
                                  origin_stop: "nagpur".to_string(),
                                   destination_stop: "pune".to_string(),
                                   origin_stop_departure_time_with_basic_date_eg:"10/04/2024 12:00:00".to_string(),
                                   destination_stop_arrival_time_with_basic_date_eg:"11/04/2024 3:00:20".to_string(),
                                   total_distance_in_km : 738,
                                   service_days: vec![
                                    ServicesDays::Monday,
                                    ServicesDays:: Wednesday,
                                    ServicesDays::Friday,
                                    ServicesDays::Saturday
                                  ],
                                  ticket_type_with_total_seat_for_ticket_type:vec![
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 1,
                                      total_seat_for_ticket_type : 5,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 2,
                                      total_seat_for_ticket_type : 3,
                                    },
                                    TicketTypeWithTotalSeat{
                                      ticket_type_id : 3,
                                      total_seat_for_ticket_type : 2
                                    }
                                  ],
                                 
                                  }, 
                                   &[]).unwrap();   
                          
             let _excute_add_train_msg = app
                                   .execute_contract(
                                    Addr::unchecked("user1"),
                                     addr.clone(),
                                      &ExcuteMsg::AddTrain { 
                                        train_name: "Chennai Express".to_string(), 
                                        train_no: 123540,
                                         total_no_seat: 10,
                                          origin_stop: "Nagpur".to_string(),
                                           destination_stop: "Chennai".to_string(),
                                           origin_stop_departure_time_with_basic_date_eg:"10/04/2024 02:00:00".to_string(),
                                           destination_stop_arrival_time_with_basic_date_eg:"12/04/2024 3:00:20".to_string(),
                                           total_distance_in_km:1024,
                                           service_days: vec![
                                             ServicesDays::Monday,
                                             ServicesDays:: Wednesday,
                                             ServicesDays::Friday,
                                             ServicesDays::Saturday
                                           ],
                                           ticket_type_with_total_seat_for_ticket_type:vec![
                                             TicketTypeWithTotalSeat{
                                               ticket_type_id : 1,
                                               total_seat_for_ticket_type : 5,
                                             },
                                             TicketTypeWithTotalSeat{
                                               ticket_type_id : 2,
                                               total_seat_for_ticket_type : 5,
                                             },
                                           ],
                                          }, 
            
                                         &[]).unwrap();   
                         
      let _query_msg_to_get_train_byusing_destination : Vec<Train> = app
                                         .wrap()
                                         .query_wasm_smart(addr.clone(), 
                                          &QueryMsg::GetTrainsUsingOriginAndFinalDestinationWithDateFortravelling { 
                                            origin_destination: "nagpur".to_string(), 
                                            final_destination: "pune".to_string(), 
                                            travelling_date: "20/05/2024".to_string() 
                                          }).unwrap(); 

          let get_ticket_price : Coin= app
                                     .wrap()
                                      .query_wasm_smart(addr.clone(),
                                       &QueryMsg::GetTicketPrice { 
                                        ticket_type_id: 1, total_diatance_in_km: 738 })
                                        .unwrap();                              
                                         
                    assert_eq!(
                      get_ticket_price,
                      Coin{
                        denom : "eth".to_string(),
                        amount : Uint128::new(2992)
                      }
                    );                  

                  let excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Payal".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "10/01/2003".to_string(),
                                           nationality: "Indian".to_string(),
                                               train_id: 1,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 1, 
                                                  },
                                                   &coins(2992, "eth".to_string())
                                                    ).unwrap();

                      let event = excute_booked_ticket
                                        .events
                                        .iter()
                                         .find(|ev|ev.ty == "wasm-booked_ticket")
                                         .expect("event type for booking is not found");

                       let attr = event
                                  .attributes
                                  .iter()
                                  .find(|attr|attr.key == "ticket_id")
                                  .expect("attribute not found for booked ticket");  


                           assert_eq!( 
                            attr.value,
                            "1 A1 30/04/2024"
                          );                                                 

                    let excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Prachi".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "20/07/19976".to_string(),
                                           nationality: "Indian".to_string(),
                                               train_id: 2,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 1
                                                  },
                                                   &coins(4136, "eth".to_string())).unwrap();      

                       let event = excute_booked_ticket
                                                   .events
                                                   .iter()
                                                    .find(|ev|ev.ty == "wasm-booked_ticket")
                                                    .expect("event type for booking is not found");
           
                                  let attr = event
                                             .attributes
                                             .iter()
                                             .find(|attr|attr.key == "ticket_id")
                                             .expect("attribute not found for booked ticket");  
           
           
                                      assert_eq!( 
                                       attr.value,
                                       "2 A1 30/04/2024"
                                     );                                      

                     let excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Shubham".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "20/07/19976".to_string(),
                                           nationality: "Indian".to_string(),
                                            
                                               train_id: 1,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 1
                                                  },
                                                   &coins(2992, "eth".to_string())).unwrap();   

                               let event = excute_booked_ticket
                                        .events
                                        .iter()
                                         .find(|ev|ev.ty == "wasm-booked_ticket")
                                         .expect("event type for booking is not found");

                       let attr = event
                                  .attributes
                                  .iter()
                                  .find(|attr|attr.key == "ticket_id")
                                  .expect("attribute not found for booked ticket");  


                           assert_eq!( 
                            attr.value,
                            "1 A2 30/04/2024"
                          );   

                          let _excute_booked_ticket = app
                                 .execute_contract(
                                  Addr::unchecked("payal"),
                                   addr.clone(),
                                    &ExcuteMsg::BookedTicket{ 
                                      first_name: "Sangita".to_string(),
                                       last_name: "Paunikar".to_string(),
                                        mobile_no: 1234567890,
                                         age: 20,
                                          date_of_birth: "10/01/2003".to_string(),
                                           nationality: "Indian".to_string(),
                                               train_id: 1,
                                                   travelling_date: "30/04/2024".to_string(),
                                                   ticket_type_id : 2, 
                                                  },
                                                   &coins(2972, "eth".to_string())).unwrap();   

                       let get_booked_ticket : BookingTicket = app
                                             .wrap()
                                             .query_wasm_smart(addr.clone(), &QueryMsg::GetBookedTicket { 
                                              train_id: 1,
                                               travelling_date: "30/04/2024".to_string(),
                                                ticket_id: "1 A3 30/04/2024".to_string() }).unwrap();                             


                     assert_eq!(
                      get_booked_ticket,
                      BookingTicket{
                       
                        first_name: "Sangita".to_string(),
                        last_name: "Paunikar".to_string(),
                         mobile_no: 1234567890,
                          age: 20,
                           date_of_birth: "10/01/2003".to_string(),
                            nationality: "Indian".to_string(),
                                train_id: 1,
                                ticket_id: "1 A3 30/04/2024".to_string(),
                                owner_addr : Addr::unchecked("payal"),
                                price : coins(2972, "eth".to_string()),
                                seat_no : "A3".to_string(),
                                date_of_travelling : "30/04/2024".to_string(),
                                ticket_type : "Sleeper".to_string(),
                                train_name: "Satabadi Express".to_string(), 
                                train_no: 123440,
            
                                  origin_destination: "nagpur".to_string(),
                                   final_destination: "pune".to_string(),
                                   origin_destination_departure_time_with_date:"30/04/2024 12:00:00".to_string(),
                                   final_destination_arrival_time_with_date:"01/05/2024 03:00:20".to_string(),
                                   total_distance_in_km : 738,


                                        
                      }
                     )                           
    }
    


    
}