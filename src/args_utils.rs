use std::{collections::HashMap, env};

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserArgs {
    pub app_path: String,
    pub command: Option<String>,
    pub params: Option<HashMap<String, String>>
}

impl std::fmt::Display for UserArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "\tapp_path: {},\n", self.app_path)?;
        write!(f, "\tcommand: {},\n", self.command.clone().unwrap_or_default())?;
        write!(f, "\tparams: <not displayed>\n")?;
        write!(f, "}}")?;
        Ok(())
    }
}

pub fn _get_user_arguments() -> UserArgs {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let app_path  = args[0].clone();
    let command: Option<String> = args.get(1).cloned();
    
    let mut params = None;
    let params_list  = &args.get(2..);
    if let Some(params_list) = params_list {
        let mut params_map = HashMap::new();

        params_list
            .chunks(2)
            .for_each(|pair| {
                params_map.insert(pair[0].clone(), pair[1].clone());
            });

        params = Some(params_map);
    }

    UserArgs { 
        app_path,
        command,
        params
    }
}
