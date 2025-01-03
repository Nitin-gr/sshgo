use crate::certificate::{
    download_host_signing_key, download_user_signing_key, request_certificate,
};
use crate::file::{
    add_ca_key, check_ssh_keys, generate_keys_with_filename, load_connections, save_connection,
};
use crate::ssh::{
    connect_ssh, delete_ssh_key, generate_keys, list_ssh_keys, password_auth, rename_ssh_key,
    secure_copy,
};

#[tauri::command]
pub fn password_auth_command(username: &str) {
    password_auth(username);
}

#[tauri::command]
pub fn generate_keys_command(algorithm: &str, password: &str) {
    generate_keys(algorithm, password);
}

#[tauri::command]
pub fn secure_copy_command(username: &str) {
    secure_copy(username);
}

#[tauri::command]
pub fn connect_ssh_command(address: &str) {
    connect_ssh(address);
}

#[tauri::command]
pub fn save_connection_command(connection: String) -> Result<(), String> {
    save_connection(connection)
}

#[tauri::command]
pub fn load_connections_command() -> Result<Vec<String>, String> {
    load_connections()
}

#[tauri::command]
pub fn generate_keys_with_filename_command(
    algorithm: &str,
    password: &str,
    filename: &str,
    overwrite: bool,
) -> Result<i32, String> {
    generate_keys_with_filename(algorithm, password, filename, overwrite)
}

#[tauri::command]
pub fn check_ssh_keys_command() -> Result<Vec<String>, String> {
    check_ssh_keys()
}

#[tauri::command]
pub fn add_ca_key_command(
    file_content: String,
    filename: String,
    role: String,
) -> Result<i32, String> {
    add_ca_key(file_content, filename, role)
}

#[tauri::command]
pub fn list_ssh_keys_command() -> Result<Vec<String>, String> {
    match list_ssh_keys() {
        Ok(keys) => Ok(keys),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn delete_ssh_key_command(key_name: &str) -> Result<(), String> {
    delete_ssh_key(key_name)
}
#[tauri::command]
pub async fn generate_certificate_command(
    public_key: String,
    is_host: bool,
    email: String,
    provider: String,
) -> Result<String, String> {
    match request_certificate(public_key, is_host, email, provider).await {
        Ok(cert) => Ok(cert),
        Err(e) => {
            println!("Error generating certificate: {}", e);
            Err(format!("Error requesting certificate: {}", e))
        }
    }
}
#[tauri::command]
pub async fn download_user_signing_key_command() -> Result<String, String> {
    download_user_signing_key().await
}

#[tauri::command]
pub async fn download_host_signing_key_command() -> Result<String, String> {
    download_host_signing_key().await
}

#[tauri::command]
pub fn rename_ssh_key_command(old_name: &str, new_name: &str) -> Result<(), String> {
    rename_ssh_key(old_name, new_name)
}
