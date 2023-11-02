use aintnobody::{Error, playintnobody};


fn main() -> Result<(), Error> {
    playintnobody(std::time::Duration::from_millis(1284))?;
    Ok(())
}
