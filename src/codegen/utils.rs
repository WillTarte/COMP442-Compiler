use crate::semantics::symbol_table::{SymbolTable, Type, Scope};

pub fn sizeof(t: &Type, symbols: &SymbolTable) -> usize
{
    let size: usize = match t
    {
        Type::Integer => { 4 }
        Type::IntegerArray(dim) => {
            4usize * dim.iter().product::<usize>()
        }
        Type::Float => { 8 }
        Type::FloatArray(dim) => {
            8usize * dim.iter().product::<usize>()
        }
        Type::String => { todo!() }
        Type::StringArray(dim) => {
            0usize * dim.iter().product::<usize>()
        }
        Type::Custom(ident) => {
            let mut temp_size: usize = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident)
            {
                for scope in ce.table().scopes()
                {
                    if let Scope::Variable(ve) = scope
                    {
                        temp_size += sizeof(ve.var_type(), symbols);
                    }
                    else if let Scope::Function(fe) = scope
                    {
                        todo!()
                    }
                }
            }
            else { panic!() }
            temp_size

        }
        Type::CustomArray(ident, dim) => {
            let mut temp_size: usize = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident)
            {
                for scope in ce.table().scopes()
                {
                    if let Scope::Variable(ve) = scope
                    {
                        temp_size += sizeof(ve.var_type(), symbols);
                    }
                    else if let Scope::Function(fe) = scope
                    {
                        todo!()
                    }
                }
            }
            else { panic!() }
            temp_size * dim.iter().product::<usize>()
        }
        Type::Void => { 0 }
    };
    size
}