// wit_bindgen_rust::export!("fprofile.wit"); is used to export the "bindings" for the following rust example
wit_bindgen_rust::export!("usergenerator.wit");

// From the bindings of the wit file, pull the User structure. This will be used later to generate multiple user profiles
use crate::usergenerator::User;

// Pull in the crate package "fakeit" and the modules for name, contact, password, unique and datetime
extern crate fakeit;
use fakeit::name;
use fakeit::contact;
use fakeit::datetime;
use fakeit::unique;
use fakeit::password;
// Another way to do this would be use fakeit::{name,contact,datetime,unique,password};

// We create the "Usergenerator" structure as a module pointer
struct Usergenerator;

// "Impl" is used to generate the "implementation types" of a module. Using the struct we created above, were going to iterate and implement each of these functions for our "fprofile" see more here - https://doc.rust-lang.org/std/keyword.impl.html
impl usergenerator::Usergenerator for Usergenerator {

    // Our first function of our package! gen_users takes in a integer (a u32 in this case) and will output a vector with the User type we pulled from the wit file above
    fn gen_users(pcount: u32) -> Vec<User> {
        // We instantiate the vector to be returned
        let mut pfs = Vec::new();

        // Iterate from 0 to pcount for each profile we want to create
        for _x in 0..pcount {
            let data = User {
                // See https://crates.io/crates/fakeit for all the fakeit types
                uid: unique::uuid_v4().to_string(),
                created: datetime::date().to_string(),
                first_name: name::first().to_string(),
                last_name: name::last().to_string(),
                email: contact::email().to_string(),
                passwd: password::generate(true, true, true, 52).to_string()
            };

            // For each of these items, we push the data to the pfs vector
            pfs.push(data);
        }

        // Return the pfs data structure after completing
        return pfs;
    }
}