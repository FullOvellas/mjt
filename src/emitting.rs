use mjp::{Json, Pair, Value};

pub fn emit(json: Json, indentation: u8) -> String {
    emit_value(json.value, indentation, 0)
}

fn emit_value(value: Value, indentation: u8, nesting: u8) -> String {
    use Value::*;
    match value {
        Object(pairs) => emit_object(pairs, indentation, nesting),
        Array(values) => emit_array(values, indentation, nesting),
        Str(s) => format!("\"{s}\""),
        Number(n) => n,
        Boolean(b) => b.to_string(),
        Null => "null".to_string(),
    }
}

fn emit_object(pairs: Vec<Pair>, indentation: u8, nesting: u8) -> String {
    let indent = " ".repeat((indentation * nesting) as usize);
    format!(
        "{{{}\n{}}}",
        emit_pairs(pairs, indentation, nesting + 1),
        indent
    )
}

fn emit_array(values: Vec<Value>, indentation: u8, nesting: u8) -> String {
    let indent = " ".repeat((indentation * nesting) as usize);
    format!(
        "[\n{}\n{}]",
        emit_values(values, indentation, nesting + 1),
        indent
    )
}

fn emit_values(values: Vec<Value>, indentation: u8, nesting: u8) -> String {
    let indent = " ".repeat((indentation * nesting) as usize);
    values
        .into_iter()
        .map(|v| format!("{}{}", indent, emit_value(v, indentation, nesting)))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn emit_pairs(pairs: Vec<Pair>, indentation: u8, nesting: u8) -> String {
    let indent = " ".repeat((indentation * nesting) as usize);
    pairs
        .into_iter()
        .map(|pair| {
            format!(
                "\n{}\"{}\": {}",
                indent,
                pair.key,
                emit_value(pair.value, indentation, nesting)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}
