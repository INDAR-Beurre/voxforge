use crate::database::dictionary::DictionaryEntry;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_dictionary(
    state: State<'_, AppState>,
) -> Result<Vec<DictionaryEntry>, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_dictionary_entries().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_dictionary_entry(
    state: State<'_, AppState>,
    spoken_phrase: String,
    replacement: String,
    category: Option<String>,
) -> Result<DictionaryEntry, String> {
    let entry = DictionaryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        spoken_phrase,
        replacement,
        category,
        enabled: true,
        use_count: 0,
    };

    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.insert_dictionary_entry(&entry).map_err(|e| e.to_string())?;
    Ok(entry)
}

#[tauri::command]
pub async fn delete_dictionary_entry(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.delete_dictionary_entry(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seed_developer_dictionary(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let entries = vec![
        ("javascript", "JavaScript"),
        ("typescript", "TypeScript"),
        ("react", "React"),
        ("next js", "Next.js"),
        ("node js", "Node.js"),
        ("vue js", "Vue.js"),
        ("angular", "Angular"),
        ("svelte", "Svelte"),
        ("rust", "Rust"),
        ("python", "Python"),
        ("go lang", "Go"),
        ("kubernetes", "Kubernetes"),
        ("docker", "Docker"),
        ("git hub", "GitHub"),
        ("git lab", "GitLab"),
        ("vs code", "VS Code"),
        ("api", "API"),
        ("rest api", "REST API"),
        ("graph ql", "GraphQL"),
        ("sql", "SQL"),
        ("no sql", "NoSQL"),
        ("postgres", "PostgreSQL"),
        ("mongo db", "MongoDB"),
        ("redis", "Redis"),
        ("aws", "AWS"),
        ("gcp", "GCP"),
        ("azure", "Azure"),
        ("terraform", "Terraform"),
        ("c i c d", "CI/CD"),
        ("dev ops", "DevOps"),
        ("html", "HTML"),
        ("css", "CSS"),
        ("json", "JSON"),
        ("yaml", "YAML"),
        ("toml", "TOML"),
        ("web socket", "WebSocket"),
        ("o auth", "OAuth"),
        ("jwt", "JWT"),
        ("http", "HTTP"),
        ("https", "HTTPS"),
        ("url", "URL"),
        ("cli", "CLI"),
        ("gui", "GUI"),
        ("i d e", "IDE"),
        ("npm", "npm"),
        ("yarn", "yarn"),
        ("p n p m", "pnpm"),
        ("webpack", "webpack"),
        ("vite", "Vite"),
        ("tailwind", "Tailwind CSS"),
        ("camel case", "camelCase"),
        ("snake case", "snake_case"),
        ("kebab case", "kebab-case"),
    ];

    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;

    for (spoken, replacement) in entries {
        let entry = DictionaryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            spoken_phrase: spoken.to_string(),
            replacement: replacement.to_string(),
            category: Some("developer".to_string()),
            enabled: true,
            use_count: 0,
        };
        let _ = db.insert_dictionary_entry(&entry);
    }

    Ok(())
}
