#![crate_name = "r2d2_beanstalkd"]

pub mod pool;

#[cfg(test)]
mod tests {
    use crate::pool::BeanstalkdConnectionManager;
    use std::time::Duration;

    #[test]
    fn connect() {
        let m = BeanstalkdConnectionManager::new("localhost", 11300);
        let pool = r2d2::Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::new(10, 0))
            .build(m)
            .expect("Connection failed");

        let mut c = pool.get().expect("Could not get connection");
        let tubes = c.tubes().expect("Could not retrieve tubes");

        assert_eq!(tubes.contains(&"default".to_string()), true);
    }

    #[test]
    #[should_panic]
    fn fail_connect() {
        let m = BeanstalkdConnectionManager::new("localhost", 11500);
        let pool = r2d2::Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::new(10, 0))
            .build(m)
            .expect("Connection failed");

        let _c = pool.get().expect("Could not get connection");
    }
}
