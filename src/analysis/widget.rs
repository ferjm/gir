use env::Env;
use gobjects::GObject;
use library;
use super::object;

pub fn new(env: &Env, obj: &GObject) -> object::Info {
    let info = object::new(env, obj);

    let has_functions = info.functions.iter().find(|f| f.kind == library::FunctionKind::Function).is_some();
    assert!(!has_functions, "Widget {} has functions, to do functions code generation", info.full_name);

    info
}
