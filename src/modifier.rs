use types::Type;
use user::UserPath;
use interpreter::Interpreter;
use environment::Environment;
use regex;

pub type ModifierFunc = Fn(&Type, &mut Interpreter, &UserPath,
                           &mut Environment, &[Type]) -> Option<Type>;

pub fn apply_default_mods(inter: &mut Interpreter) {
    inter.modifiers.insert("chars".to_string(), Box::new(default_mod_chars));
    inter.modifiers.insert("merge".to_string(), Box::new(default_mod_merge));
    inter.modifiers.insert("filter".to_string(), Box::new(default_mod_filter));
}

fn default_mod_chars(value: &Type, inter: &mut Interpreter, from: &UserPath,
                     env: &mut Environment, args: &[Type]) -> Option<Type> {
    if args.len() != 0 {
        None
    } else {
        Some(Type::Tuple(value.get_string(inter, from, env).unwrap().chars()
            .map(|v|Type::Text(v.to_string())).collect()))
    }
}

fn default_mod_merge(value: &Type, inter: &mut Interpreter, from: &UserPath,
                     env: &mut Environment, args: &[Type]) -> Option<Type> {
    if args.len() != 0 {
        None
    } else {
        Some(Type::Text(
            value.unpack(inter, from, env).iter()
                 .map(|v|v.get_string(inter, from, env).unwrap())
                 .collect::<String>()
        ))
    }
}

fn default_mod_filter(value: &Type, inter: &mut Interpreter, from: &UserPath,
                      env: &mut Environment, args: &[Type]) -> Option<Type> {
    if args.len() != 1 {
        None
    } else {
        let r = regex::Regex::new(&args[0].get_string(inter, from, env).unwrap()).unwrap();
        Some(Type::Tuple(
            value.unpack(inter, from, env).iter()
                 .map(|v|v.get_string(inter, from, env).unwrap())
                 .filter(|v|r.is_match(&v))
                 .map(|v|Type::Text(v))
                 .collect()
        ))
    }
}
