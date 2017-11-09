//! Module containing the definition of error chain types.
#![allow(box_pointers)]
#![cfg_attr(feature = "cargo-clippy", allow(large_enum_variant))]

#![allow(unused_doc_comment)]
error_chain! {
    foreign_links {
        IO(::std::io::Error) #[doc = "I/O error."];
        Template(::handlebars::TemplateFileError) #[doc = "Template error."];
        TemplateRender(::handlebars::RenderError) #[doc = "Template rendering error."];
        JSON(::serde_json::error::Error) #[doc = "JSON error."];
        TOML(::toml::de::Error) #[doc = "TOML error."];
    }

    errors {
        /// Configuration error.
        Config(message: String) {
            description("there was an error in the configuration")
            display("there was an error in the configuration: {}", message)
        }
        /// Parsing error.
        Parse {
            description("there was an error in some parsing process")
        }
        /// Template name error.
        TemplateName(message: String) {
            description("Invalid template name")
            display("{}", message)
        }
        /// Code not found.
        CodeNotFound {
            description("the code was not found in the file")
        }
    }
}

impl Into<i32> for Error {
    fn into(self) -> i32 {
        let kind = self.kind();

        match *kind {
            ErrorKind::Parse |
            ErrorKind::TOML(_) => 20,
            ErrorKind::JSON(_) => 30,
            ErrorKind::CodeNotFound => 40,
            ErrorKind::Config(_) => 50,
            ErrorKind::IO(_) => 100,
            ErrorKind::TemplateName(_) => 125,
            ErrorKind::Template(_) => 150,
            ErrorKind::TemplateRender(_) => 175,
            ErrorKind::Msg(_) => 1,
            _ => -1,
        }
    }
}
