use std::fs;


fn main() {
    println!("Hello, yellow Submarine!");

    let alu = ALU {w: 0, x: 0, y: 0, z: 0};

    println!("{:#}", &alu.x);
    
    let data = read_data();

    for line in data.lines() {

        let cmds = line.clone().split(" ");

        for cmd in cmds {
            println!("{:#?}", &cmd);
        }

        // match cmds.next() {
        //     std::option::Option::from("imp") => alu.imp(cmds.next()),
        //     "add" => alu.add(cmds.next(), cmds.next()),
        // }
    }

}


pub fn read_data() -> String {
    let data = fs::read_to_string("../input_data/input_navidad.txt").unwrap_or_else(|_| panic!("Hulp"));
    data
}


struct ALU {
    w: i32,
    x: i32,
    y: i32,
    z: i32
}


impl ALU {

    fn imp(&self, var1: char) -> i32 {

        if var1 == 'w' {
            return self.w;
        } else if var1 == 'x' {
            return self.x;
        } else if var1 == 'y' {
            return self.y;
        } else if var1 == 'z' {
            return self.z;
        } else {
            return self.w;
        }
    }

    fn add(var1: char, var2: char) {
        return
    }

    fn mul(var1: char, var2: char) {
        return
    }

    fn mud(var1: char, var2: char) {
        return
    }

    fn div(var1: char, var2: char) {
        return
    }

    fn eql(var1: char, var2: char) {
        return
    }
}
