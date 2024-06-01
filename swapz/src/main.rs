#[derive(Debug)]
struct Vehicle<T,X>{
    customer_one:T,
    customer_two:X
}
impl <T,X> Vehicle<T,X>{
    fn new(customer_one : T, customer_two : X )-> Self {
        Vehicle{customer_one,customer_two }
    }
    fn swap(& mut self, other_client: Vehicle<T,X>){
        self.customer_one = other_client.customer_two
    }
}

fn main(){
    let x : Vehicle<&str, String> = Vehicle::new(customer_one:"KYC", "KDD".to_string());
    let x : Vehicle<&str, String> = Vehicle::new(customer_one:"KAZ", "GK01".to_string());

    x.swap

}