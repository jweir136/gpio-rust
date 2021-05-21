use inline_python::{ python, Context };

pub fn is_installed() -> bool {
    let context: Context = python! {
        try:
            import RPi.GPIO
            was_installed = True
        except Exception as e:
            was_installed = False
            print(e)
    };

    context.get::<bool>("was_installed")
}

#[cfg(test)]
mod tests {
    use crate::is_installed;

    #[test]
    fn is_installed_nonrpi_test() {
        assert_eq!(is_installed(), false);
    }
}
