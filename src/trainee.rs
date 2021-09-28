

pub struct Trainee {
    id : i64,
    first_name : String,
    last_name  : String,

    address : String,
    zip     : String,
    city    : String,

}

pub fn new(id : i64, first_name : String, last_name : String, address : String, zip : String, city : String) -> Trainee {
    Trainee{ id, first_name, last_name, address, zip, city, }
}

impl Trainee {


    pub fn print(&self) {
        println!("Trainee {} -- {} {} -- {} {} {}", self.id,
                 self.first_name, self.last_name,
                 self.address   , self.zip      , self.city)
    }
}
