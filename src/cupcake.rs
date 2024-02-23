pub struct Cupcake {
    is_out: bool,
    num_made: usize,
}

impl Cupcake {
    pub fn new() -> Cupcake {
        return Cupcake {
            is_out: false,
            num_made: 0,
        };
    }
    pub fn request(&mut self) {
        self.is_out = true;
        self.num_made += 1;
    }

    pub fn eat(&mut self) {
        if !self.is_out() {
            self.request();
        }
        self.is_out = false;
    }

    pub fn get_num_eaten(&self) -> usize {
        return self.num_made - self.is_out as usize;
    }

    pub fn is_out(&self) -> bool {
        return self.is_out;
    }

    pub fn num_made(&self) -> usize {
        return self.num_made;
    }


}
