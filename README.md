PESEL as a service
====

Simple HTTP REST-ish wrapper for [PESEL](https://github.com/MaciekTalaska/pesel) library written using Actix-web. 

Routes:
----

This project exposes two routes:

#### 1. PESEL validation 

`/pesel_validator/<pesel>` - which could be used for validating PESEL number.Please note that PESEL number has to be 11-characters long string consisting only numbers (otherwise you will get error message).

Example:

`/pesel_validator/80052619986` - this results in follwing return message:

```
PESEL: 80052619986
date of birth: 1980-05-26
gender: female
valid: true
```

#### 2. PESEL generation

`/pesel_generator/<year>/<mont>/<day>/gender` - this route allows generating proper PESEL number for provided date of birth and biological gender of a person

Note:
- year, month and day are to be provided as numbers
- biological gender should be provided as either `feamle` or `male`

Example:

`pesel_generator/1996/02/17/male` - this results in following return message:

```
generated pesel
PESEL: 96021778651
date of birth: 1996-02-17
gender: male
valid: true
```
