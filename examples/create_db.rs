use bsdb::{self, DataBase};

fn main() {
    let mut db: bsdb::DataBase = DataBase::new("ExampleDatabase".to_string());
    db.add_object(bsdb::Object::new("ExampleObject".to_string()));
    db.objects
        .get_mut("ExampleObject")
        .unwrap()
        .add_member(bsdb::Member::new(
            "ExampleMember".to_string(),
            "ExampleValue".to_string(),
        ));
    db.add_object(bsdb::Object::new("ExampleObject2".to_string()));
    db.objects
        .get_mut("ExampleObject2")
        .unwrap()
        .add_member(bsdb::Member::new(
            "ExampleMember".to_string(),
            "ExampleValue".to_string(),
        ));
    db.objects
        .get_mut("ExampleObject2")
        .unwrap()
        .add_member(bsdb::Member::new(
            "ExampleMember2".to_string(),
            "ExampleValue2".to_string(),
        ));

    println!("{}", db.as_string());
    db.print_to_file(None);

    println!("From file:");
    db.from_file("ExampleDatabase.bs".to_string());
    println!("{}", db.as_string());
}
