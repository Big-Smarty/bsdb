use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Eq, Hash)]
pub struct Member {
    pub name: String,
    pub value: String,
}

pub struct Object {
    pub name: String,
    pub members: HashMap<String, Member>,
}

pub struct DataBase {
    pub name: String,
    pub objects: HashMap<String, Object>,
}

impl Member {
    pub fn new(n: String, v: String) -> Self {
        Member { name: n, value: v }
    }

    pub fn as_string(&self) -> String {
        format!("{}: {}", &self.name, &self.value).to_string()
    }
}

impl Object {
    pub fn new(n: String) -> Self {
        Object {
            name: n,
            members: HashMap::new(),
        }
    }

    pub fn from_string(s: String) -> Self {
        let mut out = Object {
            name: String::new(),
            members: HashMap::new(),
        };
        let mut object_name = String::new();

        for l in s.lines() {
            if l.contains("{") {
                object_name = l.split_whitespace().next().unwrap().to_string();
                continue;
            }
            if l.contains(":") {
                let mut member_iter = l.split(":").into_iter();
                let member_name = member_iter.next().unwrap();
                let member_value = member_iter.next().unwrap();

                out.add_member(Member::new(
                    member_name.to_string(),
                    member_value.to_string(),
                ));
                continue;
            }
        }

        out.name = object_name;
        out
    }

    pub fn add_member(&mut self, m: Member) {
        self.members.insert(m.name.clone(), m);
    }

    pub fn print_members(&self) {
        for i in self.members.iter() {
            println!("{}", i.0);
        }
    }

    pub fn as_string(&self) -> String {
        let mut out = String::new();
        out.push_str("@Object\n");
        out.push_str(&format!("{} {}\n", &self.name, "{"));
        for i in self.members.iter() {
            out.push_str(&format!("\t{}\n", i.1.as_string()));
        }
        out.push_str("}\n");
        out
    }
}

impl DataBase {
    pub fn new(n: String) -> Self {
        DataBase {
            name: n,
            objects: HashMap::new(),
        }
    }

    pub fn from_file(&mut self, path: String) -> Self {
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);

        let mut object_strings: Vec<String> = Vec::new();
        let mut inside_object: bool = false;
        let mut object_count: usize = 0;

        for line in buffered.lines() {
            let l = line.unwrap();

            if inside_object {
                object_strings[object_count].push_str(&format!("{}\n", l.clone()));
            }

            if l.contains("@") {
                inside_object = true;
                object_strings.push(String::new());
            }

            if l.contains("}") {
                inside_object = false;
                object_count += 1;
            }
        }

        for i in object_strings {
            let temp_object = Object::from_string(i);
            self.objects.insert(temp_object.name.clone(), temp_object);
        }

        DataBase::new(String::new())
    }

    pub fn add_object(&mut self, o: Object) {
        self.objects.insert(o.name.clone(), o);
    }

    pub fn print_objects(&self) {
        for i in self.objects.iter() {
            println!("{}", i.0,)
        }
    }

    pub fn as_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("//{}\n\n", self.name));
        for i in self.objects.iter() {
            out.push_str(&format!("{}\n", i.1.as_string()));
        }

        out
    }

    pub fn print_to_file(&self, path: Option<String>) {
        let mut file = {
            if path.is_none() {
                File::create(format!("{}.bsdb", self.name.clone())).unwrap()
            } else {
                File::create(format!("{}.bsdb", path.unwrap())).unwrap()
            }
        };
        write!(file, "{}", &self.as_string()).unwrap();
    }
}
