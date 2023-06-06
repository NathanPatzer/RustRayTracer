extern crate num_cpus;

pub struct ArgsChecker
{
    pub width: u32,
    pub height: u32,
    pub json: String,
    pub rpp: u32,
    pub depth: i32,
    pub name: String,
    pub num_threads: u32,
    pub interpolation: bool

}

impl ArgsChecker
{
    pub fn new(args: Vec<String>) -> ArgsChecker
    {
        let max_threads = num_cpus::get();
        let mut width = 500;
        let mut height = 0;
        let mut json: String = "".to_string();
        let mut rpp: u32 = 1;
        let mut depth: i32 = 1;
        let mut name = "output".to_string();
        let mut num_threads: u32 = 1;
        let mut interpolion: bool = false;
        for i in 0..args.len()
        {
            if args[i] == "-w"
            {
                width = args[i+1].parse::<u32>().unwrap();
            }
            else if args[i] == "-h"
            {
                height = args[i+1].parse::<u32>().unwrap();
            }
            else if args[i] == "-i"
            {
                json = args[i+1].to_string();
            }
            else if args[i] == "-r"
            {
                rpp = args[i+1].parse::<u32>().unwrap();
            }
            else if args[i] == "-k"
            {
                depth = args[i+1].parse::<i32>().unwrap();
            }
            else if args[i] == "-n"
            {
                name = args[i+1].to_string();
            }
            else if args[i] == "-t"
            {
                if let Ok(parsed_threads) = args[i+1].parse::<u32>()
                {
                    num_threads = if parsed_threads > 0
                    {
                        parsed_threads
                    }
                    else{
                        1
                    };

                    if num_threads > max_threads as u32
                    {
                        num_threads = max_threads as u32;
                    }
                }
            }
            else if args[i] == "-s"
            {
                interpolion = true;
            }
        }       
         
        if height == 0
        {
            height = width;
        }
        ArgsChecker { width: width, height: height, json: json, rpp: rpp, depth: depth, name: name,num_threads: num_threads,interpolation: interpolion }
    }

    
}

pub type Args = ArgsChecker;