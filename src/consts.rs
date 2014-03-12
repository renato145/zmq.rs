use std::libc;

pub enum SocketType {
    //PAIR = 0,
    //PUB = 1,
    //SUB = 2,
    REQ = 3,
    //REP = 4,
    //DEALER = 5,
    //ROUTER = 6,
    //PULL = 7,
    //PUSH = 8,
    //XPUB = 9,
    //XSUB = 10,
    //STREAM = 11,
}

pub enum SocketOption {
    TYPE = 16,
}

pub static HAUSNUMERO: int = 156384712;

#[deriving(Eq, Show)]
pub enum ErrorCode {
    EINVAL = libc::EINVAL as int,
    EPROTONOSUPPORT = HAUSNUMERO + 2,
}
