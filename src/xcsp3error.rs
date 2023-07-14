/**
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/12 11:56
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_core {

    // ----------------------------public for other-------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Xcsp3Error {
        ParseDomainError(ParseDomainError),
        ParseVariableError(ParseVariableError),
        ParseConstraintError(ParseConstraintError),
    }

    impl Xcsp3Error {
        pub fn get_constraint_extension_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseConstraintError(ParseConstraintError::get_extension_error(s))
        }
        pub fn get_domain_integer_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseDomainError(ParseDomainError::get_integer_error(s))
        }

        pub fn get_domain_interval_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseDomainError(ParseDomainError::get_interval_error(s))
        }

        pub fn get_domain_for_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseDomainError(ParseDomainError::get_domain_for_error(s))
        }

        pub fn get_variable_not_found_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseVariableError(ParseVariableError::get_not_found_error(s))
        }

        pub fn get_variable_size_invalid_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseVariableError(ParseVariableError::get_size_invalid_error(s))
        }

        pub fn to_string(&self) -> String {
            match self {
                Xcsp3Error::ParseDomainError(e) => e.to_string(),
                Xcsp3Error::ParseVariableError(e) => e.to_string(),
                Xcsp3Error::ParseConstraintError(e) => e.to_string(),
            }
        }
    }

    // ---------------------------------------------------------------------------------------------

    // ---------------------------all kind of error-------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DomainError {
        UnknownInteger,
        UnknownInterval,
        UnknownFor,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParseDomainError {
        pub msg: String,
        pub r#type: DomainError,
    }

    impl ParseDomainError {
        fn to_string(&self) -> String {
            format!("{:?}:{}", self.r#type, self.msg)
        }

        fn get_integer_error(s: &str) -> ParseDomainError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/variables/integer/";
            ParseDomainError {
                msg: (s.to_owned() + WEBSITE).to_string(),
                r#type: DomainError::UnknownInteger,
            }
        }
        fn get_interval_error(s: &str) -> ParseDomainError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/variables/arrays/";
            ParseDomainError {
                msg: (s.to_owned() + WEBSITE).to_string(),
                r#type: DomainError::UnknownInterval,
            }
        }
        fn get_domain_for_error(s: &str) -> ParseDomainError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/variables/arrays/";
            ParseDomainError {
                msg: (s.to_owned() + WEBSITE).to_string(),
                r#type: DomainError::UnknownFor,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParseVariableError {
        pub msg: String,
        pub r#type: VariableError,
    }

    impl ParseVariableError {
        fn to_string(&self) -> String {
            format!("{:?}:{}", self.r#type, self.msg)
        }

        fn get_not_found_error(s: &str) -> ParseVariableError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/variables/integer/";
            ParseVariableError {
                msg: (s.to_owned() + WEBSITE).to_string(),
                r#type: VariableError::NotFoundAsVariable,
            }
        }
        fn get_size_invalid_error(s: &str) -> ParseVariableError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/variables";
            ParseVariableError {
                msg: (s.to_owned() + WEBSITE).to_string(),
                r#type: VariableError::SizeInvalid,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum VariableError {
        NotFoundAsVariable,
        SizeInvalid,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParseConstraintError {
        pub msg: String,
        pub r#type: ConstraintError,
    }

    impl ParseConstraintError {
        fn to_string(&self) -> String {
            format!("{:?}:{}", self.r#type, self.msg)
        }
        fn get_extension_error(s: &str) -> ParseConstraintError {
            const WEBSITE: &str =
                " please visit http://xcsp.org/specifications/constraints/generic/extension/";
            ParseConstraintError {
                msg: (s.to_owned() + WEBSITE).to_string(),
                r#type: ConstraintError::ExtensionError,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ConstraintError {
        ExtensionError,
    }
}
