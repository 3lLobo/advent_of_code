use std::fs;
use std::fmt;


fn main() {
    println!("Hello, yellow Submarine!");

    let alu = ALU {w: 0, x: 0, y: 0, z: 0, secret_code: Vec::new()};

    // println!("{:#}", &alu.x);
    
    let data = read_data();

    for line in data.lines() {

        // let cmds = line.split(" ")
        //                 .map(|x| x.parse::<str>().unwrap())
        //                 .collect();

        let cmds: Vec<String> = line.clone().split(" ").map(|x| x.parse().unwrap()).collect();

        dbg!(&cmds);

        for cmd in cmds {
            println!("{:#?}", &cmd);
        }

        // match cmds.next() {
        //     Some("imp") => alu.imp(&cmds.next().unwrap()),
        //     Some("add") => alu.add(&cmds.next().unwrap(), &cmds.next().unwrap()),
        // };
    }

}


pub fn read_data() -> String {
    let data = fs::read_to_string("../input_data/input_navidad.txt").unwrap_or_else(|_| panic!("Hulp"));
    data
}


enum AluCmd {
    Imp {x: String},
    Add {x: String, y: String},
    Mul {x: String, y: String},
    Div {x: String, y: String},
    Mud {x: String, y: String},
    Eql {x: String, y: String},
}


#[derive(Debug)]
struct ALU {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    secret_code: Vec<i32>,
}


impl ALU {

    fn match_var(&self, var1: &str) -> i32 {

        match var1 {
            "w" => self.w,
            "x" => self.x,
            x => x.parse().unwrap(),
        }
    }

    fn match_cmd(&mut self, opt_cmd: AluCmd) {

        match opt_cmd {
            AluCmd::Imp {x} => self.secret_code.push(x.parse().unwrap()),
            AluCmd::Add { x, y } => (),
            AluCmd::Mul { x, y } => (),
            AluCmd::Div { x, y } => (),
            AluCmd::Mud { x, y } => (),
            AluCmd::Eql { x, y } => (),
        }
    }

    fn imp(&self, var1: &str) -> i32 {

        if var1 == "w" {
            return self.w;
        } else if var1 == "x" {
            return self.x;
        } else if var1 == "y" {
            return self.y;
        } else if var1 == "z" {
            return self.z;
        } else {
            return self.w;
        }
    }

    fn add(&self, var1: &str, var2: &str) -> i32 {
        let var1 = var1;
        println!("{} + {}", &var1, &var2);
        return var1.parse::<i32>().unwrap();
    }

    fn mul(&self, var1: &str, var2: &str) {
        return
    }

    fn mud(&self, var1: &str, var2: &str) {
        return
    }

    fn div(&self, var1: &str, var2: &str) {
        return
    }

    fn eql(&self, var1: &str, var2: &str) {
        return
    }
}
