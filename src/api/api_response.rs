use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn error(message: String) -> String {
        let response = ApiResponse {
            status: "error".to_string(),
            message: message,
            data: Vec::<T>::new(),
        };

        return serde_json::to_string_pretty(&response).unwrap();
    }

    pub fn success(data: T) -> String {
        let response = ApiResponse {
            status: "success".to_string(),
            message: "".to_string(),
            data: data,
        };

        return serde_json::to_string_pretty(&response).unwrap();
    }

    // pub fn success_with_message(data: T, message: String) -> String {
    //     let response = ApiResponse {
    //         status: "success".to_string(),
    //         message: message,
    //         data: data,
    //     };

    //     return serde_json::to_string_pretty(&response).unwrap();
    // }
}
