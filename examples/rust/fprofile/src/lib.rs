wit_bindgen_rust::export!("fprofile.wit");
use fakeit::name;
struct Fprofile;
use crate::fprofile::User;
 
impl fprofile::Fprofile for Fprofile {
 
    fn gen_users(pcount: u32) -> Vec<User> {
        let mut pfs = Vec::new();
 
        for _x in 0..11 {
            let data = User {
            first_name: name::first(),
            last_name: name::last(),
            prefix: name::prefix(),
            suffix: name::suffix(),
            };
            pfs.push(data);
        }
        return pfs;
    }
}
