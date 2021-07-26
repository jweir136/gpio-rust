use inline_python::{ Context, python };

/// A stand-alone function used to turn on a specified GPIO pin on the Raspberry PI.
/// Once a pin is turned on, it starts emitting an electrical current.
/// If you attempt to turn on a pin that is already on then nothing will happen.
/// 
/// This function takes an unsigned 8-bit integer as an argument. The argument is used to specify
/// which pin on the PI to turn on.
///
/// Returns a Result type which is either empty or contains a String datatype. The String may contain
/// a an error message to aid in debugging.
///
/// Example:
/// ```
/// match turn_on(1) {
///     Ok(()) => {
///         println!("The pin is now on!");
///     },
///     Err(msg) => {
///         println!("The pin could not be turn on: {}", msg);
///     }
/// };
/// ```
pub fn turn_on(pin: u8) -> Result<(), String> {
    let context: Context = python! {
        try:
            import RPi.GPIO as GPIO

            GPIO.setmode(GPIO.BCM)
            GPIO.setup('pin, GPIO.OUT)
            GPIO.output('pin, GPIO.HIGH)

            success = True
            msg = ""
        except Exception as e:
            msg = str(e)
            success = False
    };

    let success: bool = context.get::<bool>("success");
    let msg: String = context.get::<String>("msg");

    if success {
        Result::Ok(())
    } else {
        Result::Err(msg.to_string())
    }
}

/// A stand-alone function used to turn off a specified GPIO pin that is already on.
/// Once a pin is turned off, it will stop emitting any electrical current.
/// If you attempt to turn off a pin that is already off then nothing will happen.
/// 
/// This function takes an unsigned 8-bit integer as an argument. The argument is used to specify
/// which pin on the PI to turn off.
///
/// Returns a Result type which is either empty or contains a String datatype. The String may contain
/// a an error message to aid in debugging.
///
/// Example:
/// ```
/// match turn_off(1) {
///     Ok(()) => {
///         println!("The pin is now off!");
///     },
///     Err(msg) => {
///         println!("The pin could not be turn off: {}", msg);
///     }
/// };
/// ```
pub fn turn_off(pin: u8) -> Result<(), String> {
    let context: Context = python! {
        try:
            import RPi.GPIO as GPIO

            GPIO.setmode(GPIO.BCM)
            GPIO.setup('pin, GPIO.OUT)
            GPIO.output('pin, GPIO.LOW)

            success = True
            msg = ""
        except Exception as e:
            msg = str(e)
            success = False
    };

    let success: bool = context.get::<bool>("success");
    let msg: String = context.get::<String>("msg");

    if success {
        Result::Ok(())
    } else {
        Result::Err(msg.to_string())
    }
}

/// A stand-alone function used to cleanup the Raspberry PI. When a PI is cleaned up, all allocated resources will be freed up.
/// It is important to turn off and cleanup any pins that are not being currently used. This will help to avoid shorting out
/// you PI.
/// 
/// Returns a Result type which is either empty or contains a String datatype. The String may contain
/// a an error message to aid in debugging.
///
/// Example:
/// ```
/// match cleanup() {
///     Ok(()) => {
///         println!("The cleanup has been run!");
///     },
///     Err(msg) => {
///         println!("The cleanup could not be run: {}", msg);
///     }
/// };
/// ```
pub fn cleanup() -> Result<(), String> {
    let context: Context = python! {
        try:
            import RPi.GPIO as GPIO

            GPIO.cleanup()

            success = True
            msg = ""
        except Exception as e:
            msg = str(e)
            success = False
    };

    let success: bool = context.get::<bool>("success");
    let msg: String = context.get::<String>("msg");

    if success {
        Result::Ok(())
    } else {
        Result::Err(msg.to_string())
    }
}