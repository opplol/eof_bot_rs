#[derive(Clone, Eq, PartialEq, Debug)]
enum StOrIn {
    String(String),
    u32(u32),
}
impl StOrIn {
    fn get_str(&self) -> String {
        match &self {
            StOrIn::String(n) => String::from(n),
            StOrIn::u32(_n) => String::from(""),
        }
    }
    fn get_u32(&self) -> u32 {
        match &self {
            StOrIn::u32(n) => *n,
            StOrIn::String(_n) => 0,
        }
    }
}
#[derive(Debug)]
struct MultiType {
    st_or_in: StOrIn,
    name: String,
}
#[derive(Debug)]
struct GenericStruct<T> {
    generic_v: T,
    name: String,
}

fn main() {
    let mut multi_type = MultiType {
        st_or_in: StOrIn::String("it string".to_string()),
        name: "my name".to_string(),
    };

    println!("{:?}", multi_type);
    println!("{:?}", multi_type.st_or_in);
    println!("{:?}", multi_type.st_or_in.get_str());

    multi_type.st_or_in = StOrIn::u32(30);

    println!("{:?}", multi_type);
    println!("{:?}", multi_type.st_or_in);
    println!("{:?}", multi_type.st_or_in.get_u32());

    println!("30 + 20 : {}", multi_type.st_or_in.get_u32() + 20);

    let mut gen_struct = GenericStruct {
        generic_v: String::from("iam string"),
        name: String::from("myname"),
    };

    println!("{:?}", gen_struct);
    println!("{:?}", gen_struct.generic_v);
    // gen_struct.generic_v = 32;

    println!("{:?}", gen_struct);
    println!("{:?}", gen_struct.generic_v);
}
