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
    }

    impl Xcsp3Error {
        pub fn get_domain_integer_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseDomainError(ParseDomainError::get_integer_error(s))
        }

        pub fn get_domain_interval_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseDomainError(ParseDomainError::get_interval_error(s))
        }

        pub fn to_string(&self) -> String {
            match self {
                Xcsp3Error::ParseDomainError(e) => e.to_string(),
                Xcsp3Error::ParseVariableError(e) => e.to_string(),
            }
        }
    }

    // ---------------------------------------------------------------------------------------------

    // ---------------------------all kind of error-------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DomainError {
        UnknownInteger,
        UnknownInterval,
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
            ParseDomainError {
                msg: s.to_string(),
                r#type: DomainError::UnknownInteger,
            }
        }
        fn get_interval_error(s: &str) -> ParseDomainError {
            ParseDomainError {
                msg: s.to_string(),
                r#type: DomainError::UnknownInterval,
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
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum VariableError {}
}
