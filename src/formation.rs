
use chrono::NaiveDate;

pub struct Object {
    id : i64,

    name : String,
    code : String,
    program : String,

    place : String,

    date : [chrono::NaiveDate ; 5],
    duration : i8,


}