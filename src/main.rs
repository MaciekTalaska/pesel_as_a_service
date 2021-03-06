use actix_web::{web, App, HttpServer, Responder};
use pesel::pesel::*;
// use pesel::pesel_parsing_error::*;
use std::str::FromStr;

fn pesel_check(info: web::Path<String>) -> impl Responder {
	let result = PESEL::from_str(&info);

	match result {
		Ok(parsed_pesel) => format!("{}",parsed_pesel),
		_ => "unable to parse PESEL".to_string()
	}
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
	let server_address = "127.0.0.1:8080";
	println!("running server on: http://{0}", server_address);
	HttpServer::new(||
		{App::new()
		.service(web::resource("/pesel_generator/{year}/{month}/{day}/{gender}").to(generate_pesel))
		.service(web::resource("/pesel_validator/{pesel}").to(pesel_check))
	})
	.bind(server_address)?
	.run()
}
