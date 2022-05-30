use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;



pub struct CStream {  // do i make this pub??
	filename: String,
	// line_num: i32,
	// char_pos: i32,
	content: String, // contains the file
	// overall_pos: i32,
	size: i32
}


impl CStream {
    pub fn new(f: &str) -> CStream {
        CStream {
            filename: f.to_string(),
			// line_num: -1,
			// char_pos: -1,
            content: String::new(),
            // overall_pos: -1,
            size: 0
        }
    }

    fn set_content(&mut self) -> io::Result<()> {
        let file = File::open(self.filename.as_str())?;     // open the file
        let mut buf_reader = io::BufReader::new(file);      // creates a buffer that contains the file ??
        buf_reader.read_to_string(&mut self.content);       // reads the contents of the buffer into a string and stores in content
        self.size = self.content.chars().count() as i32;    // gets the number of chars in content
        Ok(())
    }

    pub fn get_content(mut self) -> String {
        self.content
    }

    pub fn get_size(mut self) -> i32 {
        self.size
    }
}

pub fn run() -> String {
    let args: Vec<String> = env::args().collect();
    // if args.len() != 1 {
    //     panic!("too many arguments. only need one")
    // }
    let filename = &args[1];
    let mut ex = CStream::new(filename);
	ex.set_content();
    ex.get_content()
    println!("{}", ex.content);
}
