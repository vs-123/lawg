# lawg

lawg is a Rust library designed to log and create log files with ease.

# Example

```rs
use lawg::Logger;

fn main() {
    let logger = Logger::new("General Logger", Some("../logs/general/logs.txt"), true);

    logger.log("Started"); // My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: Started
    logger.log_to_file("Started again");

    let mut x = 1 + 1;

    if x != 2 {
        logger.log_and_log_to_file("It is two"); // My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: It is two
    } else {
        logger.error_and_stop("1 + 1 is not two"); // ERROR: My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: 1 + 1 is not two
    }
}
```
