extern crate env_logger;
extern crate rand;

use std::time::{Instant};
use actix_cors::Cors;
use actix_web::{middleware, http, get, post, web::{Json}, App,  HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_files::Files;
use rand::Rng;

use rubik_lib::rubik::cubie_cube::{CubieCube};
use rubik_lib::pruning::pruning::{Pruning};
use rubik_lib::algo::solve::*;
use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};
use rubik_lib::pruning::moves::{Moves};

mod parsing;

use parsing::parse::{parse_inputs};

#[derive(Deserialize)]
struct Request {
    sequence: String,
}

#[derive(Serialize)]
struct Response {
    solution: String,
    status: String,
}

#[get("/scramble")]
async fn scramble() -> impl Responder {
    println!("Request from /scramble");
    let input_sequence: Vec<usize> = (0..15).map(|_| rand::thread_rng().gen_range(0, 17)).collect();
    let shuffle: String = input_sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" ").to_owned();
    HttpResponse::Ok().body(shuffle)
}

#[post("/solver")]
async fn solver(req: Json<Request>) -> impl Responder {
    println!("Request from /solver: {:?}", req.sequence);
    
    match parse_inputs(&req.sequence) {
        Ok(input_sequence) => {
            let pruning_tables: Pruning = Pruning::new();
            let moves_tables: Moves = Moves::new();
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            cb_cube.apply_sequence(&input_sequence);
            let start_time: std::time::Instant = Instant::now();
            match solve(&mut cb_cube, &pruning_tables, &moves_tables, start_time) {
                Ok(s) => {
                    println!("solution: {}", s.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
                    println!("duration: {:?}", start_time.elapsed());
                    return HttpResponse::Ok().json(Response {
                        status: "Ok".to_string(),
                        solution: s.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" ").to_owned(),
                    })
                },
                Err(error) => {
                    println!("error: {}", error.to_string());
                    return HttpResponse::InternalServerError().json(Response {
                        status: error.to_string(),
                        solution: "".to_string(),
                    })
                }
            }
        },
        Err(error) => {
            println!("error: {}", error.to_string());
            return HttpResponse::InternalServerError().json(Response {
                status: error.to_string(),
                solution: "".to_string(),
            })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    print_launch_resume();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                .allowed_origin("https://rubik.nicolasvienot.co")
                .allowed_origin("http://localhost:8080")
                .allowed_origin("http://0.0.0.0:8080")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish())
            .service(scramble)
            .service(solver)
            .service(Files::new("/", "./public").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub fn print_launch_resume() {
    let ascii_name = r#" *******           **      ** **     **        
/**////**         /**     // /**    //*        
/**   /**  **   **/**      **/**  ** /   ******
/*******  /**  /**/****** /**/** **     **//// 
/**///**  /**  /**/**///**/**/****     //***** 
/**  //** /**  /**/**  /**/**/**/**     /////**
/**   //**//******/****** /**/**//**    ****** 
//     //  ////// /////   // //  //    //////  "#;
    eprintln!("{}", ascii_name);
    eprintln!();
    eprintln!("Server listening on:\t{:?}", "0.0.0.0:8080");
    eprintln!();
    eprintln!("Source code:\t\thttps://github.com/sgalasso42/rubik");
    eprintln!("Contact:\t\thttps://github.com/sgalasso42/rubik or bonjour@rubik.com");
    eprintln!();
}