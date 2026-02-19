use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::usize;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}
struct Todolist {
    tasks: Vec<Task>,
}
impl Todolist {
    fn new() -> Todolist {
        Todolist { tasks: Vec::new() }
    }
    fn add_task(&mut self, desc: &str) {
        let new_task = Task {
            description: desc.to_string(),
            completed: false,
        };
        self.tasks.push(new_task);
    }
    fn show_tasks(&self) {
        if self.tasks.is_empty() {
            println!("no task in list");
            return;
        }
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[X]" } else { "[ ]" };
            println!("{} {} {}", i + 1, status, task.description);
        }
    }
    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(file, "{}|{}", task.description, task.completed)?;
        }
        Ok(())
    }
    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let desc = parts[0].to_string();
                let completed = parts[1].parse().unwrap_or(false);
                self.tasks.push(Task {
                    description: desc,
                    completed,
                });
            }
        }
        Ok(())
    }
    fn mark_done(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            let target_index = index - 1;
            self.tasks[target_index].completed = true;
            println!(
                "Congrat! you complete your {}",
                self.tasks[target_index].description
            );
        } else {
            println!("Sorry can't found in your {}", index);
        }
    }
}
fn main() {
    let mut my_list = Todolist::new();
    let filename = "task.txt";
    let _ = my_list.load_from_file(filename);

    loop {
        println! {"\n ------TO DO LIST-------"};
        my_list.show_tasks();
        println!("\n เลือกทำรายการ");
        println!("1.เพิ่มรายการใหม่");
        println!("2.ทำงานสำเร็จแล้ว(mark done)");
        println!("3.บัยทึกออกจากโปรแกรม");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("อ่านค่าไม่ได้");
        let choice = choice.trim();
        match choice {
            "1" => {
                println!("พิมพ์ชื่องานที่ต้องการเพิ่ม");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).expect("อ่านค่าไม่ได้");
                my_list.add_task(desc.trim());
            }
            "2" => {
                println!("ใส่ 'เลขลำดับงาน' ที่ทำเสร็จแล้ว");
                let mut idx_str = String::new();
                io::stdin().read_line(&mut idx_str).expect("อ่านค่าไม่ได้");
                if let Ok(num) = idx_str.trim().parse::<usize>() {
                    my_list.mark_done(num);
                } else {
                    println!("กรุณาใส่ตัวเลขที่ถูกต้อง");
                }
            }
            "3" => {
                let _ = my_list.save_to_file(filename);
                println!("บันทึกข้อมูลเรียบร้อย");
                break;
            }
            _ => println!("เลือกเมนู ไม่ถูกต้อง กรุณาลองใหม่"),
        }
    }
}
