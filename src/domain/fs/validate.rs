use std::io;

pub fn validate_entry_name(name: &str) -> Result<(), io::Error> {
    if name.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry name cannot be empty",
        ));
    }

    if name == "." || name == ".." {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Entry name '{}' is reserved", name),
        ));
    }

    if name.contains('\0') || name.contains('/') || name.contains('\\') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry name contains invalid characters (\\0, /, \\)",
        ));
    }

    if name.len() > 255 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry name exceeds maximum length (255 bytes)",
        ));
    }

    Ok(())
}
