use std::io::SeekFrom;


struct User <'a> {
    username: String,
    email: String,
    books: &'a[u32],
    sign_in_count: u64,
    active: bool,
}

impl<'a> User<'a> {
    fn new(username:impl Into<String>,email:impl Into<String>,books:&'a [u32] )->Self{
        Self { username: username.into(), email: email.into(),books, sign_in_count: 0, active: true }
    }
}

fn main(){
    let user ;
    
         let books= vec![1,2,3];
        user =User::new("ynk", "3343", &books);
    
    print!("{}",user.email);
}