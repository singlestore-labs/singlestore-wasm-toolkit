// export! will load usergenerator.wit and generate Wasi compatible bindings
wit_bindgen_rust::export!("usergenerator.wit");

// From the bindings of the wit file, pull the user structure. This will be used
// later to generate multiple user profiles
use crate::usergenerator::User;

// Pull in fakeit modules for name, contact, password, unique and datetime
use fakeit::{contact, datetime, name, password, unique};

// define a struct to be the concrete implementation of the wit interface
struct Usergenerator;

// implement the gen_users method of our wit interface (usergenerator.wit)
impl usergenerator::Usergenerator for Usergenerator {
    // Our first function of our package! gen_users takes in a integer (a u32 in
    // this case) and will output a vector with the User type we pulled from the
    // wit file above
    fn gen_users(count: u32) -> Vec<User> {
        // We instantiate the vector to be returned
        let mut users = Vec::new();

        // Iterate from 0 to count for each user we want to create
        for _x in 0..count {
            let user = User {
                // See https://crates.io/crates/fakeit for all the fakeit types
                uid: unique::uuid_v4().to_string(),
                created: datetime::date().to_string(),
                first_name: name::first().to_string(),
                last_name: name::last().to_string(),
                email: contact::email().to_string(),
                passwd: password::generate(true, true, true, 52).to_string(),
            };

            // push the user into the vector
            users.push(user);
        }

        // Return the list of users
        return users;
    }
}
