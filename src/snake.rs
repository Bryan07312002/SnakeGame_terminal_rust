pub struct Snake<'a> {
    pub x:usize,
    pub y:usize,
    pub char:&'a char
}

impl Snake <'static> {
    pub fn new(center: (usize,usize)) -> Snake <'static>{
        Snake {
            x:center.1,
            y:center.0,
            char:&'1'
        }
    }

    pub fn down(&mut self) {
        let x = self.x;
        if x - 1 > 100 {
            return
        }
    
       self.x = self.x - 1;
    }
}
