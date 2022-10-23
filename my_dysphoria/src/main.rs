use chrono::Datelike;

fn main() {
    let date = chrono::Utc::now();
    let year: i32 = date.year();
    let month: u32 = date.month();
    let age: i32;
    if month < 12 {
        age = year - 2004;
    } else {
        age = year - 2003;
    }
    println!("{}", age);
    let shit_tanmay: Person = Person {  gender: Gender::Male,
                                        age,
                                        name: String::from("Tanmay Deshpande") };
    let gay_tanmay: Person = Person {  gender: Gender::Other(OtherGenderKind::Gay),
                                        age,
                                        name: String::from("Tammy Deshpande") };
    let pansexual_tanmay: Person = Person {  gender: Gender::Other(OtherGenderKind::Pansexual),
                                        age,
                                        name: String::from("Tanmayee Deshpande") };
    let female_tanmay: Person = Person {  gender: Gender::Female,
                                        age,
                                        name: String::from("Tanvy Deshpande") };
    let lesbian_tanmay: Person = Person {  gender: Gender::Other(OtherGenderKind::Lesbian),
                                        age,
                                        name: String::from("Tanvi Deshpande") };
    println!("--------------------------");
    println!("{} has a value of {} because it is a pig", shit_tanmay.name, shit_tanmay.get_value());
    println!("{} has a value of {} because they are {}", gay_tanmay.name, gay_tanmay.get_value(), gay_tanmay.get_gender());
    println!("{} has a value of {} because they are {}", pansexual_tanmay.name, pansexual_tanmay.get_value(), pansexual_tanmay.get_gender());
    println!("{} has a value of {} because she is a {}", female_tanmay.name, female_tanmay.get_value(), female_tanmay.get_gender());
    println!("{} has a value of {} because she is a princess", lesbian_tanmay.name, lesbian_tanmay.get_value());
    println!("--------------------------");
    println!("{} is at least {} times better than {}", gay_tanmay.name, gay_tanmay.get_value()/shit_tanmay.get_value(), shit_tanmay.name);
    println!("{} is at least {} times better than {}", pansexual_tanmay.name, pansexual_tanmay.get_value()/gay_tanmay.get_value(), gay_tanmay.name);
    println!("{} is at least {} times better than {}", female_tanmay.name, female_tanmay.get_value()/pansexual_tanmay.get_value(), pansexual_tanmay.name);
    println!("{} is at least {} times better than {}", lesbian_tanmay.name, lesbian_tanmay.get_value()/female_tanmay.get_value(), female_tanmay.name);
    println!("{} is at least {} times better than {}", lesbian_tanmay.name, lesbian_tanmay.get_value()/shit_tanmay.get_value(), shit_tanmay.name);
    println!("--------------------------");
    println!("{} is missing out", shit_tanmay.name);
    println!("--------------------------");
}
enum OtherGenderKind {
    Gay,
    Pansexual,
    Lesbian,
}
enum Gender {
    Male,
    Female,
    Other(OtherGenderKind),
}
struct Person {
    gender: Gender,
    age: i32,
    name: String,
}
impl Person {
    fn get_gender (&self) -> String {
        match &self.gender {
            Gender::Male => String::from("Male"),
            Gender::Other(kind) => {
                match kind {
                    OtherGenderKind::Gay => String::from("Gay"),
                    OtherGenderKind::Pansexual => String::from("Pansexual"),
                    OtherGenderKind::Lesbian => String::from("Lesbian"),
                }
            }
            Gender::Female => String::from("Woman"),
        }
    }
    fn get_value (&self) -> f64 {
        let gender_value: i32;
        let age_value: i32;
        match &self.gender {
            Gender::Female => {
                gender_value = 2;
                if self.age <= 50 {
                    age_value = 50 + self.age;
                } else {
                    age_value = 150 - self.age;
                }
            },
            Gender::Male => {
                gender_value = 0;
                age_value = self.age / 4;
            },
            Gender::Other(kind) => {
                match kind {
                    OtherGenderKind::Gay => {
                        gender_value = 0;
                        age_value = self.age / 2;
                    },
                    OtherGenderKind::Pansexual => {
                        gender_value = 1;
                        if self.age <= 50 {
                            age_value = 50 + self.age;
                        } else {
                            age_value = 150 - self.age;
                        }
                    },
                    OtherGenderKind::Lesbian => {
                        gender_value = 3;
                        if self.age <= 75 {
                            age_value = 75 + self.age;
                        } else {
                            age_value = 175 - self.age;
                        }
                    }
                }
            }
        }
        (gender_value * self.age + age_value) as f64
    }
}
