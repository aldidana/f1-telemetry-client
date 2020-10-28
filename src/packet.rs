use crate::f1_2020::packet::Packet2020;

#[derive(Debug, PartialEq)]
pub enum Packet {
    F12020(Packet2020),
    F12019,
    F12018,
    NONE,
}
