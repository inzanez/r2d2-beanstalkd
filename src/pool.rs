use beanstalkc::{Beanstalkc, BeanstalkcError};
use r2d2;
use std::result::Result;
use std::time;

#[derive(Clone, Debug)]
pub struct BeanstalkdConnectionManager {
    host: String,
    port: u16,
}

impl BeanstalkdConnectionManager {
    pub fn new<S: Into<String>>(host: S, port: u16) -> BeanstalkdConnectionManager {
        BeanstalkdConnectionManager {
            host: host.into(),
            port,
        }
    }
}

impl r2d2::ManageConnection for BeanstalkdConnectionManager {
    type Connection = Beanstalkc;
    type Error = BeanstalkcError;

    fn connect(&self) -> Result<Beanstalkc, BeanstalkcError> {
        Beanstalkc::new()
            .host(&self.host)
            .port(self.port)
            .connection_timeout(Some(time::Duration::from_secs(10)))
            .connect()
    }

    fn is_valid(&self, conn: &mut Beanstalkc) -> Result<(), BeanstalkcError> {
        conn.tubes().map(|_| ())
    }

    fn has_broken(&self, conn: &mut Beanstalkc) -> bool {
        conn.tubes().is_err()
    }
}
