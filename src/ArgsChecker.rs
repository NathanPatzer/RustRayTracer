
pub struct ArgsChecker
{
    pub width: u32,
    pub height: u32,
    pub json: String,
    pub rpp: u32,
    pub depth: i32,
    pub name: String,
    pub interpolation: bool

}

impl ArgsChecker
{
    pub fn new(args: Vec<String>) -> ArgsChecker
    {
        let mut width = 500;
        let mut height = 0;
        let mut json: String = "".to_string();
        let mut rpp: u32 = 1;
        let mut depth: i32 = 1;
        let mut name = "output".to_string();
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
            else if args[i] == "-s"
            {
                interpolion = true;
            }
        }       
         
        if height == 0
        {
            height = width;
        }
        ArgsChecker { width: width, height: height, json: json, rpp: rpp, depth: depth, name: name,interpolation: interpolion }
    }

    
}

pub type Args = ArgsChecker;