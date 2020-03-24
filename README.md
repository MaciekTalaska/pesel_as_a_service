PESEL as a service
====

Simple HTTP REST-ish wrapper for [PESEL](https://github.com/MaciekTalaska/pesel) library written using Actix-web. 

Routes:
----

This project exposes two routes:

`/pesel_validator/<pesel>` - which could be used for validating PESEL number.Please note that PESEL number has to be 11-characters long string consisting only numbers (otherwise you will get error message).

`/pesel_generator/<year>/<mont>/<day>/gender` - this route allows generating proper PESEL number for provided date of birth and biological gender of a person
