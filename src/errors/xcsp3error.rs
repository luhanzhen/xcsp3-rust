/*=============================================================================
* parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2023 xcsp.org (contact @ xcsp.org)
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*=============================================================================
*/

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
    use crate::errors::parse_constraint_error::xcsp3_core::ParseConstraintError;
    use crate::errors::parse_domain_error::xcsp3_core::ParseDomainError;
    use crate::errors::parse_variable_error::ParseVariableError;


    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Xcsp3Error {
        ParseDomainError(ParseDomainError),
        ParseVariableError(ParseVariableError),
        ParseConstraintError(ParseConstraintError),
    }

    /// error factory
    impl Xcsp3Error {
        pub fn get_constraint_extension_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseConstraintError(ParseConstraintError::get_extension_error(s))
        }

        pub fn get_constraint_scope_not_found_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseConstraintError(ParseConstraintError::get_scope_not_found_error(s))
        }

        pub fn get_constraint_list_of_values_error(s: &str) -> Xcsp3Error {
            Xcsp3Error::ParseConstraintError(ParseConstraintError::get_list_of_values_error(s))
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
}
