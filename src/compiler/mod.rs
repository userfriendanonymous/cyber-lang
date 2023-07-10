use std::{collections::HashMap, ops::Add};

impl Function {
    pub fn compile(&self, env: &Module) {
        match self.expression {
            Expression::Call { path, input } => {
                env.function(path)
            }
        }
    }
}
use crate::parser::{Ident, Type};


impl<'a> Module<'a> {
    pub fn path(&self, names: &[EnvIdent]) -> Option<&Module<'_>> {
        let mut env = self;
        for name in names {
            if let Some(next_env) = match name {
                EnvIdent::Sub(name) =>  self.modules.get(name),
                EnvIdent::Super => env.sup
            } {
                env = next_env;
            } else {
                return None
            }
        }

        Some(env)
    }

    pub fn function(&self, path: &[EnvIdent], name: &Ident) -> Option<&Function> {
        self.path(path)?.functions.get(name)
    }

    pub fn type_(&self, path: &[EnvIdent], name: &Ident) -> Option<&Type> {
        self.path(path)?.types.get(name)
    }
}

impl Module {
    pub fn type_(&self, path: Path) -> Option<&Type> {
        self.module_of_path_base(&normalize_path_base(path.base))?
        .types.get(&path.name)
    }

    pub fn function(&self, path: Path) -> Option<&Function> {
        self.module_of_path_base(&normalize_path_base(path.base))?
        .functions.get(&path.name)
    }

    pub fn module(&self, path: Path) -> Option<&Module> {
        self.module_of_path_base(&normalize_path_base(path.base))?
        .modules.get(&path.name)
    }

    pub fn module_of_path_base(&self, value: &[Ident]) -> Option<&Module> {
        let mut current = self;
        for part in value {
            if let Some(module) = current.modules.get(part) {
                current = module;
            } else {
                return None
            }
        }
        Some(current)
    }

    pub fn compile_function(&self, value: Function) {
        match value.expression {
            Expression::Call { path, input } => {
                self.module(path.base)?.
                self.function(path)
            }
        }
    }


    pub fn compile(self) {
        for (name, value) in self.functions {
            if value.is_external {
                value.expression
            }
        }
    }
}
