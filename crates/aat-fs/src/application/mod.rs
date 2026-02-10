pub mod errortranslator;
pub mod filesizetool;

use crate::{domain::DomainService, error::TError};
use adk_rust::AdkError;
use schemars::_serde_json::{Value, from_value, to_value};
use serde::{Serialize, de::DeserializeOwned};

pub async fn handler<AP, DP, AR, DR, S, E>(
    service: S,
    args: Value,
    error_translator: fn(E) -> String,
) -> Result<Value, AdkError>
where
    E: TError,
    AP: TryInto<DP, Error = E> + DeserializeOwned,
    AR: From<DR> + Serialize,
    S: DomainService<Params = DP, Result = DR, Error = E>,
{
    let to_adkerror = |e| AdkError::Tool(error_translator(e));

    // read, validate and map args
    let args = from_value::<AP>(args).map_err(|e| to_adkerror(E::from_deserialize_error(e)))?;
    let params: DP = args.try_into().map_err(to_adkerror)?;

    // invoke service
    let res = service.invoke(params).await.map_err(to_adkerror)?;

    // map result
    to_value(AR::from(res)).map_err(|e| to_adkerror(E::from_serialize_error(e)))
}
