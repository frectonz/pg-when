use nom::IResult;
use nom_language::error::VerboseError;

pub type NomResult<T, O> = IResult<T, O, VerboseError<T>>;
