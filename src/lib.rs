use inline_python::{ python, Context };


pub fn is_installed() -> bool {
    let context: Context = python! {
        try:
            import RPi.GPIO
            was_installed = True
        except Exception as e:
            was_installed = False
    };

    context.get::<bool>("was_installed")
}

pub mod gpio;

#[cfg(test)]
mod tests {
    use crate::is_installed;
    use crate::gpio;
    use std::time::Duration; // for sleep function
    use std::thread;         // for sleep function

    #[test]
    fn is_installed_test() {
        assert_eq!(is_installed(), true);
    }

    #[test]
    fn basic_on_off_test() {
        gpio::turn_on(18);
        thread::sleep(Duration::from_secs(30));
        gpio::turn_off(18);
    }

    #[test]
    fn cleanup_test() {
        match gpio::cleanup() {
            Ok(_) => {
                // holy crap is this stupid...def need to refactor this later on
                assert_eq!(true, true);
            },
            Err(msg) => {
                assert_eq!(false, true);
            }
        };
    }
}
