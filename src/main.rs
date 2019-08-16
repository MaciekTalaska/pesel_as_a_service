use actix_web::{web, App, HttpServer, Responder};
use pesel::pesel::*;
use pesel::pesel_parsing_error::*;
use std::str::FromStr;

fn index(info: web::Path<(u32, String)>) -> impl Responder {
	format!("Hello {}! id: {}", info.1, info.0)
}

fn pesel_check(info: web::Path<String>) -> impl Responder {
	let result = PESEL::from_str(&info);

	let pesel = match result {
		Ok(parsed_pesel) => parsed_pesel,
		_ => panic!("unable to parse PESEL") 
	};

	format!("{}", pesel)
}


fn generate_pesel(info: web::Path<(u16, u8, u8, String)>) -> impl Responder {
    let gender = match (info.3).as_ref() {
		"male" => PeselGender::Male,
		"female" => PeselGender::Female,
		_ => panic!("error while processing request!"),
	};

	let year = info.0;
	let month = info.1;
	let day = info.2;

	let pesel = PESEL::new(year, month, day, gender);
	format!("generated pesel\n{}", pesel)
}

fn main() -> std::io::Result<()> {
	//HttpServer::new(||App::new().service(web::resource("/{id}/{name}").to(index)))
	// HttpServer::new(||App::new().service(web::resource("/{pesel}").to(pesel_check)))
	HttpServer::new(||
		{App::new()
		.service(web::resource("/{year}/{month}/{day}/{gender}").to(generate_pesel))
		.service(web::resource("/{id}/{name}").to(index))
		.service(web::resource("/{pesel}").to(pesel_check))
	})
	.bind("127.0.0.1:8080")?
	.run()
}
