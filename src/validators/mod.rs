pub struct Validator {
    pub empty: String,
    pub invalid: String,
    pub min_length: Option<usize>,
}

impl Validator {
    /// Initialize validator builder
    pub fn builder() -> Self {
        Self {
            empty: String::default(),
            invalid: String::default(),
            min_length: None,
        }
    }

    /// Set empty error message
    pub fn set_empty<T>(&mut self, message: T) -> &mut Self
        where T: ToString
    {
        self.empty = message.to_string();

        self
    }

    /// Set invalid error message
    pub fn set_invalid<T>(&mut self, message: T) -> &mut Self
        where T: ToString
    {
        self.invalid = message.to_string();

        self
    }

    /// Set min length
    pub fn set_min_length(&mut self, size: usize) -> &mut Self {
        self.min_length = Some(size);

        self
    }

    /// Validate Option<String> value
    pub fn validate_string<T>(&self, value: T) -> String
        where T: ToString
    {
        // Generate empty error message
        let empty = match self.empty.is_empty() {
            true => match self.invalid.is_empty() {
                true => String::from("Field is empty"),
                false => self.invalid.clone(),
            },
            false => self.empty.clone()
        };

        // Generate invalid error message
        let invalid = match self.invalid.is_empty() {
            true => match self.empty.is_empty() {
                true => String::from("Field is invalid"),
                false => self.empty.clone(),
            },
            false => self.invalid.clone()
        };

        // Set value bindings
        let value = value.to_string();

        // Check if value is empty
        if value.is_empty() {
            return empty;
        }

        // Check if min length is set
        if let Some(size) = self.min_length {
            if size > value.len() {
                return invalid;
            }
        }

        // Return default string if no error was found
        String::default()
    }

    /// Validate Option<String> value
    pub fn validate_string_option<T>(&self, value: Option<T>) -> Option<String>
        where T: ToString
    {
        let value = match value {
            Some(value) => value.to_string(),
            None => String::default()
        };

        let validated = self.validate_string(value);
        match validated.is_empty() {
            true => None,
            false => Some(validated)
        }
    }
}