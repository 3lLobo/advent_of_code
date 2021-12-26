use std::fs;
use rand::Rng;


fn main() {
    println!("Hello, yellow Submarine!");

    let test_model = Vec::from([1,3,5,7,9,2,4,6,8,9,9,9,9,9]);
    let mut code_cout = 0;
    let cheat = true;

    'search_loop: loop {

        let model_num = gen_code(cheat);
        dbg!(&model_num);
        let my_alu = navidad(&model_num);
        if my_alu.z == 0 {
            println!("{:?}", &model_num);
            code_cout += 1;
            println!("Final z value is {}", &my_alu.z);
            
            if code_cout > 0 {
                break 'search_loop;
            }
        }

    }

}


fn gen_code(cheat: bool) -> Vec<i64> {

    let mut rng = rand::thread_rng();
    let mut my_code = Vec::new();

    for _ in 0..14 {

        my_code.push(rng.gen_range(1..10));
    }

    let cheat_vals = Vec::from([12,6,4,5,9,9,9,9,9,9,9]);
    let cheat_pos = Vec::from([0,1,2,3,4,5,7,9,10,11,12]);
    if cheat {
        for (i, n) in cheat_pos.iter().enumerate() {

            my_code[*n] = cheat_vals[i];
        }
    }
    my_code
}


fn navidad(model_num: &Vec<i64>) -> ALU {

    let mut alu = ALU {w: 0, x: 0, y: 0, z: 0, secret_code: Vec::new()};
    
    let data = read_data();
    let mut pos_count = 0;

    for line in data.lines() {

        let cmds: Vec<String> = line.clone()
                                    .split(" ")
                                    .map(|x| x.parse().unwrap())
                                    .collect();

        // dbg!(&cmds);

        if cmds[0] == "inp" {
                        
            alu.set_var(&cmds[1], model_num[pos_count]);
            pos_count += 1;
            // println!("Setting {} to Nr. {} of Barcode on position {}!", &cmds[1], &alu.w, &pos_count);

        } else if cmds[0] == "add" {

            let x = alu.match_var(&cmds[1]);
            let y = alu.match_var(&cmds[2]);
            let val = x + y;
            // println!("Pushing {} + {} to {}! New value is {}", &x, &y, &cmds[1], &val);
            alu.set_var(&cmds[1], val);

        } else if cmds[0] == "mul" {
            
            let x = alu.match_var(&cmds[1]);
            let y = alu.match_var(&cmds[2]);
            let val = x * y;
            // println!("Pushing {} * {} to {}! New value is {}", &x, &y, &cmds[1], &val);
            alu.set_var(&cmds[1], val);

        } else if cmds[0] == "div" {

            let x = alu.match_var(&cmds[1]);
            let y = alu.match_var(&cmds[2]);
            let val = x / y;
            // println!("Pushing {} / {} to {}! New value is {}", &x, &y, &cmds[1], &val);
            alu.set_var(&cmds[1], val);
            
        } else if cmds[0] == "mod" {

            let x = alu.match_var(&cmds[1]);
            let y = alu.match_var(&cmds[2]);
            let val = x % y;
            // println!("Pushing {} % {} to {}! New value is {}", &x, &y, &cmds[1], &val);
            alu.set_var(&cmds[1], val);
            
        } else if cmds[0] == "eql" {

            let x = alu.match_var(&cmds[1]);
            let y = alu.match_var(&cmds[2]);
            
            let val: i64;
            if x == y {
                val = 1;
            } else {
                val = 0;
            }

            // println!("Pushing {} == {} to {}! New value {}", &x, &y, &cmds[1], &val);
            alu.set_var(&cmds[1], val);
            
        } else {
            panic!();
        }
    }
    return alu

}


pub fn read_data() -> String {
    let data = fs::read_to_string("../input_data/input_navidad.txt").unwrap_or_else(|_| panic!("Hulp"));
    data
}


// enum AluCmd {
//     Imp {x: String},
//     Add {x: String, y: String},
//     Mul {x: String, y: String},
//     Div {x: String, y: String},
//     Mud {x: String, y: String},
//     Eql {x: String, y: String},
// }


#[derive(Debug)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    secret_code: Vec<i64>,
}


impl ALU {

    fn match_var(&self, var1: &str) -> i64 {

        match var1 {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            x => x.parse().unwrap(),
        }
    }


    fn set_var(&mut self, var1: &str, val: i64) {

        match var1 {
            "w" => self.w = val,
            "x" => self.x = val,
            "y" => self.y = val,
            "z" => self.z = val,
            _ => panic!()
        }
    }
    // fn get_cmd(cmd_vec: &Vec<String>) -> AluCmd {

    //     let cmd1: str = &cmd_vec[1]; 
    //     match &cmd1 {
    //         "imp" => AluCmd::Imp {x: cmd_vec[1]},
    //         "add" => AluCmd::Add {x: cmd_vec[1], y: cmd_vec[2] },
    //         "mul" => AluCmd::Mul {x: cmd_vec[1], y: cmd_vec[2] },
    //         "div" => AluCmd::Div {x: cmd_vec[1], y: cmd_vec[2] },
    //         "mod" => AluCmd::Mud {x: cmd_vec[1], y: cmd_vec[2] },
    //         "eql" => AluCmd::Eql {x: cmd_vec[1], y: cmd_vec[2] }

    //     }
    // }

    // fn match_cmd(&mut self, opt_cmd: AluCmd) {

    //     match opt_cmd {
    //         AluCmd::Imp {x} => self.secret_code.push(x.parse().unwrap()),
    //         AluCmd::Add { x, y } => (),
    //         AluCmd::Mul { x, y } => (),
    //         AluCmd::Div { x, y } => (),
    //         AluCmd::Mud { x, y } => (),
    //         AluCmd::Eql { x, y } => (),
    //     }
    // }

    fn imp(&self, var1: &str) -> i64 {

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
}
