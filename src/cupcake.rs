
pub struct Cupcake {
    is_out: bool,
    num_made: u64,
}

impl Cupcake {
    fn request(&mut self){
        self.is_out = true;
        self.num_made += 1;
    }

    fn eat(&mut self){
        self.is_out = false;
    }

    fn get_num_eaten(&self) -> u64{
        return self.num_made - self.is_out as u64; 
    }

}
