use serde_json::Value;

/// return value or none from serde_json::Value structures
/// Example:
/// ```rust
/// use opentelemetry_resourcedetector_gcp_rust::get_val::get_val;
/// let obj = serde_json::json!({
///     "foo": [
///         {
///             "bar": "bazz"
///         }
///     ]
/// });
/// let resp = get_val(&obj, &["foo", "0", "bar"], None);
/// assert_eq!(resp, Some(&serde_json::Value::String("bazz".to_string())));
/// ```
pub fn get_val<'a>(
    data: &'a Value,
    keys: &[&str],
    default: Option<&'a Value>,
) -> Option<&'a Value> {
    if keys.is_empty() {
        return Some(data);
    }
    let key = keys[0];
    if let Ok(key_int) = key.parse::<usize>() {
        match data.get(key_int) {
            Some(next_data) => get_val(next_data, &keys[1..], default),
            None => default,
        }
    } else {
        match data.get(key) {
            Some(next_data) => get_val(next_data, &keys[1..], default),
            None => default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_val_existing_key() {
        let json = serde_json::json!({
            "foo": [
                {
                    "bar": "bazz"
                }
            ]
        });
        let result = get_val(&json, &["foo", "0", "bar"], None);
        assert_eq!(result, Some(&serde_json::Value::String("bazz".to_string())));
    }

    #[test]
    fn test_get_val_existing_key_array() {
        let json = serde_json::json!({
          "value": {
            "user_id": "75bd116d-06f1-4ea8-95c2-712939c8b254",
            "content": {
              "args": {
                "text": "hi"
              },
              "type": "text"
            },
            "message_id": "da56df18-0192-40f4-b0c4-12cd05cff35d",
            "chat_id": "e942ca6d-cf39-4af9-b5e7-c3f9ef5b72ec"
          },
          "event_type": "message_posted_event"
        });
        let result = get_val(&json, &["value", "content", "args", "text"], None);
        assert_eq!(result, Some(&serde_json::Value::String("hi".to_string())));
        let result = get_val(&json, &["value", "chat_id"], None);
        assert_eq!(
            result,
            Some(&serde_json::Value::String(
                "e942ca6d-cf39-4af9-b5e7-c3f9ef5b72ec".to_string()
            ))
        );
    }
}
