use inline_python::{ Context, python };

pub enum Power {
    High,
    Low
}

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