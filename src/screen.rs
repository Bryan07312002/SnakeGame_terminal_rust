extern crate clearscreen;
extern crate term_size;

pub struct Screen<'a> {
    pub width:usize,
    pub height:usize,
    pub grid: Vec<Vec<&'a char>>,
    pub screen: String
}

impl Screen <'static>{
    pub fn new() -> Screen<'static>{
        let width:usize;
        let height:usize;
        let mut width_vec: Vec<&char>;
        let mut height_vec:Vec<Vec<&char>>;
        let screen_f: String;
        let mut line: Vec<&char> = Vec::new();

        if let Some((w, h)) = term_size::dimensions() {
            width = w;
            height = h - 4;
        } else {
            width = 0;
            height = 0;
        };

        width_vec = Vec::new();
        height_vec = Vec::new();
        
        //grid
        for i in 0..width{
            width_vec.push(&' ');
        }


        for i in 0..width{
            line.push(&'-');
        }


        height_vec.push(line.clone());
        for _i in 1..height-1 {
            height_vec.push(width_vec.clone());
        }
        height_vec.push(line.clone());

        screen_f = Screen::build_screen(height_vec.clone());

        // Screen
        height_vec.reverse();
        return Screen { width: width, height: height, grid: height_vec, screen:screen_f };
    }

    pub fn update_screen(&mut self){

        let mut width_vec;
        let mut height_vec;

        height_vec = Vec::new();
        
        //grid
        for i in 0..self.height-1 {
            width_vec = self.grid[i].clone();
            height_vec.push(width_vec.clone());
        }

        self.screen = Screen::build_screen(self.grid.clone());
    }

    pub fn build_screen(height_vec:Vec<Vec<&char>>) -> String {
        let mut screen_vec: Vec<String> = Vec::new();
        let mut screen_x: String = String::from("");
        let mut screen_f: String = String::from("");

        for i in height_vec{
            screen_x = String::from("");
            for z in i {
                let char_txt = z.to_string();
                screen_x.push_str(&char_txt);
            }

            screen_vec.push(screen_x.clone())
        }

        for i in screen_vec {
            screen_f.push_str(&i)
        }

        screen_f
    }

    pub fn get_center(&self) -> (usize, usize){
        let w = self.width.clone() as f32;
        let h = self.height.clone() as f32;
        ((w/2.0).floor() as usize, (h/2.0).ceil() as usize)
    }
    
    pub fn draw_char_at(&mut self, x:usize , y: usize, char_icon:char) {
        self.grid[y][x] = &'0';
    }
}
