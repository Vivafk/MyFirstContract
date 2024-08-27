use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    UploadDocument { document: Binary },
    SignDocument { document_id: u64, signature: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetSignatures { document_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Document {
    id: u64,
    data: Binary,
    signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignaturesResponse {
    signatures: Vec<String>,
}

static mut DOCUMENTS: Vec<Document> = Vec::new();
static mut DOCUMENT_ID_COUNTER: u64 = 0;

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UploadDocument { document } => upload_document(document),
        ExecuteMsg::SignDocument { document_id, signature } => sign_document(document_id, signature),
    }
}

fn upload_document(document: Binary) -> StdResult<Response> {
    unsafe {
        DOCUMENT_ID_COUNTER += 1;
        let new_document = Document {
            id: DOCUMENT_ID_COUNTER,
            data: document,
            signatures: Vec::new(),
        };
        DOCUMENTS.push(new_document);
    }
    Ok(Response::default())
}

fn sign_document(document_id: u64, signature: String) -> StdResult<Response> {
    unsafe {
        if let Some(doc) = DOCUMENTS.iter_mut().find(|d| d.id == document_id) {
            doc.signatures.push(signature);
        }
    }
    Ok(Response::default())
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSignatures { document_id } => to_binary(&query_signatures(document_id)),
    }
}

fn query_signatures(document_id: u64) -> SignaturesResponse {
    unsafe {
        if let Some(doc) = DOCUMENTS.iter().find(|d| d.id == document_id) {
            return SignaturesResponse {
                signatures: doc.signatures.clone(),
            };
        }
    }
    SignaturesResponse {
        signatures: Vec::new(),
    }
}
