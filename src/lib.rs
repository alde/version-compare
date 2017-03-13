pub mod comp_op;
pub mod version;
pub mod version_part;
pub mod version_manifest;
#[cfg(test)]
mod test;

use comp_op::CompOp;
use version::Version;

/// Version compare structure.
pub struct VersionCompare { }

/// Version compare implementation.
impl VersionCompare {

    /// Compare two version number strings to each other.
    /// This compares version `a` to version `b`, and returns whether version `a` is greater, less
    /// or equal to version `b`.
    ///
    /// The two given version numbers must be valid, or an error will be returned.
    ///
    /// One of the following ok results may be returned:
    /// - CompOp::Eq
    /// - CompOp::Lt
    /// - CompOp::Gt
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::VersionCompare;
    /// use version_compare::comp_op::CompOp;
    ///
    /// // Compare version numbers
    /// assert_eq!(VersionCompare::compare("1.2.3", "1.2.3"), Ok(CompOp::Eq));
    /// assert_eq!(VersionCompare::compare("1.2.3", "1.2.4"), Ok(CompOp::Lt));
    /// assert_eq!(VersionCompare::compare("1", "0.1"), Ok(CompOp::Gt));
    /// ```
    pub fn compare(a: &str, b: &str) -> Result<CompOp, ()> {
        // Create version instances
        let a_ver = Version::from(a);
        let b_ver = Version::from(b);

        // Both version numbers must have been parsed
        if a_ver.is_none() || b_ver.is_none() {
            return Err(());
        }

        // Compare and return the result
        Ok(a_ver.unwrap().compare(&b_ver.unwrap()))
    }

    /// Compare two version number strings to each other and check whether the given comparison
    /// `operator` is valid.
    ///
    /// The two given version numbers must be valid, or an error will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::VersionCompare;
    /// use version_compare::comp_op::CompOp;
    ///
    /// // Compare version numbers
    /// assert!(VersionCompare::compare_to("1.2.3", "1.2.3", &CompOp::Eq).unwrap());
    /// assert!(VersionCompare::compare_to("1.2.3", "1.2.3", &CompOp::Le).unwrap());
    /// assert!(VersionCompare::compare_to("1.2.3", "1.2.4", &CompOp::Lt).unwrap());
    /// assert!(VersionCompare::compare_to("1", "0.1", &CompOp::Gt).unwrap());
    /// assert!(VersionCompare::compare_to("1", "0.1", &CompOp::Ge).unwrap());
    /// ```
    pub fn compare_to(a: &str, b: &str, operator: &CompOp) -> Result<bool, ()> {
        // Create version instances
        let a_ver = Version::from(a);
        let b_ver = Version::from(b);

        // Both version numbers must have been parsed
        if a_ver.is_none() || b_ver.is_none() {
            return Err(());
        }

        // Compare and return the result
        Ok(a_ver.unwrap().compare_to(&b_ver.unwrap(), &operator))
    }
}

#[cfg(test)]
mod tests {
    use comp_op::CompOp;
    use test::test_version_set::{TEST_VERSION_SETS, TEST_VERSION_SETS_ERROR};
    use super::VersionCompare;

    #[test]
    fn compare() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            assert_eq!(
                VersionCompare::compare(&entry.0, &entry.1),
                Ok(entry.2.clone())
            );
        }

        // Compare each error version in the version set
        for entry in TEST_VERSION_SETS_ERROR {
            let result = VersionCompare::compare(&entry.0, &entry.1);

            if result.is_ok() {
                assert!(result != Ok(entry.2.clone()));
            }
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Test
            assert!(VersionCompare::compare_to(&entry.0, &entry.1, &entry.2).unwrap());

            // Make sure the inverse operator is not correct
            assert_eq!(VersionCompare::compare_to(&entry.0, &entry.1, &entry.2.invert()).unwrap(), false);
        }

        // Compare each error version in the version set
        for entry in TEST_VERSION_SETS_ERROR {
            let result = VersionCompare::compare_to(&entry.0, &entry.1, &entry.2);

            if result.is_ok() {
                assert!(!result.unwrap())
            }
        }

        // Assert an exceptional case, compare to not equal
        assert!(VersionCompare::compare_to("1.2.3", "1.2", &CompOp::Ne).unwrap());
    }
}
