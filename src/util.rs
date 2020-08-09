/*
 * The "Print Unwrap" macro simply performs .unwrap(),
 * but prints errors to stdout and performs some control
 * flow (return, continue, break) instead of panicing.
 */
#[macro_export]
macro_rules! punwrap_r {
    ($e:expr, continue) => {
      match $e {
        Ok(val) => val,
        Err(e) => {
          println!("{}:{} e={}", file!(), line!(), e);
          continue;
        }
      }
    };
    ($e:expr, break) => {
      match $e {
        Ok(val) => val,
        Err(e) => {
          println!("{}:{} e={}", file!(), line!(), e);
          break;
        }
      }
    };
    ($e:expr, return) => {
      match $e {
        Ok(val) => val,
        Err(e) => {
          println!("{}:{} e={}", file!(), line!(), e);
          return;
        }
      }
    };
    ($e:expr, nothing) => { // deprecated
      match $e {
        Ok(_val) => (),
        Err(e) => {
          println!("{}:{} e={}", file!(), line!(), e);
        }
      }
    };
    ($e:expr) => {
      match $e {
        Ok(_val) => (),
        Err(e) => {
          println!("{}:{} e={}", file!(), line!(), e);
        }
      }
    };
}
