

pub trait ToString {
    fn to_string(&mut self)->String;
}

impl ToString for std::vec::Vec<char> {
    fn to_string(&mut self)->String{
        self.into_iter().map(|i| i.to_string()).collect::<String>()
    }
}