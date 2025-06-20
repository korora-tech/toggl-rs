//! Base HTTP client functionality shared across toggl crates

use reqwest::{
    blocking::{Client, Response},
    Method,
};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;

use crate::{Error, Result};

/// Base client for Toggl API requests
#[derive(Clone)]
pub struct BaseClient {
    api_token: String,
    client: Client,
}

impl BaseClient {
    /// Create a new base client with the given API token
    pub fn new(api_token: String) -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(|e| Error::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { api_token, client })
    }

    /// Get the authorization header value
    pub fn auth_header(&self) -> String {
        use base64::Engine;
        let credentials = format!("{}:api_token", self.api_token);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
        format!("Basic {}", encoded)
    }

    /// Make a request without parameters
    pub fn request<T: DeserializeOwned>(&self, method: Method, url: &str) -> Result<T> {
        self.request_with_params(method, url, &BTreeMap::new())
    }

    /// Make a request with query parameters
    pub fn request_with_params<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        params: &BTreeMap<String, String>,
    ) -> Result<T> {
        let response = self.send_request(method, url, params, None::<&()>)?;
        self.handle_response(response)
    }

    /// Make a request with a JSON body
    pub fn request_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        url: &str,
        body: &B,
    ) -> Result<T> {
        let response = self.send_request(method, url, &BTreeMap::new(), Some(body))?;
        self.handle_response(response)
    }

    /// Make a request expecting no response body
    pub fn request_empty(&self, method: Method, url: &str) -> Result<()> {
        let response = self.send_request(method, url, &BTreeMap::new(), None::<&()>)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    /// Make a request with a JSON body expecting no response
    pub fn request_with_body_empty<B: Serialize>(
        &self,
        method: Method,
        url: &str,
        body: &B,
    ) -> Result<()> {
        let response = self.send_request(method, url, &BTreeMap::new(), Some(body))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    /// Make a request expecting binary data
    #[allow(unused_mut)]
    pub fn request_binary(&self, method: Method, url: &str) -> Result<Vec<u8>> {
        let mut response = self.send_request(method, url, &BTreeMap::new(), None::<&()>)?;
        let status = response.status();

        if status.is_success() {
            response.bytes().map(|b| b.to_vec()).map_err(|e| {
                Error::ResponseParseError(format!("Failed to read response bytes: {}", e))
            })
        } else {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    /// Make a request with a JSON body expecting binary data
    pub fn request_bytes_with_body<B: Serialize>(
        &self,
        method: Method,
        url: &str,
        body: &B,
    ) -> Result<Vec<u8>> {
        let response = self.send_request(method, url, &BTreeMap::new(), Some(body))?;

        if response.status().is_success() {
            response
                .bytes()
                .map(|b| b.to_vec())
                .map_err(|e| Error::NetworkError(e.to_string()))
        } else {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| status.to_string());
            Err(Error::api_error(status.as_u16(), error_text, None))
        }
    }

    /// Make a multipart request with file upload
    pub fn request_multipart(
        &self,
        method: Method,
        url: &str,
        file_data: &[u8],
        file_name: &str,
    ) -> Result<()> {
        self.request_multipart_with_mime(method, url, file_data, file_name, "text/csv")
    }

    /// Make a multipart request with file upload and custom MIME type
    pub fn request_multipart_with_mime(
        &self,
        method: Method,
        url: &str,
        file_data: &[u8],
        file_name: &str,
        mime_type: &str,
    ) -> Result<()> {
        #[cfg(test)]
        println!("Sending multipart request to: {} {}", method, url);

        let part = reqwest::blocking::multipart::Part::bytes(file_data.to_vec())
            .file_name(file_name.to_string())
            .mime_str(mime_type)
            .map_err(|e| Error::RequestBuildError(format!("Failed to create multipart: {}", e)))?;

        let form = reqwest::blocking::multipart::Form::new().part("file", part);

        let response = self
            .client
            .request(method, url)
            .header("Authorization", self.auth_header())
            .multipart(form)
            .send()
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    /// Make a multipart request with JSON data and file
    #[allow(clippy::too_many_arguments)]
    pub fn request_multipart_json_file<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        url: &str,
        json_data: &B,
        file_data: &[u8],
        file_name: &str,
        file_field_name: &str,
        mime_type: &str,
    ) -> Result<T> {
        #[cfg(test)]
        println!("Sending multipart JSON+file request to: {} {}", method, url);

        let json_string = serde_json::to_string(json_data)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize JSON: {}", e)))?;

        let json_part = reqwest::blocking::multipart::Part::text(json_string)
            .mime_str("application/json")
            .map_err(|e| Error::RequestBuildError(format!("Failed to create JSON part: {}", e)))?;

        let file_part = reqwest::blocking::multipart::Part::bytes(file_data.to_vec())
            .file_name(file_name.to_string())
            .mime_str(mime_type)
            .map_err(|e| Error::RequestBuildError(format!("Failed to create file part: {}", e)))?;

        let form = reqwest::blocking::multipart::Form::new()
            .part("data", json_part)
            .part(file_field_name.to_string(), file_part);

        let response = self
            .client
            .request(method, url)
            .header("Authorization", self.auth_header())
            .multipart(form)
            .send()
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        if response.status().is_success() {
            response.json::<T>().map_err(|e| {
                Error::ResponseParseError(format!("Failed to deserialize response: {}", e))
            })
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    /// Internal method to send a request
    fn send_request<B: Serialize>(
        &self,
        method: Method,
        url: &str,
        params: &BTreeMap<String, String>,
        body: Option<&B>,
    ) -> Result<Response> {
        #[cfg(test)]
        println!("Sending request to: {} {}", method, url);

        let mut request = self
            .client
            .request(method, url)
            .header("Authorization", self.auth_header());

        // Add content type for JSON requests
        if body.is_some() {
            request = request.header("Content-Type", "application/json");
        }

        // Add query parameters
        for (key, value) in params {
            request = request.query(&[(key, value)]);
        }

        // Add body if provided
        if let Some(body) = body {
            request = request.json(body);
        }

        request
            .send()
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))
    }

    /// Internal method to handle response
    #[allow(unused_mut)]
    fn handle_response<T: DeserializeOwned>(&self, mut response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().map_err(|e| {
                Error::ResponseParseError(format!("Failed to deserialize JSON: {}", e))
            })
        } else {
            let status_code = status.as_u16();

            // Extract retry-after header for rate limiting
            let retry_after = if status_code == 429 {
                response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(60)
            } else {
                0
            };

            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse as JSON to get structured error details
            let details = serde_json::from_str::<serde_json::Value>(&error_text).ok();
            let message = if let Some(ref json) = details {
                json.get("message")
                    .or_else(|| json.get("error"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&error_text)
                    .to_string()
            } else {
                error_text
            };

            // Map specific status codes to more specific error types
            match status_code {
                401 => Err(Error::AuthError(message)),
                403 => Err(Error::PermissionError(message)),
                404 => Err(Error::ApiError {
                    status: status_code,
                    message,
                    details,
                }),
                429 => Err(Error::RateLimitError { retry_after }),
                422 => Err(Error::ValidationError(message)),
                _ => Err(Error::ApiError {
                    status: status_code,
                    message,
                    details,
                }),
            }
        }
    }

    /// Convenience method for GET requests
    pub fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &BTreeMap<String, String>,
    ) -> Result<T> {
        self.request_with_params(Method::GET, url, params)
    }

    /// Convenience method for POST requests
    pub fn post<T: DeserializeOwned, B: Serialize>(&self, url: &str, body: &B) -> Result<T> {
        self.request_with_body(Method::POST, url, body)
    }

    /// Convenience method for PUT requests
    pub fn put<T: DeserializeOwned, B: Serialize>(&self, url: &str, body: &B) -> Result<T> {
        self.request_with_body(Method::PUT, url, body)
    }

    /// Convenience method for PATCH requests
    #[allow(dead_code)]
    pub fn patch<T: DeserializeOwned, B: Serialize>(&self, url: &str, body: &B) -> Result<T> {
        self.request_with_body(Method::PATCH, url, body)
    }

    /// Convenience method for DELETE requests
    #[allow(dead_code)]
    pub fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.request(Method::DELETE, url)
    }

    /// Convenience method for DELETE requests with no response
    pub fn delete_empty(&self, url: &str) -> Result<()> {
        self.request_empty(Method::DELETE, url)
    }
}
