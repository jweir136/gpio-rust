use inline_python::{ python, Context };

/// Returns a boolean value to indicate if the correct Raspberry PI GPIO libraries have been installed.
/// This crate will not work is the libraries are not installed. The libraries do not come with this crate
/// and need to be installed separately.
///
/// Here is an example:
/// ```
/// assert!(is_installed());
/// ```
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

/// Contains all the basic, low-level functions used to control the Raspberry PI's GPIO pins.
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
    #[cfg(target_os="linux")]
    fn basic_on_off_test() {
        gpio::turn_on(18);
        thread::sleep(Duration::from_secs(30));
        gpio::turn_off(18);
    }

    #[test]
    #[cfg(target_os="linux")]
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
