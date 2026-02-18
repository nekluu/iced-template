#[derive(Clone, Copy, Debug)]
pub enum Message {
    Start,
    Data(usize),
}
