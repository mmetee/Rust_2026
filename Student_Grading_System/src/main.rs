#[derive(Debug)]
enum Grade {
    A,
    B,
    C,
    D,
    F,
}
struct Student {
    name: String,
    scores: Vec<f32>,
}
impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.to_string(),
            scores: Vec::new(),
        }
    }

    fn add_score(&mut self, score: f32) {
        self.scores.push(score);
    }

    fn calculate_average(&self) -> f32 {
        if self.scores.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.scores.iter().sum();
        sum / self.scores.len() as f32
    }

    fn get_grade(&self) -> Grade {
        let avg = self.calculate_average();
        if avg >= 80.0 {
            Grade::A
        } else if avg >= 70.0 {
            Grade::B
        } else if avg >= 60.0 {
            Grade::C
        } else if avg >= 50.0 {
            Grade::D
        } else {
            Grade::F
        }
    }
}
fn main() {
    let mut s1 = Student::new("Metee");
    s1.add_score(85.0);
    s1.add_score(73.0);
    s1.add_score(42.0);

    let avg = s1.calculate_average();
    let grade = s1.get_grade();

    println!("นักเรียน:{}", s1.name);
    println!("คะแนนเฉลี่ย: {:.2}", avg);
    println!("เกรดที่ได้: {:?}", grade);
}
