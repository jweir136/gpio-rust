# Rust GPIO

![Travis CI Badge | Build](https://www.travis-ci.com/jweir136/gpio-rust.svg?branch=master)

Rust GPIO is a low-level library used to interact with a Raspberry PI's GPIO pins.

## Installation

In order to use Rust GPIO you not only need to install this crate to your project, but to
install the correct GPIO library on your Raspberry PI. Under the hood, Rust GPIO uses 
[RPi.GPIO](https://pypi.org/project/RPi.GPIO/). Luckily, the library comes pre-installed
in most Raspbian/Raspberry Pi OS operating systems. If you find yourself needing to download or
update the RPi.GPIO library you can run ```sudo apt-get update && sudo apt-get install python-rpi.gpio python3-rpi.gpi```. If you are unsure if the library is already installed, you install this crate and call the ```is_installed()``` method. The Rust GPIO crate can be installed to a project
by adding ```gpio-rust = "0.1.3"``` to your Cargo.toml file.

## Usage

Using this crate is very basic by design. You are given low-level GPIO access with the *turn_on* and *turn_off* functions located in the *gpio* module. The *cleanup* function is used to free up all allocated resources.

```
match turn_on(1) {
    Ok(()) => {
        println!("The pin is now on!");
    },
    Err(msg) => {
        println!("The pin could not be turn on: {}", msg);
    }
};

match turn_off(1) {
    Ok(()) => {
        println!("The pin is now off!");
    },
    Err(msg) => {
        println!("The pin could not be turn off: {}", msg);
    }
};

match cleanup() {
    Ok(()) => {
        println!("The cleanup has been run!");
    },
    Err(msg) => {
        println!("The cleanup could not be run: {}", msg);
    }
};
```

## Documentation

The Rust GPIO crate is very self-explanatory. However, if you find yourself in the need of some documentation, and there is a good change you will, you can access it at [Docs.rs | Rust GPIO](https://docs.rs/gpio-rust/0.1.3/gpio_rust/). You can also download the crate from source and build the docs using ```cargo doc --no-deps --open```.